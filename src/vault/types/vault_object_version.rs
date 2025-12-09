use serde::{Deserialize, Serialize};

use crate::Timestamp;

/// Represents a static version of an object stored by Vault.
///
/// [WorkOS Docs: Object Version](https://workos.com/docs/reference/vault)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VaultObjectVersion {
    /// The version identifier.
    pub id: String,

    /// The timestamp when this version was created.
    pub created_at: Timestamp,

    /// Whether this is the current version.
    pub current_version: bool,

    /// The ETag for this version.
    pub etag: String,

    /// The size of the object in bytes.
    pub size: u64,
}
