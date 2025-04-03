use clap::{Parser, Subcommand};
use dotenv::dotenv;
use std::io::{self, Read};
use std::str;
mod audit;
mod git;
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
    Search {
        #[arg(trailing_var_arg = true)]
        query: Vec<String>,
    },
    /// Analyze the current branch against main
    Audit,
    /// A toxic code review
    Roast,
}

async fn run_search(query: &Vec<String>, mut reader: impl Read) -> Result<(), anyhow::Error> {
    let query_text = match query.len() > 0 {
        true => query[0].clone(),
        false => {
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
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();
    let cli = Cli::parse();
    match &cli.command {
        Commands::Search { query } => run_search(query, io::stdin()).await,
        Commands::Roast {} => llm::roast().await,
        Commands::Audit {} => audit::run_audit().await,
    }
}
