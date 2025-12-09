use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use super::{VaultObjectId, VaultObjectUpdatedBy};
use crate::Timestamp;

/// Metadata for a [VaultObject](crate::vault::VaultObject).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VaultObjectMetadata {
    /// The unique identifier for the object.
    pub id: VaultObjectId,

    /// The environment ID.
    pub environment_id: String,

    /// The key context used for encryption.
    pub context: HashMap<String, Value>,

    /// The ID of the encryption key used.
    pub key_id: String,

    /// The timestamp when the object was last updated.
    pub updated_at: Timestamp,

    /// Information about who last updated the object.
    pub updated_by: VaultObjectUpdatedBy,

    /// The version identifier for the object.
    pub version_id: String,
}
