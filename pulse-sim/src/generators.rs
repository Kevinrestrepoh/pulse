use crate::models::{Event, EventPayload};
use chrono::Utc;
use rand::seq::IndexedRandom;
use uuid::Uuid;

pub fn generate_post(author_id: &str) -> Event {
    Event {
        event_id: Uuid::new_v4(),
        timestamp_ms: Utc::now().timestamp_millis() as u64,
        payload: EventPayload::PostCreated {
            post_id: Uuid::new_v4().to_string(),
            author_id: author_id.to_string(),
            text: "hello from pulse-sim".into(),
        },
    }
}

pub fn generate_notification(user_id: &str) -> Event {
    Event {
        event_id: Uuid::new_v4(),
        timestamp_ms: Utc::now().timestamp_millis() as u64,
        payload: EventPayload::Notification {
            user_id: user_id.to_string(),
            message: "you have a new follower".into(),
        },
    }
}

pub fn generate_system_alert() -> Event {
    let levels = ["info", "warning", "critical"];
    let level = levels
        .choose(&mut rand::rng())
        .expect("levels array should not be empty");

    Event {
        event_id: Uuid::new_v4(),
        timestamp_ms: Utc::now().timestamp_millis() as u64,
        payload: EventPayload::SystemAlert {
            level: level.to_string(),
            message: "simulated system alert".into(),
        },
    }
}
