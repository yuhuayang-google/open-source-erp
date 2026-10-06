use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Uom(pub String);

impl Uom {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    pub fn nos() -> Self {
        Self("Nos".into())
    }

    pub fn kg() -> Self {
        Self("Kg".into())
    }

    pub fn meter() -> Self {
        Self("Meter".into())
    }

    pub fn hour() -> Self {
        Self("Hour".into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Uom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T: Into<String>> From<T> for Uom {
    fn from(val: T) -> Self {
        Self(val.into())
    }
}
