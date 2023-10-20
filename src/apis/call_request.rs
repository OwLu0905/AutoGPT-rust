use crate::models::general::llm::{ChatCompletion, Message};
use dotenv::dotenv;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use std::env;

// Call LLM (i.e. GPT-4)
pub async fn call_gpt(messages: Vec<Message>) {
    dotenv().ok();

    // Extract API Key Information
    let api_key: String = env::var("OPEN_AI_KEY").expect("KEY IS NOT FOUND");
    let api_org: String = env::var("OPEN_AI_ORG").expect("ORG_ID IS NOT FOUND");

    // Confirm endpoint
    let url: &str = "https://api.openai.com/v1/chat/completions";

    // Create headers
    let mut headers: HeaderMap = HeaderMap::new();
    // Create api key header
    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
    );

    // Create Open AI Org header
    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(api_org.as_str()).unwrap(),
    );

    // Create client
    let client = Client::builder().default_headers(headers).build().unwrap();

    // Create chat completion
    let chat_completion: ChatCompletion = ChatCompletion {
        // model: "gpt-4".to_string(),
        model: "gpt-3.5-turbo".to_string(),
        messages,
        temperature: 0.1,
    };

    // Troubleshooting
    let res_raw = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .unwrap();

    dbg!(res_raw.text().await.unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_call_to_openai() {
        let message = Message {
            role: "user".to_string(),
            content: "this is a test. Give me a short response.".to_string(),
        };

        let messages: Vec<Message> = vec![message];

        call_gpt(messages).await;
    }
}
