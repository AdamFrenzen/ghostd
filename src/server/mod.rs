use reqwest::Client;
use std::process::Stdio;
use std::time::Duration;
use tokio::process::{Child, Command};

pub async fn start_server(model_path: &str, port: &str) -> anyhow::Result<Child> {
    let server_path = "./vendor/llama.cpp/build/bin/llama-server";

    let model_server = Command::new(server_path)
        .arg("-m")
        .arg(model_path)
        .arg("--port")
        .arg(port)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    async fn wait_for_server(port: &str) -> anyhow::Result<()> {
        let client = Client::new();
        let url = format!("http://localhost:{}/", port);

        for _ in 0..10 {
            match client.get(&url).send().await {
                Ok(res) if res.status().is_success() => {
                    return Ok(());
                }
                // Ok(res) if res.status() == 503 => {
                //     println!("loading: {:?}", res);
                //     tokio::time::sleep(Duration::from_millis(500)).await;
                // }
                _ => {
                    println!("sleep");
                    tokio::time::sleep(Duration::from_millis(500)).await;
                }
            }
        }

        Err(anyhow::anyhow!("Server did not become ready in time"))
    }

    wait_for_server(port).await?;
    Ok(model_server)
}

pub async fn stop_server(mut child: Child) -> anyhow::Result<()> {
    child.kill().await?;
    Ok(())
}
