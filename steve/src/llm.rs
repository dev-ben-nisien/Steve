use rig::providers::openai;
use rig::streaming::{StreamingPrompt, stream_to_stdout};
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
pub async fn research(query: String) -> Result<(), anyhow::Error> {
    let openai_client = openai::Client::from_env();
    let vectors = vectors::embed_docs(openai_client.clone()).await?;
    let search_agent = openai_client
        .agent("gpt-4o")
        .preamble("Make fun of anything given below. Be insulting")
        .dynamic_context(2, vectors)
        .temperature(0.9)
        .build();

    let mut stream = search_agent.stream_prompt(&query).await?;
    stream_to_stdout(search_agent, &mut stream).await?;
    Ok(())
}
