use std::fs;

use reqwest::Client;
use serde::de::DeserializeOwned;

use crate::apis::call_request::call_gpt;
use crate::models::general::llm::Message;

use super::command_line::PrintCommand;

const CODE_TEMPLATE_PATH: &str =
    "/Users/OwLu/Desktop/Udemy/rust-gpt/basics/web_template/src/code_template.rs";
const EXEC_MAIN_PATH: &str = "/Users/OwLu/Desktop/Udemy/rust-gpt/basics/web_template/src/main.rs";
const API_SCHEMA_PATH: &str =
    "/Users/OwLu/Desktop/Udemy/rust-gpt/basics/auto_gippity/schemas/api_schema.json";

// Extend ai function to encourage specific output
pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) -> Message {
    let ai_function_str = ai_func(func_input);

    // Extend the string to encourage only printing the output
    let msg = format!(
        "FUNCTION {} 
  INSTRUCTION: You are a function printer. You ONLY print the results of functions.
  Nothing else. No commentary. Here is the input to the function: {}.
  Print out what the function will return.",
        ai_function_str, func_input
    );

    // Return message
    Message {
        role: "system".to_string(),
        content: msg,
    }
}

// NOTE: Performs call to LLM GPT
pub async fn ai_task_request(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> String {
    // NOTE: Extend AI function
    let extended_msg: Message = extend_ai_function(function_pass, &msg_context);

    // NOTE: Print current status
    PrintCommand::AICall.print_agent_message(agent_position, agent_operation);

    // NOTE: Get LLM response
    let llm_result: Result<String, Box<dyn std::error::Error + Send>> =
        call_gpt(vec![extended_msg.clone()]).await;

    // TODO: Return Success or try again
    let llm_response: String = match llm_result {
        Ok(llm_resp) => llm_resp,
        Err(_) => call_gpt(vec![extended_msg.clone()])
            .await
            .expect("Failed twice to call OpenAI"),
    };

    llm_response
}

// NOTE: Performs call to LLM GPT Decode
pub async fn ai_task_request_decoded<T: DeserializeOwned>(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> T {
    let llm_response =
        ai_task_request(msg_context, agent_position, agent_operation, function_pass).await;

    let decoded_response: T = serde_json::from_str(llm_response.as_str())
        .expect("Failed to decode ai response from serde_json");
    decoded_response
}

// TODO: Check whether request url is valid
pub async fn check_status_code(client: &Client, url: &str) -> Result<u16, reqwest::Error> {
    let response: reqwest::Response = client.get(url).send().await?;
    Ok(response.status().as_u16())
}

// NOTE: Get Code Template
pub fn read_code_template_contents() -> String {
    let path: String = CODE_TEMPLATE_PATH.to_string();
    fs::read_to_string(path).expect("Failed to read code template")
}
// NOTE: Save New Backend Code
pub fn save_backend_code(contents: &str) {
    let path: String = EXEC_MAIN_PATH.to_string();
    fs::write(path, contents).expect("Failed to write main.rs file");
}

// NOTE: Save JSON API Endpoint schema
pub fn save_api_endpoints(api_endpoints: &str) {
    let path: String = API_SCHEMA_PATH.to_string();
    fs::write(path, api_endpoints).expect("Failed to write API Endpoints to file");
}

// NOTE:

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;

    #[test]
    fn tests_extending_ai_function() {
        let extended_msg = extend_ai_function(convert_user_input_to_goal, "dummy variable");
        dbg!(&extended_msg);
        assert_eq!(extended_msg.role, "system".to_string());
    }

    #[tokio::test]
    async fn tests_ai_tasks_request() {
        let ai_func_param =
            "Build me a web server for making stock price api requests.".to_string();
        let res: String = ai_task_request(
            ai_func_param,
            "Managing Agent",
            "Defining user requirements",
            convert_user_input_to_goal,
        )
        .await;

        dbg!(res);
    }
}
