pub mod envelope;

pub use envelope::EventEnvelope;
use async_trait::async_trait;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EventError {
    #[error("Failed to publish event: {0}")]
    PublishFailed(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

#[async_trait]
pub trait EventPublisher: Send + Sync {
    async fn publish<T: serde::Serialize + Send + Sync>(
        &self,
        envelope: &EventEnvelope<T>,
    ) -> Result<(), EventError>;
}
