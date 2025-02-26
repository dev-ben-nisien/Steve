import os, sys
from typing import List, Dict, Any
from pathlib import Path

from langchain_community.document_loaders import DirectoryLoader
from langchain.text_splitter import RecursiveCharacterTextSplitter
from langchain_core.output_parsers import StrOutputParser
from langchain_core.runnables.passthrough import RunnablePassthrough
from langchain_openai import OpenAIEmbeddings
from langchain_openai import ChatOpenAI
from langchain.prompts import PromptTemplate
from langchain_community.vectorstores import DocArrayInMemorySearch
from pydantic import ValidationError
from dotenv import load_dotenv

# Import callback base class
from langchain.callbacks.base import BaseCallbackHandler

# Load environment variables from .env file
load_dotenv()


class StreamingCallbackHandler(BaseCallbackHandler):
    """Callback handler that streams LLM tokens to console."""
    
    def on_llm_new_token(self, token: str, **kwargs: Any) -> None:
        print(token, end="", flush=True)


class MarkdownRAG:
    def __init__(
        self,
        docs_dir: str,
        chunk_size: int = 1000,
        chunk_overlap: int = 200,
        openai_api_key: str = None,
    ):
        self.docs_dir = docs_dir
        self.chunk_size = chunk_size
        self.chunk_overlap = chunk_overlap
        
        # Set API key
        self.api_key = openai_api_key or os.getenv("OPENAI_API_KEY")
        if not self.api_key:
            raise ValueError("OpenAI API key is required")
        
        # Initialize embedding model
        self.embeddings = OpenAIEmbeddings(
            openai_api_key=self.api_key,
            model="text-embedding-3-small"
        )
        
        # Initialize vector store
        self.vector_store = None
        
        # Initialize LLM with streaming enabled and pass the custom callback.
        self.llm = ChatOpenAI(
            model_name="o1-mini",
            temperature=1,
            openai_api_key=self.api_key,
            streaming=True,
            callbacks=[StreamingCallbackHandler()]
        )

    def load_documents(self) -> List[Dict]:
        """Load markdown documents from the specified directory."""
        loader = DirectoryLoader(
            self.docs_dir,
            glob="**/*.md",
            show_progress=True,
        )
        documents = loader.load()
        
        # Split documents into chunks
        text_splitter = RecursiveCharacterTextSplitter(
            chunk_size=self.chunk_size,
            chunk_overlap=self.chunk_overlap,
            separators=["\n\n", "\n", " ", ""],
        )
        
        return text_splitter.split_documents(documents)

    def create_index(self) -> None:
        """Create the in-memory vector store."""
        documents = self.load_documents()
        
        # Create vector store
        self.vector_store = DocArrayInMemorySearch.from_documents(
            documents,
            self.embeddings
        )
        
    def describe(
        self, 
        prompt: str, 
        system_prompt: str = None, 
        temperature: float = None
    ) -> str:
        """Extract architectural decisions based on a prompt and git diff."""
        # Create a new LLM instance with streaming enabled and callback.
        llm = ChatOpenAI(
            model_name="gpt-4o-mini",
            temperature=0.1,
            openai_api_key=self.api_key,
            streaming=False,
            callbacks=[StreamingCallbackHandler()]
        )

        if system_prompt is None:
            system_prompt = (
                "We document architectural decisions using ADRS. Your job is to "
                "extract architectural decisions from the result of a git diff. "
                "List accurately the areas in which the engineer has made a decision "
                "that may have trade offs. Keep the response very consise. Suggest "
                "semantically similar topics to search for in documentation"
            )

        prompt_template = PromptTemplate(
            template="""
            {system_prompt}

            User: {prompt}
            
            Assistant:""",
            input_variables=["system_prompt", "prompt"]
        )

        # Setup the chain with the prompt template, LLM, and output parser.
        chain = prompt_template | llm | StrOutputParser()
        response = chain.invoke(
            {"system_prompt": system_prompt, "prompt": prompt}
        )
        
        return response.strip()

    def query(self, question: str, k: int = 8) -> Dict:
        if not self.vector_store:
            self.create_index()

        prompt_template = PromptTemplate(
            template="""
            You have been provided the description of a change, and the git diff from a PR an engineer
            has requested to be merged. Do we have existing documentation for this? We write ADRs for technical 
            decisions, is the change significant enough to warrant this?
            If its worth creating an ADR suggest some positive & negative consequences of the decision and trade-offs.

            Change: {question}

            Context: {context}
            
            Important!: Keep it consise - so its a very quick read.
            Important!: Consider the time efficiency when suggesting to write a ADR. Only suggest writing
            and ADR if the change is significant and spending time documenting is worth it.
            Important!: If according to the git diff documentation has been added, do not suggest creating a new one,
            review the added one(s) to identify gaps in logic.

            Answer:""",
            input_variables=["context", "question"],
        )

        retriever = self.vector_store.as_retriever(
            search_type="mmr",
            search_kwargs={'k': 4, 'lambda_mult': 0.50}
        )
        
        # The chain now uses streaming llm from self.llm which already supports streaming.
        chain = (
            {
                "context": retriever,
                "question": RunnablePassthrough()
            }
            | prompt_template
            | self.llm
            | StrOutputParser()
        )

        response = chain.invoke(question)
        sources = retriever.invoke(question)
        
        return {
            "answer": response,
            "sources": [
                {
                    "content": doc.page_content,
                    "source": doc.metadata.get("source", "Unknown"),
                }
                for doc in sources
            ]
        }
                
def main():
    # Initialize RAG system
    docs_path = Path(os.getenv("DOCS_PATH"))
    rag = MarkdownRAG(str(docs_path))
    
    # Create index (only needed once)
    rag.create_index()
    
    # Example query
    diff = sys.stdin.read()
    description = rag.describe(diff)
    print("\n\n# Steve: ")
    answer = rag.query(f"Description: {description} Diff: {diff}")
    output = []
    output.append("\n## Referenced Documentation:")
    for doc in answer["sources"]:
        source_label = doc.get("source", "Unknown")
        output.append(f"\n- **{source_label}**")
    print("\n\n" + "".join(output))

if __name__ == "__main__":
    main()
