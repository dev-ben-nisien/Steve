use clap::{Parser, Subcommand};
use dotenv::dotenv;
use std::io::{self, Read};
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
    Audit {},
}

async fn run_search(query: &Option<String>, mut reader: impl Read) -> String {
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
    return llm::search(query_text).await;
}
#[tokio::main]
async fn main() {
    dotenv().ok();
    let cli = Cli::parse();
    match &cli.command {
        Commands::Search { query } => {
            let result = run_search(query, io::stdin()).await;
            println!("Search Result: {}", result)
        }
        Commands::Audit {} => println!("Executed audit"),
    }
}
