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

#[derive(Clone, Copy)]
pub enum Priority {
    Critical,
    Normal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub event_id: Uuid,
    pub payload: EventPayload,
    pub timestamp_ms: u64,
}

impl EventPayload {
    pub fn priority(&self) -> Priority {
        match self {
            EventPayload::SystemAlert { .. } => Priority::Critical,
            _ => Priority::Normal,
        }
    }

    pub fn route_topic(&self) -> String {
        match self {
            EventPayload::PostCreated { author_id, .. } => {
                format!("feed:{}", author_id)
            }
            EventPayload::Notification { user_id, .. } => {
                format!("user:{}", user_id)
            }
            EventPayload::SystemAlert { level, .. } => {
                format!("alerts:{}", level.to_lowercase())
            }
        }
    }
}
