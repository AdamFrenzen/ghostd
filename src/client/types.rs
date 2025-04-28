use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct LlamaParams {
    pub n_predict: u32,
    pub temperature: Option<f32>,
    pub top_k: Option<u32>,
    pub top_p: Option<f32>,
    pub stop: Option<Vec<&'static str>>,
    pub stream: Option<bool>,
}

impl LlamaParams {
    pub fn new(n_predict: u32) -> Self {
        Self {
            n_predict,
            ..Default::default()
        }
    }

    pub fn with_temperature(mut self, temp: f32) -> Self {
        self.temperature = Some(temp);
        self
    }

    pub fn with_top_k(mut self, top_k: u32) -> Self {
        self.top_k = Some(top_k);
        self
    }

    pub fn with_top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p);
        self
    }

    pub fn with_stop(mut self, stop: Vec<&'static str>) -> Self {
        self.stop = Some(stop);
        self
    }

    pub fn with_stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }
}

// chat payload
// {
//   "messages": [
//     {"role": "system", "content": "You are a helpful assistant."},
//     {"role": "user", "content": "Tell me a story about dragons."}
//   ],
//   "n_predict": 128,
//   "temperature": 0.7,
//   "top_k": 40,
//   "top_p": 0.95,
//   "stop": ["</s>"]
// }

#[derive(Debug, Serialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct ChatRequest {
    pub messages: Vec<Message>,
    #[serde(flatten)]
    pub params: LlamaParams,
}

// completion payload
// {
//   "prompt": "Write a Rust function that adds two numbers.",
//   "n_predict": 128,
//   "temperature": 0.8,
//   "top_k": 40,
//   "top_p": 0.95,
//   "stop": ["\n\n"]
// }

#[derive(Debug, Serialize)]
pub struct CompletionRequest {
    pub prompt: String,
    #[serde(flatten)]
    pub params: LlamaParams,
}

// response

#[derive(Debug, Deserialize)]
pub struct Timings {
    pub prompt_ms: f64,
    pub prompt_n: i32,
    pub predicted_ms: f64,
}

#[derive(Debug, Deserialize)]
pub struct Response {
    pub content: String,
    pub timings: Timings,
}

#[derive(Debug, Deserialize)]
pub struct ErrorData {
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub error: ErrorData,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum LlamaResponse {
    Ok(Response),
    Err(ErrorResponse),
}
