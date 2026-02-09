use crate::{models::event::Event, ws::wshub::WsHub};
use tokio::sync::{broadcast, mpsc};

#[derive(Clone)]
pub struct Broker {
    sender: mpsc::Sender<Event>,
}

impl Broker {
    pub fn new(
        buffer: usize,
        hub: WsHub,
        shutdown: broadcast::Receiver<()>,
    ) -> (Self, BrokerWorker) {
        let (tx, rx) = mpsc::channel(buffer);
        let broker = Broker { sender: tx };
        let worker = BrokerWorker {
            receiver: rx,
            hub,
            shutdown,
        };
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
    shutdown: broadcast::Receiver<()>,
}

impl BrokerWorker {
    pub async fn run(mut self) {
        loop {
            tokio::select! {
                maybe_event = self.receiver.recv() => {
                    if let Some(event) = maybe_event {
                        self.hub.publish(event).await;
                    } else {
                        break;
                    }
                }
                _ = self.shutdown.recv() => {
                    tracing::info!("Broker worker shutting down");
                    break;
                }
            }
        }

        tracing::info!("Broker worker shutting down");
    }
}
