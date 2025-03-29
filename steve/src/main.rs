use clap::{Parser, Subcommand};
use dotenv::dotenv;
use std::io::{self, Read};
use std::process::Command;
use std::str;
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
    return llm::search(query_text).await;
}

async fn run_audit() -> Result<(), anyhow::Error> {
    let output = Command::new("git")
        .arg("diff")
        .output()
        .expect("Failed to execute git diff");
    if output.status.success() {
        let diff = str::from_utf8(&output.stdout).unwrap().to_string();
        return llm::search(diff).await;
    }
    panic!("Something terrible has occured")
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();
    let cli = Cli::parse();
    match &cli.command {
        Commands::Search { query } => run_search(query, io::stdin()).await,
        Commands::Audit {} => run_audit().await,
    }
}
