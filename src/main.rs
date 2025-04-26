mod code_completion;
mod server;

use server::client::{LLMResponse, send_prompt};
use server::manager::{start_server, stop_server};

use code_completion::prompts::get_prompts_from_file;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let prompts = get_prompts_from_file()?;

    let port = "64057";
    let model_path = "models/qwen2.5-coder-3b-instruct-q4_k_m.gguf";
    // let model_path = "models/mistral-7b-instruct-v0.2-q4_k_m.gguf";

    let server = start_server(model_path, port).await?;
    println!("✅ Server started");

    for prompt in prompts {
        let res = send_prompt(&prompt, port).await?;

        match res {
            LLMResponse::Ok(result) => {
                let prompt_n = result.timings.prompt_n;
                let prompt_ms = result.timings.prompt_ms;
                let predicted_ms = result.timings.predicted_ms;
                let total = prompt_ms + predicted_ms;

                println!("===");
                println!(
                    "GHOST: {} tokens - {:.1}ms => {:.1}ms = {:.1}ms\n{}",
                    prompt_n, prompt_ms, predicted_ms, total, result.content
                );
            }
            LLMResponse::Err(err) => {
                eprintln!("❌ Model error: {}", err.error.message);
                stop_server(server).await?;
                return Err(anyhow::anyhow!("Model failed during prompt"));
            }
        }
    }

    stop_server(server).await?;
    println!("🛑 Server stopped");

    Ok(())
}
