use futures::StreamExt;
use rig::providers::openai;
use rig::streaming::{StreamingChoice, StreamingPrompt, stream_to_stdout};
mod vectors;
pub async fn prompt(query: String) -> Result<(), anyhow::Error> {
    let openai_client = openai::Client::from_env();
    let search_agent = openai_client
        .agent("gpt-4o")
        .preamble("Make fun of anything given below. Be insulting")
        .temperature(0.9)
        .build();

    let mut stream = search_agent.stream_prompt(&query).await?;
    stream_to_stdout(search_agent, &mut stream).await?;
    Ok(())
}
pub async fn extract(diff: String) -> Result<Vec<String>, anyhow::Error> {
    let openai_client = openai::Client::from_env();
    let search_agent = openai_client
        .agent("gpt-4o")
        .preamble("Extract high-level architectural decisions from the given git diff, your only job is to generate a list of questions prefixing them 'Q:' as to why these decisions were made. If trivial ignore the decision, it is ok to return no questions")
        .temperature(0.9)
        .build();
    let mut buffer = String::new();
    let mut stream = search_agent.stream_prompt(&diff).await?;
    while let Some(result) = stream.next().await {
        let choice = result?;
        match choice {
            StreamingChoice::Message(token) => {
                print!("{}", token);
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
    println!("Question Count:{}", questions.len());
    Ok(questions)
}

pub async fn research(query: String) -> Result<(), anyhow::Error> {
    let openai_client = openai::Client::from_env();
    let vectors = vectors::embed_docs(openai_client.clone()).await?;
    let search_agent = openai_client
        .agent("gpt-4o")
        .preamble("Does the attached context sufficiently answer the the provided question? Response with one of the three following options only. 'Answers:Implicitly', 'Answers:Explicitly' or 'Answers:Nothing'")
        .dynamic_context(2, vectors)
        .temperature(0.9)
        .build();
    let mut stream = search_agent.stream_prompt(&query).await?;
    stream_to_stdout(search_agent, &mut stream).await?;
    Ok(())
}
