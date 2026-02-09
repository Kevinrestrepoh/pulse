use std::sync::Arc;

use futures_util::StreamExt;
use tokio::sync::Barrier;
use tokio_tungstenite::connect_async;
use tracing::info;

use crate::models::Event;

pub async fn connect_feed(url: &str, topic: &str, ready: Arc<Barrier>) {
    let url = format!("{}/ws?topic={}", url, topic);

    let (ws, _) = connect_async(url).await.expect("WS connect failed");
    info!("Connected to {}", topic);

    let (_, mut read) = ws.split();

    ready.wait().await;

    while let Some(msg) = read.next().await {
        if let Ok(msg) = msg {
            if let tokio_tungstenite::tungstenite::Message::Text(txt) = msg {
                if let Ok(events) = serde_json::from_str::<Vec<Event>>(&txt) {
                    tracing::info!(topic = %topic, count = events.len(), "Received batch");
                }
            }
        }
    }
}
