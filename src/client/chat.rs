use crate::client::types::LlamaParams;
use crate::client::types::Timings;

use reqwest::Client;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

impl Message {
    pub fn new(role: &str, content: &str) -> Self {
        Self {
            role: role.to_string(),
            content: content.to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ChatRequest {
    pub messages: Vec<Message>,
    #[serde(flatten)]
    pub params: LlamaParams,
}

// LLM response is inside of choices.message
#[derive(Debug, Deserialize)]
pub struct Choice {
    pub message: Message,
}

#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub choices: Vec<Choice>,
    pub timings: Timings,
}

impl ChatResponse {
    pub fn print(&self) {
        let result = &self.choices[0];

        let response: &str = &result.message.content;

        let prompt_n = self.timings.prompt_n;
        let prompt_ms = self.timings.prompt_ms;
        let predicted_ms = self.timings.predicted_ms;
        let total = prompt_ms + predicted_ms;

        println!(
            "GHOST: {} tokens - {:.1}ms => {:.1}ms = {:.1}ms\n{}\n",
            prompt_n, prompt_ms, predicted_ms, total, response
        );
    }
}

pub struct ChatClient {
    url: String,
    http_client: Client,
}

impl ChatClient {
    pub fn new(port: &str, http_client: Client) -> Self {
        Self {
            url: format!("http://localhost:{}/v1/chat/completions", port),
            http_client,
        }
    }

    pub async fn send_chat_prompt(
        &self,
        messages: Vec<Message>,
        params: LlamaParams,
    ) -> anyhow::Result<ChatResponse> {
        let req = ChatRequest { messages, params };
        let res = self.http_client.post(&self.url).json(&req).send().await?;

        let parsed = res.json::<ChatResponse>().await?;
        Ok(parsed)
    }
}
