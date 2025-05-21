mod agents;
mod client;
mod code_completion;
mod llm_backends;
mod router;
mod websocket;

use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio_tungstenite::accept_async;

use router::Router;

use websocket::messages::InboundMessage;
use websocket::server::{WebSocketServer, WebSocketSession};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (inbound_tx, inbound_rx) = mpsc::channel(32);
    let (outbound_tx, outbound_rx) = mpsc::channel(32);

    let router = Router::new(inbound_rx, outbound_tx)?;
    tokio::spawn(router.start());

    let ws_server = WebSocketServer::new(inbound_tx.clone(), outbound_rx);
    tokio::spawn(ws_server.run());

    let demo_message = InboundMessage::UserPrompt {
        prompt: "test".to_string(),
    };
    inbound_tx.clone().send(demo_message).await?;

    tokio::signal::ctrl_c().await?;
    println!("Shutting down gracefully.");

    ///////////////////////////////
    ///////////////////////////////
    ////////
    // let prompts = get_prompts_from_file()?;
    //
    // let port = "64058";
    // // let model_path = "./models/qwen2.5-coder-3b-instruct-q4_k_m.gguf";
    // let model_path = "./models/mistral-7b-instruct-v0.2-q4_k_m.gguf";
    //
    // let server = start_server(model_path, port).await?;
    // println!("✅ Server started");
    //
    // let client = Client::new();
    // let chat_client = ChatClient::new(port, client);
    // let chat_agent = ChatAgent::new(chat_client);
    ////////

    // let response = chat_agent.send("what is the meaning of life?").await?;
    // response.print();

    // chat_agent
    //     .send("In Rust how should I pass through params to be this struct the LlamaParams struct, right now I am using a builder pattern but I need to pass these as arugments instead of hardcoding values:\n\n`let params = LlamaParams::new(1000).with_temperature(0.2);`")
    //     .await?
    //     .print();

    // chat_agent
    //     .send("How should i have my Rust backend stream the llama.cpp responses to my Lua Neovim frontend? When I enable the stream how should that communication work?")
    //     .await?
    //     .print();

    // match res {
    //     LlamaResponse::Ok(result) => {
    //         let prompt_n = result.timings.prompt_n;
    //         let prompt_ms = result.timings.prompt_ms;
    //         let predicted_ms = result.timings.predicted_ms;
    //         let total = prompt_ms + predicted_ms;
    //
    //         println!("===");
    //         println!(
    //             "GHOST: {} tokens - {:.1}ms => {:.1}ms = {:.1}ms\n{}",
    //             prompt_n, prompt_ms, predicted_ms, total, result.content
    //         );
    //     }
    //     LlamaResponse::Err(err) => {
    //         eprintln!("❌ Model error: {}", err.error.message);
    //         stop_server(server).await?;
    //         return Err(anyhow::anyhow!("Model failed during prompt"));
    //     }
    // }
    //
    // completion prompts:
    // for prompt in prompts {
    //     let res = send_completion_prompt(prompt, port).await?;
    //
    //     match res {
    //         LlamaResponse::Ok(result) => {
    //             let prompt_n = result.timings.prompt_n;
    //             let prompt_ms = result.timings.prompt_ms;
    //             let predicted_ms = result.timings.predicted_ms;
    //             let total = prompt_ms + predicted_ms;
    //
    //             println!("===");
    //             println!(
    //                 "GHOST: {} tokens - {:.1}ms => {:.1}ms = {:.1}ms\n{}",
    //                 prompt_n, prompt_ms, predicted_ms, total, result.content
    //             );
    //         }
    //         LlamaResponse::Err(err) => {
    //             eprintln!("❌ Model error: {}", err.error.message);
    //             stop_server(server).await?;
    //             return Err(anyhow::anyhow!("Model failed during prompt"));
    //         }
    //     }
    // }

    // stop_server(server).await?;
    // println!("🛑 Server stopped");

    Ok(())
}
