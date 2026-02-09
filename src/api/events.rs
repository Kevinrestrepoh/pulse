use crate::{broker::broker::Broker, models::event::Event};
use axum::Json;
use serde_json::json;

pub async fn ingest_event(Json(event): Json<Event>, broker: Broker) -> Json<serde_json::Value> {
    tracing::info!("Received event: {:?}", event);
    broker.publish(event).await;
    Json(json!({ "message": "event created" }))
}
