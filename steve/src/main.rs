mod search;

use clap::{Parser, Subcommand};
use std::io::{self, Read};
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

fn run_search(query: &Option<String>, mut reader: impl Read) -> String {
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
    return query_text;
}
fn main() {
    // Parse the command-line arguments
    let cli = Cli::parse();
    match &cli.command {
        Commands::Search { query } => {
            let result = run_search(query, io::stdin());
            println!("Search Result: {}", result)
        }
        Commands::Audit {} => println!("Executed audit"),
    }
}
