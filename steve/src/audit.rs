use crate::llm;
use colored::Colorize;
use std::{process::Command, str};

pub async fn run_audit() -> Result<(), anyhow::Error> {
    let output = Command::new("git")
        .arg("diff")
        .output()
        .expect("Failed to execute git diff");
    if output.status.success() {
        let diff = str::from_utf8(&output.stdout).unwrap().to_string();
        let questions = llm::extract(diff).await?;
        let mut answers = Vec::new();
        println!("\n---- Searching Documentation ----");
        for question in questions {
            println!("\nSearching documentation for: {question}");
            let result = llm::research(question).await?;
            handle_response(&result, &mut answers);
        }
        return Ok(());
    }
    panic!("Something terrible has occured")
}
enum Answer {
    Nothing = 0,
    Implicitly = 50,
    Explicitly = 100,
}

fn handle_response(result: &str, answers: &mut Vec<Answer>) {
    let answer;
    match result {
        "Answers:Explicitly" => {
            println!("\n{}", "This has an explicit answer".bold().green());
            answer = Answer::Explicitly
        }
        "Answers:Implicitly" => {
            println!(
                "\n{}",
                "This has an implicit/partial answer".bold().yellow()
            );
            answer = Answer::Implicitly
        }
        "Answers:Nothing" => {
            println!(
                "\n{}",
                "Cannot find anything in documentation about this"
                    .bold()
                    .red()
            );
            answer = Answer::Nothing
        }
        &_ => answer = Answer::Nothing,
    }
    answers.push(answer);
}
