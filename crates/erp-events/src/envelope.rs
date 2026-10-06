use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope<T> {
    pub event_id: Uuid,
    pub event_type: String,
    pub company_id: Uuid,
    pub aggregate_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub payload: T,
}

impl<T> EventEnvelope<T> {
    pub fn new(event_type: impl Into<String>, company_id: Uuid, aggregate_id: Uuid, payload: T) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            event_type: event_type.into(),
            company_id,
            aggregate_id,
            timestamp: Utc::now(),
            payload,
        }
    }
}
