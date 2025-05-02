use anyhow::Context;
use futures_util::SinkExt;
use tokio::sync::mpsc::Receiver;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::Utf8Bytes;

use crate::websocket::messages::OutboundMessage;

pub async fn start(
    mut ws_writer: impl SinkExt<Message, Error = tokio_tungstenite::tungstenite::Error> + Unpin,
    mut rx: Receiver<OutboundMessage>,
) -> anyhow::Result<()> {
    while let Some(outbound) = rx.recv().await {
        let json = serde_json::to_string(&outbound)?;
        let bytes = Utf8Bytes::from(json);

        ws_writer
            .send(Message::Text(bytes))
            .await
            .context("failed to send websocket message")?;
    }

    Ok(())
}
