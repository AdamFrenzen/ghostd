use crate::client::types::CompletionRequest;
use crate::client::types::LlamaParams;

use reqwest::Client;
use serde_json;

use super::types::LlamaResponse;

pub async fn send_completion_prompt(prompt: String, port: &str) -> anyhow::Result<LlamaResponse> {
    let client = Client::new();
    let url = format!("http://localhost:{}/completion", port);

    let params = LlamaParams::new(16)
        .with_temperature(0.2)
        .with_stop(vec!["\n"]);

    let req = CompletionRequest { prompt, params };
    let res = client.post(&url).json(&req).send().await?;
    let body = res.text().await?;
    // println!("{body}");

    let parsed: LlamaResponse = serde_json::from_str(&body)?;
    Ok(parsed)
}
