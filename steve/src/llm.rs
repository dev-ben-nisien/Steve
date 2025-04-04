use colored::Colorize;
use futures::StreamExt;
use indicatif::ProgressBar;
use rig::completion::Prompt;
use rig::providers::openai;
use rig::streaming::{StreamingChoice, StreamingPrompt, stream_to_stdout};

use crate::git;
mod vectors;

pub async fn extract(diff: &String) -> Result<Vec<String>, anyhow::Error> {
    println!("\n{}\n", "Analysing Decisions".bold().blue().underline());
    let openai_client = openai::Client::from_env();
    let search_agent = openai_client
        .agent("gpt-4o")
        .preamble("Extract high-level, not code-specific, architectural questions from the given git diff, your only job is to generate a list of questions prefixing them 'Q:' as to why these decisions were made. If trivial ignore, it is ok to return no questions. For example: Q: Why was library X chosen to solve this problem over library Y?")
        .temperature(0.9)
        .build();
    let mut buffer = String::new();
    let mut stream = search_agent.stream_prompt(&diff).await?;
    let pb = ProgressBar::new(200);

    while let Some(result) = stream.next().await {
        let choice = result?;
        match choice {
            StreamingChoice::Message(token) => {
                pb.inc(1);
                buffer.push_str(&token);
            }
            StreamingChoice::ToolCall(_, _, _) => todo!(),
        }
    }
    let questions: Vec<String> = buffer
        .split("Q:")
        .filter_map(|chunk| {
            let trimmed = chunk.trim();
            if !trimmed.is_empty() {
                Some(trimmed.to_string())
            } else {
                None
            }
        })
        .collect();
    pb.finish_with_message("Done");
    println!(
        "\n\n{}{}",
        "Decision Count: ".bold().blue(),
        questions.len()
    );
    Ok(questions)
}

pub async fn research(query: String) -> Result<String, anyhow::Error> {
    let openai_client = openai::Client::from_env();
    let vectors = vectors::embed_docs().await?;
    let search_agent = openai_client
        .agent("gpt-4o")
        .preamble("Does the attached context answer the the provided question? Response with one of the three following options only. 'Answers:Present' or 'Answers:Missing'")
        .dynamic_context(4, vectors)
        .temperature(1.0)
        .build();
    let response = search_agent.prompt(query).await?;
    Ok(response)
}
pub async fn search(query: String) -> Result<(), anyhow::Error> {
    println!("{}", "\nBeginning Search...\n".bold().blue().underline());
    let openai_client = openai::Client::from_env();
    let vectors = vectors::embed_docs().await?;
    let search_agent = openai_client
        .agent("gpt-4o")
        .preamble("You are a architectural decision records librarian. You have been given the context of the related documentation. Explain this to the user consisely & clearly, make it relevant to the question asked. Feel free to add examples or addition context. But do not stray from the source of truth provided.")
        .dynamic_context(4, vectors)
        .temperature(1.0)
        .build();
    let mut stream = search_agent.stream_prompt(&query).await?;
    stream_to_stdout(search_agent, &mut stream).await?;
    Ok(())
}
pub async fn roast() -> Result<(), anyhow::Error> {
    println!(
        "{}",
        "\nYour feelings maybe hurt...\n".bold().blue().underline()
    );
    let diff = git::get_diff();
    let openai_client = openai::Client::from_env();
    let search_agent = openai_client
        .agent("gpt-4o")
        .preamble("Do a code review of the current changes. Be brutally honest and insulting.")
        .temperature(0.9)
        .build();

    let mut stream = search_agent.stream_prompt(&diff).await?;
    stream_to_stdout(search_agent, &mut stream).await?;
    Ok(())
}
