use crate::llm;
use std::{process::Command, str};

pub async fn run_audit() -> Result<(), anyhow::Error> {
    let output = Command::new("git")
        .arg("diff")
        .output()
        .expect("Failed to execute git diff");
    if output.status.success() {
        let diff = str::from_utf8(&output.stdout).unwrap().to_string();
        let questions = llm::extract(diff).await?;
        println!("\n---- Searching Documentation ----");
        for question in questions {
            println!("\nSearching documentation for: {question}");
            let _ = llm::research(question).await?;
        }
        return Ok(());
    }
    panic!("Something terrible has occured")
}
