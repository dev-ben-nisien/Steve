use anyhow::anyhow;
use colored::Colorize;
use rig::{
    Embed,
    embeddings::EmbeddingsBuilder,
    providers::openai::{self},
    vector_store::in_memory_store::{InMemoryVectorIndex, InMemoryVectorStore},
};
use serde::{Deserialize, Serialize};
use std::{env, path::Path};
use tokio::fs;

#[derive(Clone, Deserialize, Debug, Serialize, Eq, PartialEq, Default)]
pub struct MarkdownFile {
    id: String,
    content: String,
}
impl Embed for MarkdownFile {
    fn embed(
        &self,
        embedder: &mut rig::embeddings::TextEmbedder,
    ) -> Result<(), rig::embeddings::EmbedError> {
        Ok(embedder.embed(self.content.clone()))
    }
}
pub async fn embed_docs()
-> Result<InMemoryVectorIndex<rig::providers::openai::EmbeddingModel, MarkdownFile>, anyhow::Error>
{
    let client = openai::Client::from_env();
    let embedding_model = client.embedding_model("text-embedding-ada-002");
    let path;
    match env::var("STEVE_DOCS_PATH") {
        Ok(val) => path = val,
        Err(_) => return Err(anyhow!("Missing STEVE_DOCS_PATH environment variable")),
    }
    let documents = load_markdown_files(Path::new(&path)).await;
    let embeddings = EmbeddingsBuilder::new(embedding_model.clone())
        .documents(documents?)?
        .build()
        .await?;

    let store = InMemoryVectorStore::from_documents_with_id_f(embeddings, |doc| doc.id.clone());
    let index = store.index(embedding_model);

    println!("\n{}", "Finished Embedding Documents".blue());
    return Ok(index);
}

async fn load_markdown_files(dir: &Path) -> Result<Vec<MarkdownFile>, anyhow::Error> {
    let mut markdown_contents = Vec::new();
    let mut dir = fs::read_dir(dir).await?;
    while let Some(entry) = dir.next_entry().await? {
        let file_type = entry.file_type().await?;
        if !file_type.is_file() {
            continue;
        }
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "md") {
            let content = fs::read_to_string(&path).await?;
            markdown_contents.push(MarkdownFile {
                id: path
                    .file_name()
                    .map(|os_str| os_str.to_string_lossy().into_owned())
                    .unwrap(),
                content,
            });
        }
    }
    return Ok(markdown_contents);
}
