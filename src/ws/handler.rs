use crate::{models::event::Event, ws::wshub::WsHub};
use axum::{
    extract::{
        Query,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
};
use std::collections::HashMap;
use tokio::sync::mpsc;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Query(params): Query<HashMap<String, String>>,
    hub: WsHub,
) -> impl IntoResponse {
    let topic = params
        .get("topic")
        .cloned()
        .unwrap_or_else(|| "default".into());

    ws.on_upgrade(move |socket| handle_socket(socket, hub, topic))
}

async fn handle_socket(mut socket: WebSocket, hub: WsHub, topic: String) {
    let (tx, mut rx) = mpsc::channel::<Event>(32);

    hub.subscribe(topic.clone(), tx).await;
    tracing::info!("WebSocket client subscribed to {}", topic);

    while let Some(event) = rx.recv().await {
        let json = serde_json::to_string(&event).unwrap();
        if socket.send(Message::Text(Into::into(json))).await.is_err() {
            break;
        }
    }

    tracing::info!("WebSocket client disconnected from {}", topic);
}
