use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// A unique identifier for a [VaultObject](crate::vault::VaultObject).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct VaultObjectId(String);

impl Display for VaultObjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for VaultObjectId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for VaultObjectId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}
