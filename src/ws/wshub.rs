use crate::models::event::Event;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{RwLock, mpsc};

pub type SubscriberTx = mpsc::Sender<Event>;

#[derive(Clone, Default)]
pub struct WsHub {
    topics: Arc<RwLock<HashMap<String, Vec<SubscriberTx>>>>,
}

impl WsHub {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn subscribe(&self, topic: String, tx: SubscriberTx) {
        let mut topics = self.topics.write().await;
        topics.entry(topic).or_default().push(tx);
    }

    pub async fn publish(&self, event: Event) {
        let topics = self.topics.read().await;

        if let Some(subscribers) = topics.get(&event.topic) {
            for sub in subscribers {
                let _ = sub.send(event.clone()).await;
            }
        }
    }
}
