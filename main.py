import os
import sys
from typing import List, Dict
from pathlib import Path

from langchain_community.document_loaders import DirectoryLoader
from langchain.text_splitter import RecursiveCharacterTextSplitter
from langchain_core.output_parsers import StrOutputParser
from langchain_core.runnables.passthrough import RunnablePassthrough
from langchain_openai import OpenAIEmbeddings
from langchain_openai import ChatOpenAI
from langchain.chains import RetrievalQA, LLMChain
from langchain.prompts import PromptTemplate
from langchain.vectorstores.docarray import DocArrayInMemorySearch
from dotenv import load_dotenv

# Load environment variables from .env file
load_dotenv()

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
        
        # Initialize LLM
        self.llm = ChatOpenAI(
            model_name="gpt-4o-mini",
            temperature=0.2,
            openai_api_key=self.api_key
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
    def describe(self, prompt: str, system_prompt: str = None, temperature: float = None) -> str:
        llm = self.llm if temperature is None else ChatOpenAI(
            model_name=self.llm.model_name,
            temperature=temperature,
            openai_api_key=self.api_key
        )

        if system_prompt is None:
            system_prompt = """We document architectural decisions using ADRS. Your job is to extract architectural decisions from the result of a git diff. List accurately the areas in which the engineer has made a decision that may have trade offs. Keep the response very consise. Suggest semantically similar topics to search for in documentation"""

        prompt_template = PromptTemplate(
            template="""
            {system_prompt}

            User: {prompt}
            
            Assistant:""",
            input_variables=["system_prompt", "prompt"]
        )

        chain = prompt_template | llm | StrOutputParser()
        response = chain.invoke({"system_prompt": system_prompt, "prompt": prompt})
        
        return response.strip()

    def query(self, question: str, k: int = 8) -> Dict:
        if not self.vector_store:
            self.create_index()

        prompt_template = PromptTemplate(
            template="""
            You have been provided the description of a change, and the git diff an engineer has requested to be merged. Do we have existing documentation for this? We create ADRs for technical decisions is this required for this change?
            Consider the time efficiency when suggesting to write a ADR. Only suggest writing and ADR if the change is significant and spending time documenting is worth it. If an existing ADR can just be updated suggest that instead.
            If its worth creating an ADR suggest some positive & negative consequences of the decision and trade-offs.
            Keep it consise - so its a quick read.
            Change: {question}

            Context: {context}
            
            Answer:""",
            input_variables=["context", "question"],
        )

        retriever = self.vector_store.as_retriever(search_type="mmr",search_kwargs={'k':6, 'lambda_mult':0.25})
        
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
        sources = retriever.get_relevant_documents(question)
        
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
    answer = rag.query(f"Description: {description} Diff: {diff}")
    output = []
    output.append("# Steve:\n")
    output.append(f"{answer['answer']}")
    output.append("\n## Existing Related Documentation:")
    for doc in answer["sources"]:
            source_label = doc.get("source", "Unknown")
            output.append(f"\n- **{source_label}**")
    print("".join(output))

if __name__ == "__main__":
    main()