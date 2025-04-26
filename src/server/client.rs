use reqwest::Client;
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
pub struct CompletionResponse {
    pub content: String,
    pub timings: Timings,
}

#[derive(Debug, Deserialize)]
pub struct Timings {
    pub prompt_ms: f64, // or i32 if you're sure it's always an integer
    pub prompt_n: i32,
    pub predicted_ms: f64,
}

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub error: ErrorData,
}

#[derive(Debug, Deserialize)]
pub struct ErrorData {
    pub message: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum LLMResponse {
    Ok(CompletionResponse),
    Err(ErrorResponse),
}

pub async fn send_prompt(prompt: &str, port: &str) -> anyhow::Result<LLMResponse> {
    let client = Client::new();
    let url = format!("http://localhost:{}/completion", port);

    let res = client
        .post(&url)
        .json(&json!({
            "prompt": prompt,
            // "n_predict": 32,
            "n_predict": 16,
            "stop": ["\n"],
            // "stop": ["\n\n"],
            "temperature": 0.2
        }))
        .send()
        .await?;

    let body = res.text().await?;
    // println!("{body}");
    let parsed: LLMResponse = serde_json::from_str(&body)?;

    Ok(parsed)
}

/* PAYLOAD ITEMS IN THE BODY
{
  "tokens": [],
  "id_slot": 0,
  "stop": true,
  "model": "gpt-3.5-turbo",
  "tokens_predicted": 64,
  "tokens_evaluated": 113,
uyy     ;/`.`
  "generation_settings": {
    "n_predict": 64,
    "seed": 4294967295,
    "temperature": 0.20000000298023224,
    "dynatemp_range": 0.0,
    "dynatemp_exponent": 1.0,
    "top_k": 40,
    "top_p": 0.949999988079071,
    "min_p": 0.05000000074505806,
    "xtc_probability": 0.0,
    "xtc_threshold": 0.10000000149011612,
    "typical_p": 1.0,
    "repeat_last_n": 64,
    "repeat_penalty": 1.0,
    "presence_penalty": 0.0,
    "frequency_penalty": 0.0,
    "dry_multiplier": 0.0,
    "dry_base": 1.75,
    "dry_allowed_length": 2,
    "dry_penalty_last_n": 4096,
    "dry_sequence_breakers": [
      "\n",
      ":",
      "\"",
      "*"
    ],

    "mirostat": 0,
    "mirostat_tau": 5.0,
    "mirostat_eta": 0.10000000149011612,
    "stop": [
      "\n\n"
    ],

    "max_tokens": 64,
    "n_keep": 0,
    "n_discard": 0,
    "ignore_eos": false,
    "stream": false,
    "logit_bias": [],
    "n_probs": 0,
    "min_keep": 0,
    "grammar": "",
    "grammar_lazy": false,
    "grammar_triggers": [],
    "preserved_tokens": [],
    "chat_format": "Content-only",

    "samplers": [
      "penalties",
      "dry",
      "top_k",
      "typ_p",
      "top_p",
      "min_p",
      "xtc",
      "temperature"
    ],

    "speculative.n_max": 16,
    "speculative.n_min": 0,
    "speculative.p_min": 0.75,
    "timings_per_token": false,
    "post_sampling_probs": false,
    "lora": []
  },

  "prompt": "\nimport torch\nimport torch.nn as nn\nimport torch.optim as optim\nfrom torch.utils.data import DataLoader\n\nclass MyModel(nn.Module):\n    def __init__(self):\n        super().__init__()\n        self.linear = nn.Linear(128,\n    10)\n\n    def forward(self,\n    x):\n        return self.linear(x)\n\nmodel = MyModel()\noptimizer = optim.Adam(model.parameters())\ncriterion = nn.CrossEntropyLoss()\n\ntrain_loader = DataLoader(dataset,\n    batch_size=32,\n    shuffle=True)\n\n# training loop\nfor epoch in range(\n",
  "has_new_line": true,
  "truncated": false,
  "stop_type": "limit",
  "stopping_word": "",
  "tokens_cached": 176,

  "timings": {
    "prompt_n": 113,
    "prompt_ms": 531.403,
    "prompt_per_token_ms": 4.702681415929204,
    "prompt_per_second": 212.6446406964206,
    "predicted_n": 64,
    "predicted_ms": 2571.477,
    "predicted_per_token_ms": 40.179328125,
    "predicted_per_second": 24.888420156975933
  }
}
*/
