use futures_util::StreamExt;
use tokio::sync::mpsc::Sender;
use tokio_tungstenite::tungstenite::Message;

use crate::websocket::messages::InboundMessage;

pub async fn start(
    mut ws_reader: impl StreamExt<Item = Result<Message, tokio_tungstenite::tungstenite::Error>> + Unpin,
    tx: Sender<InboundMessage>,
) -> anyhow::Result<()> {
    while let Some(msg) = ws_reader.next().await {
        let msg = msg?; // Error handling: if WebSocket errors, return early

        if msg.is_text() {
            let text = msg.into_text()?;
            println!("recieved: {}", text);
            let inbound: InboundMessage = serde_json::from_str(&text)?;

            tx.send(inbound).await?;
        }
    }

    Ok(())
}
