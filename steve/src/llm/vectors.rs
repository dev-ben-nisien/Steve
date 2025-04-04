use anyhow::anyhow;
use colored::Colorize;
use rig::{
    embeddings::EmbeddingsBuilder,
    providers::openai::Client,
    vector_store::{
        VectorStoreIndex,
        in_memory_store::{InMemoryVectorIndex, InMemoryVectorStore},
    },
};
use std::{env, path::Path};
use tokio::fs;
pub async fn embed_docs(
    client: Client,
) -> Result<
    InMemoryVectorIndex<rig::providers::openai::EmbeddingModel, Vec<std::string::String>>,
    anyhow::Error,
> {
    let embedding_model = client.embedding_model("text-embedding-ada-002");
    let path;
    match env::var("STEVE_DOCS_PATH") {
        Ok(val) => path = val,
        Err(_) => return Err(anyhow!("Missing STEVE_DOCS_PATH environment variable")),
    }
    let documents = laod_markdown_files(Path::new(&path)).await;
    let embeddings = EmbeddingsBuilder::new(embedding_model.clone())
        .documents(documents)?
        .build()
        .await?;

    let store = InMemoryVectorStore::from_documents(embeddings);
    println!(
        "\n{}",
        "Finsihed Embedding Documents".bold().blue().underline()
    );
    return Ok(store.index(embedding_model));
}

async fn laod_markdown_files(dir: &Path) -> Result<Vec<String>, anyhow::Error> {
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
            markdown_contents.push(content);
        }
    }
    return Ok(markdown_contents);
}
