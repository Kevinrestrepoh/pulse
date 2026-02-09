use std::{env, sync::Arc, thread::sleep, time::Duration};

use dotenv::dotenv;
use rand::{RngExt, seq::IndexedRandom};
use reqwest::Client;
use tracing::info;

use crate::{
    generators::{generate_notification, generate_post, generate_system_alert},
    models::Event,
};

mod generators;
mod models;
mod ws_client;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();
    let client = Client::new();

    let user_count = 1000;
    let total_events = 100;
    let barrier = Arc::new(tokio::sync::Barrier::new(user_count + 1));

    let users: Vec<String> = (0..user_count).map(|i| format!("user-{}", i)).collect();

    for user in users.clone() {
        let ws_url = env::var("WS_URL").expect("WS_URL must be set");
        let barrier = barrier.clone();
        tokio::spawn(async move {
            ws_client::connect_feed(&ws_url, &format!("feed:{}", user), barrier).await;
        });
        tokio::time::sleep(Duration::from_millis(5)).await;
    }

    for _ in 0..total_events {
        let mut rng = rand::rng();

        let roll: u8 = rng.random_range(0..100);

        let event: Event = if roll < 70 {
            let author = users.choose(&mut rng).unwrap();
            generate_post(author)
        } else if roll < 90 {
            let user = users.choose(&mut rng).unwrap();
            generate_notification(user)
        } else {
            generate_system_alert()
        };

        info!("Sending event: {:?}", event.payload);

        let _ = client
            .post(format!(
                "{}/events",
                env::var("API_URL").expect("API_URL must be set")
            ))
            .json(&event)
            .send()
            .await;

        sleep(Duration::from_millis(200));
    }

    tokio::time::sleep(Duration::from_secs(1)).await;

    let metrics_url = format!(
        "{}/metrics",
        env::var("API_URL").expect("API_URL must be set")
    );

    match client.get(metrics_url).send().await {
        Ok(resp) => {
            let body = resp.text().await.unwrap_or_default();
            info!("Final metrics snapshot:\n{}", body);
        }
        Err(e) => {
            tracing::error!("Failed to fetch metrics: {}", e);
        }
    }
}
