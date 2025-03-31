use clap::{Parser, Subcommand};
use dotenv::dotenv;
use std::io::{self, Read};
use std::str;
mod audit;
mod llm;
/// STEVE: Search Technical Evidence Very Easy.
#[derive(Parser, Debug)]
#[command(name = "steve", version, author, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Search evidence on a given prompt
    Search { query: Option<String> },
    /// Analyize the current branch against main
    Audit,
    /// A toxic code review
    Roast { query: Option<String> },
}

async fn run_search(query: &Option<String>, mut reader: impl Read) -> Result<(), anyhow::Error> {
    let query_text = match query {
        Some(q) => q.clone(),
        None => {
            let mut buffer = String::new();
            reader
                .read_to_string(&mut buffer)
                .expect("Failed to read from STDIN");
            buffer.trim().to_string()
        }
    };
    return llm::prompt(query_text).await;
}
async fn run_prompt(query: &Option<String>, mut reader: impl Read) -> Result<(), anyhow::Error> {
    let query_text = match query {
        Some(q) => q.clone(),
        None => {
            let mut buffer = String::new();
            reader
                .read_to_string(&mut buffer)
                .expect("Failed to read from STDIN");
            buffer.trim().to_string()
        }
    };
    return llm::prompt(query_text).await;
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();
    let cli = Cli::parse();
    match &cli.command {
        Commands::Search { query } => run_search(query, io::stdin()).await,
        Commands::Roast { query } => run_prompt(query, io::stdin()).await,
        Commands::Audit {} => audit::run_audit().await,
    }
}
