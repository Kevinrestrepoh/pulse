use crate::{models::event::Event, ws::wshub::WsHub};
use tokio::sync::mpsc;

#[derive(Clone)]
pub struct Broker {
    sender: mpsc::Sender<Event>,
}

impl Broker {
    pub fn new(buffer: usize, hub: WsHub) -> (Self, BrokerWorker) {
        let (tx, rx) = mpsc::channel(buffer);
        let broker = Broker { sender: tx };
        let worker = BrokerWorker { receiver: rx, hub };
        (broker, worker)
    }

    pub async fn publish(&self, event: Event) {
        if let Err(e) = self.sender.send(event).await {
            tracing::error!("Failed to publish event to broker: {}", e);
        }
    }
}

pub struct BrokerWorker {
    receiver: mpsc::Receiver<Event>,
    hub: WsHub,
}

impl BrokerWorker {
    pub async fn run(mut self) {
        while let Some(event) = self.receiver.recv().await {
            tracing::info!("Broker routing event: {}", event.topic);
            self.hub.publish(event).await;
        }

        tracing::info!("Broker worker shutting down");
    }
}
