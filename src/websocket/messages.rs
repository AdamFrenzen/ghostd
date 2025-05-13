use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum InboundMessage {
    UserPrompt { prompt: String },
    Temporary { temp: String },
    // ... more message types here
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum OutboundMessage {
    LLMResponse { message: String },
    LLMResponseStream { token: String },
    CompletionEnd,
}
