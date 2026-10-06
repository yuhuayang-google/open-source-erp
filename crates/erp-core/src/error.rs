use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum CoreError {
    #[error("Currency mismatch: expected {expected}, found {actual}")]
    CurrencyMismatch { expected: String, actual: String },

    #[error("Invalid amount: {0}")]
    InvalidAmount(String),

    #[error("Document state transition error: {0}")]
    InvalidStateTransition(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Entity not found: {entity} with ID {id}")]
    NotFound { entity: String, id: String },

    #[error("Duplicate entity: {entity} with ID {id}")]
    Duplicate { entity: String, id: String },
}

pub type DomainResult<T> = Result<T, CoreError>;
