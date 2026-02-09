use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventPayload {
    PostCreated {
        post_id: String,
        author_id: String,
        text: String,
    },
    Notification {
        user_id: String,
        message: String,
    },
    SystemAlert {
        level: String,
        message: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub event_id: Uuid,
    pub topic: String,
    pub payload: EventPayload,
    pub timestamp_ms: u64,
}
