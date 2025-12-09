use serde::{Deserialize, Serialize};

/// Information about who last updated a vault object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VaultObjectUpdatedBy {
    /// The ID of the user who updated the object.
    pub id: String,

    /// The name of the user who updated the object.
    pub name: String,
}
