use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditMetadata {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: String,
    pub updated_by: String,
}

impl AuditMetadata {
    pub fn new(user: impl Into<String>) -> Self {
        let now = Utc::now();
        let u = user.into();
        Self {
            created_at: now,
            updated_at: now,
            created_by: u.clone(),
            updated_by: u,
        }
    }

    pub fn touch(&mut self, user: impl Into<String>) {
        self.updated_at = Utc::now();
        self.updated_by = user.into();
    }
}
