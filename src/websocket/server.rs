// use tokio::task::JoinHandle;
//
// use futures_util::StreamExt;
// use tokio::net::{TcpListener, TcpStream};
// use tokio::sync::mpsc;
// use tokio_tungstenite::WebSocketStream;
// use tokio_tungstenite::accept_async;
// // use tokio_tungstenite::tungstenite::{Message, Utf8Bytes};
//
// use std::sync::{
//     Arc,
//     atomic::{AtomicBool, Ordering},
// };
//
// use crate::websocket::channel_to_ws;
// use crate::websocket::messages::{InboundMessage, OutboundMessage};
// use crate::websocket::ws_to_channel;
//
// pub struct WebSocketServer {
//     has_connection: Arc<AtomicBool>,
//     inbound_tx: mpsc::Sender<InboundMessage>,
//     outbound_rx: mpsc::Receiver<OutboundMessage>,
// }
//
// impl WebSocketServer {
//     pub fn new(
//         inbound_tx: mpsc::Sender<InboundMessage>,
//         outbound_rx: mpsc::Receiver<OutboundMessage>,
//     ) -> Self {
//         Self {
//             has_connection: Arc::new(AtomicBool::new(false)),
//             inbound_tx,
//             outbound_rx,
//         }
//     }
//
//     pub async fn run(self) -> anyhow::Result<()> {
//         let websocket_port = "64057";
//         let address = format!("127.0.0.1:{}", websocket_port);
//         let listener = TcpListener::bind(&address).await?;
//         println!("WebSocket server listening on ws://{}", address);
//
//         let (session_tx, session_rx) = mpsc::channel(32);
//
//         tokio::spawn(Self::forward_session_to_inbound(
//             session_rx,
//             self.inbound_tx.clone(),
//         ));
//
//         // let inbound_tx_clone = inbound_tx.clone();
//
//         loop {
//             if self.has_connection.load(Ordering::SeqCst) {
//                 tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
//                 continue;
//             }
//
//             let (stream, _) = listener.accept().await?;
//             let ws_stream = accept_async(stream).await?;
//             WebSocketSession::new(ws_stream, session_tx.clone(), self.outbound_rx).await?;
//         }
//     }
//
//     async fn forward_session_to_inbound(
//         mut session_rx: mpsc::Receiver<InboundMessage>,
//         inbound_tx: mpsc::Sender<InboundMessage>,
//     ) {
//         while let Some(msg) = session_rx.recv().await {
//             if inbound_tx.send(msg).await.is_err() {
//                 eprintln!("Failed to forward message to inbound_tx");
//                 break;
//             }
//         }
//     }
// }
//
// pub struct WebSocketSession {
//     // inbound_tx: mpsc::Sender<InboundMessage>,
//     // outbound_rx: mpsc::Receiver<OutboundMessage>,
//     // ws_to_channel_task: JoinHandle<anyhow::Result<()>>,
//     // channel_to_ws_task: JoinHandle<anyhow::Result<()>>,
// }
//
// impl WebSocketSession {
//     pub async fn new(
//         ws_stream: WebSocketStream<TcpStream>,
//         inbound_tx: mpsc::Sender<InboundMessage>,
//         outbound_rx: mpsc::Receiver<OutboundMessage>,
//     ) -> anyhow::Result<Self> {
//         let (ws_writer, ws_reader) = ws_stream.split();
//
//         // Task that receives WebSocket messages and forwards them into the internal channel
//         let inbound_tx_clone = inbound_tx.clone();
//         let ws_to_channel_task =
//             tokio::spawn(async move { ws_to_channel::start(ws_reader, inbound_tx_clone).await });
//
//         // Task that takes internal messages and sends them out to the WebSocket
//         let channel_to_ws_task =
//             tokio::spawn(async move { channel_to_ws::start(ws_writer, outbound_rx).await });
//
//         // channel_to_ws_task.id
//
//         // Stop both tasks if one fails
//         tokio::spawn(Self::manage_tasks(ws_to_channel_task, channel_to_ws_task));
//
//         Ok(Self {
//             is_connected,
//             inbound_tx,
//         })
//     }
//
//     async fn manage_tasks(
//         self,
//         ws_to_channel_task: JoinHandle<anyhow::Result<()>>,
//         channel_to_ws_task: JoinHandle<anyhow::Result<()>>,
//     ) {
//         // Stop both tasks if one fails
//         tokio::select! {
//             result = ws_to_channel_task => {
//                 match result {
//                     Ok(Ok(())) => println!("reader completed successfully"),
//                     Ok(Err(e)) => eprintln!("reader failed with error: {}", e),
//                     Err(join_err) => eprintln!("reader task panicked or was cancelled: {}", join_err),
//                 }
//             },
//             result = channel_to_ws_task => {
//                 match result {
//                     Ok(Ok(())) => println!("writer completed successfully"),
//                     Ok(Err(e)) => eprintln!("writer failed with error: {}", e),
//                     Err(join_err) => eprintln!("writer task panicked or was cancelled: {}", join_err),
//                 }
//             },
//         }
//         self.is_connected.store(false, Ordering::SeqCst);
//     }
// }

/////////////////////////
// use tokio::task::JoinHandle;
//
// use tokio::net::{TcpListener, TcpStream};
// use tokio::sync::mpsc;
// use tokio_tungstenite::WebSocketStream;
// use tokio_tungstenite::accept_async;
// // use tokio_tungstenite::tungstenite::{Message, Utf8Bytes};
//
// use std::sync::{
//     Arc,
//     atomic::{AtomicBool, Ordering},
// };
//
// use crate::websocket::channel_to_ws;
// use crate::websocket::ws_to_channel;
use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::{Mutex, mpsc},
};
use tokio_tungstenite::tungstenite::Utf8Bytes;
use tokio_tungstenite::{WebSocketStream, accept_async, tungstenite::Message};

use crate::websocket::messages::{InboundMessage, OutboundMessage};

pub struct WebSocketServer {
    has_connection: Arc<AtomicBool>,
    inbound_tx: mpsc::Sender<InboundMessage>,
    outbound_rx: mpsc::Receiver<OutboundMessage>,
    session_tx: Arc<Mutex<Option<mpsc::Sender<OutboundMessage>>>>,
}

impl WebSocketServer {
    pub fn new(
        inbound_tx: mpsc::Sender<InboundMessage>,
        outbound_rx: mpsc::Receiver<OutboundMessage>,
    ) -> Self {
        Self {
            has_connection: Arc::new(AtomicBool::new(false)),
            inbound_tx,
            outbound_rx,
            session_tx: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn run(mut self) -> Result<()> {
        let websocket_port = "64057";
        let address = format!("127.0.0.1:{}", websocket_port);
        let listener = TcpListener::bind(&address).await?;
        println!("WebSocket server listening on ws://{}", address);

        // Forward global outbound_rx to active session
        let session_tx = self.session_tx.clone();
        tokio::spawn(async move {
            while let Some(msg) = self.outbound_rx.recv().await {
                if let Some(tx) = &*session_tx.lock().await {
                    tx.send(msg).await.ok();
                }
            }
        });

        loop {
            if self.has_connection.load(Ordering::SeqCst) {
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                continue;
            }

            let (stream, _) = listener.accept().await?;
            let ws_stream = accept_async(stream).await?;
            println!("Client connected");

            self.has_connection.store(true, Ordering::SeqCst);

            let (session_tx, session_rx) = mpsc::channel(32);
            *self.session_tx.lock().await = Some(session_tx);
            let inbound_tx = self.inbound_tx.clone();
            let has_connection = self.has_connection.clone();

            tokio::spawn(async move {
                if let Err(e) = WebSocketSession::start(ws_stream, inbound_tx, session_rx).await {
                    eprintln!("Session error: {:?}", e);
                }
                *session_tx.lock().await = None;
                has_connection.store(false, Ordering::SeqCst);
                println!("Client disconnected");
            });
        }
    }
}

pub struct WebSocketSession;

impl WebSocketSession {
    pub async fn start(
        ws_stream: WebSocketStream<TcpStream>,
        inbound_tx: mpsc::Sender<InboundMessage>,
        mut session_rx: mpsc::Receiver<OutboundMessage>,
    ) -> Result<()> {
        let (mut ws_writer, mut ws_reader) = ws_stream.split();

        let inbound = tokio::spawn(async move {
            while let Some(msg) = ws_reader.next().await {
                let msg = msg?; // Error handling: if WebSocket errors, return early

                if msg.is_text() {
                    let text = msg.into_text()?;
                    println!("recieved: {}", text);
                    let inbound: InboundMessage = serde_json::from_str(&text)?;

                    inbound_tx.send(inbound).await?;
                }
            }

            Ok::<_, anyhow::Error>(())
        });

        let outbound = tokio::spawn(async move {
            while let Some(outbound) = session_rx.recv().await {
                let json = serde_json::to_string(&outbound)?;
                let bytes = Utf8Bytes::from(json);

                ws_writer.send(Message::Text(bytes)).await?
            }

            Ok::<_, anyhow::Error>(())
        });

        let _ = tokio::try_join!(inbound, outbound);
        Ok(())
    }
}
