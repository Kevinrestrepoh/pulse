use axum::Json;
use serde_json::json;

pub mod events;
pub mod metrics;

pub async fn health() -> Json<serde_json::Value> {
    Json(json!({ "message": "ok" }))
}
