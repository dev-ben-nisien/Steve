use rig::{completion::Prompt, providers::openai};
pub async fn search(query: String) -> String {
    let openai_client = openai::Client::from_env();
    let search_agent = openai_client
        .agent("gpt-4o")
        .preamble("Make fun of anything given below. Be insulting")
        .temperature(0.9)
        .build();

    return search_agent
        .prompt(query)
        .await
        .expect("Failed to prompt the agent");
}
