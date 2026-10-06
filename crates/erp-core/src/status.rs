use serde::{Deserialize, Serialize};
use std::fmt;

/// Standard document status mirroring ERPNext DocStatus:
/// 0: Draft, 1: Submitted, 2: Cancelled
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(i8)]
pub enum DocStatus {
    Draft = 0,
    Submitted = 1,
    Cancelled = 2,
}

impl DocStatus {
    pub fn is_draft(&self) -> bool {
        matches!(self, DocStatus::Draft)
    }

    pub fn is_submitted(&self) -> bool {
        matches!(self, DocStatus::Submitted)
    }

    pub fn is_cancelled(&self) -> bool {
        matches!(self, DocStatus::Cancelled)
    }

    pub fn as_i64(&self) -> i64 {
        *self as i64
    }

    pub fn from_i64(val: i64) -> Option<Self> {
        match val {
            0 => Some(DocStatus::Draft),
            1 => Some(DocStatus::Submitted),
            2 => Some(DocStatus::Cancelled),
            _ => None,
        }
    }
}

impl fmt::Display for DocStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DocStatus::Draft => write!(f, "Draft"),
            DocStatus::Submitted => write!(f, "Submitted"),
            DocStatus::Cancelled => write!(f, "Cancelled"),
        }
    }
}

// Typestate pattern markers
pub trait DocState: Send + Sync + 'static {
    fn status() -> DocStatus;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct DraftState;
impl DocState for DraftState {
    fn status() -> DocStatus {
        DocStatus::Draft
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SubmittedState;
impl DocState for SubmittedState {
    fn status() -> DocStatus {
        DocStatus::Submitted
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CancelledState;
impl DocState for CancelledState {
    fn status() -> DocStatus {
        DocStatus::Cancelled
    }
}
