use std::thread::JoinHandle;

use anyhow::Result;
use futures_util::StreamExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::accept_async;
// use tokio_tungstenite::tungstenite::{Message, Utf8Bytes};

use crate::websocket::channel_to_ws;
use crate::websocket::messages::{InboundMessage, OutboundMessage};
use crate::websocket::ws_to_channel;

pub struct WebSocketSession {
    // model_registry: Arc<ModelRegistry>,
    inbound_tx: mpsc::Sender<InboundMessage>,
    // outbound_rx: mpsc::Receiver<OutboundMessage>,
    // ws_to_channel_task: JoinHandle<anyhow::Result<()>>,
    // channel_to_ws_task: JoinHandle<anyhow::Result<()>>,
}

impl WebSocketSession {
    pub async fn new(
        ws_stream: WebSocketStream<TcpStream>,
        inbound_tx: mpsc::Sender<InboundMessage>,
        outbound_rx: mpsc::Receiver<OutboundMessage>,
    ) -> anyhow::Result<Self> {
        let (ws_writer, ws_reader) = ws_stream.split();

        // Task that receives WebSocket messages and forwards them into the internal channel
        let inbound_tx_clone = inbound_tx.clone();
        let ws_to_channel_task =
            tokio::spawn(async move { ws_to_channel::start(ws_reader, inbound_tx_clone).await });

        // Task that takes internal messages and sends them out to the WebSocket
        let channel_to_ws_task =
            tokio::spawn(async move { channel_to_ws::start(ws_writer, outbound_rx).await });

        // Stop both tasks if one fails
        tokio::spawn(async move {
            tokio::select! {
            result = ws_to_channel_task => {
                match result {
                    Ok(Ok(())) => println!("reader completed successfully"),
                    Ok(Err(e)) => eprintln!("reader failed with error: {}", e),
                    Err(join_err) => eprintln!("reader task panicked or was cancelled: {}", join_err),
                }
            },
            result = channel_to_ws_task => {
                match result {
                    Ok(Ok(())) => println!("writer completed successfully"),
                    Ok(Err(e)) => eprintln!("writer failed with error: {}", e),
                    Err(join_err) => eprintln!("writer task panicked or was cancelled: {}", join_err),
                }
            },
            }
        });

        Ok(Self {
            // model_registry,
            inbound_tx,
        })
    }

    pub async fn run(mut self) -> anyhow::Result<()> {
        // tokio::select! on tasks
        // Forward messages, process prompts, etc.
        Ok(())
    }

    pub async fn send(self, message: InboundMessage) -> anyhow::Result<()> {
        self.inbound_tx.send(message).await?;
        Ok(())
    }
}
