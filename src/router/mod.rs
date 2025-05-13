use crate::websocket::messages::{InboundMessage, OutboundMessage};
use tokio::sync::mpsc;

pub struct Router {
    // Read incoming messages
    inbound_rx: mpsc::Receiver<InboundMessage>,
    // Forward outgoing messages
    outbound_tx: mpsc::Sender<OutboundMessage>,
}

impl Router {
    pub fn new(
        inbound_rx: mpsc::Receiver<InboundMessage>,
        outbound_tx: mpsc::Sender<OutboundMessage>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            inbound_rx,
            outbound_tx,
        })
    }

    pub async fn start(mut self) -> anyhow::Result<()> {
        while let Some(message) = self.inbound_rx.recv().await {
            match message {
                InboundMessage::UserPrompt { prompt } => {
                    println!("Received user prompt: {}", prompt);

                    self.outbound_tx
                        .send(OutboundMessage::LLMResponse {
                            message: "Hello!!".to_string(),
                        })
                        .await?;
                }
                _ => {
                    println!("Invalid message type received");
                }
            }
        }
        Ok(())
    }
}
