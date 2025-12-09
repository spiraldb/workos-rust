use serde::{Deserialize, Serialize};

use super::{VaultObjectId, VaultObjectMetadata};

/// An encrypted object stored by Vault.
///
/// [WorkOS Docs: Vault Object](https://workos.com/docs/reference/vault)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VaultObject {
    /// The unique identifier for the object.
    pub id: VaultObjectId,

    /// The name of the secret.
    pub name: String,

    /// The decrypted value of the secret.
    pub value: String,

    /// Metadata about the object.
    pub metadata: VaultObjectMetadata,
}