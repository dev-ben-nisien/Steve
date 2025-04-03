use crate::{git, llm};
use colored::Colorize;
use std::str;

pub async fn run_audit() -> Result<(), anyhow::Error> {
    let diff = git::get_diff();
    let questions = llm::extract(diff).await?;
    let mut answers = Vec::new();
    println!("\n{}", "Searching Documentation".bold().blue().underline());
    for question in questions {
        println!("\n{}{}", "Question: ".blue(), question);
        let result = llm::research(question).await?;
        handle_response(&result, &mut answers);
    }
    print_stats(&answers);
    return Ok(());
}
enum Answer {
    Missing = 0,
    Present = 1,
}

fn handle_response(result: &str, answers: &mut Vec<Answer>) {
    let answer;
    match result {
        "Answers:Present" => {
            println!("\n{}", "Found related documentation".bold().green());
            answer = Answer::Present;
        }
        "Answers:Missing" => {
            println!(
                "\n{}",
                "Cannot find anything in documentation about this"
                    .bold()
                    .red()
            );
            answer = Answer::Missing;
        }
        &_ => {
            answer = Answer::Missing;
        }
    }
    answers.push(answer);
}
fn print_stats(answers: &Vec<Answer>) {
    let mut missing: f32 = 0.0;
    let mut present: f32 = 0.0;
    for answer in answers {
        match answer {
            Answer::Missing => missing += 1.0,
            Answer::Present => present += 1.0,
        }
    }
    println!("\n{}", "Audit Results".bold().blue().underline());
    println!("\n{}{}", "Not Documented: ".bold().red(), missing);
    println!("\n{}{}", "Found: ".bold().green(), present);
    println!(
        "\n{}{}%\n",
        "Average: ".bold().blue(),
        ((present) / (missing + present)) * 100.0
    );
}
