use std::net::SocketAddr;

use axum::{
    Router,
    routing::{get, post},
};
use tracing_subscriber;

use crate::ws::wshub::WsHub;

mod api;
mod broker;
mod metrics;
mod models;
mod ws;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let addr: SocketAddr = "0.0.0.0:8080".parse()?;

    let hub = WsHub::new();
    let (broker, worker) = broker::broker::Broker::new(1024, hub.clone());
    tokio::spawn(async move {
        worker.run().await;
    });

    let app = Router::new()
        .route("/health", get(api::health))
        .route(
            "/events",
            post({
                let broker = broker.clone();
                move |payload| api::events::ingest_event(payload, broker)
            }),
        )
        .route(
            "/ws",
            get({
                let hub = hub.clone();
                move |ws, query| ws::handler::ws_handler(ws, query, hub)
            }),
        );

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::info!("Server listening on port {}", addr);

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
