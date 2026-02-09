use crate::{broker::broker::Broker, metrics::metrics::Metrics, models::event::Event};
use axum::Json;
use serde_json::json;

pub async fn ingest_event(
    Json(event): Json<Event>,
    broker: Broker,
    metrics: Metrics,
) -> Json<serde_json::Value> {
    metrics.inc_received();

    let topic = event.payload.route_topic();
    broker.publish(topic, event).await;
    Json(json!({ "message": "event created" }))
}
