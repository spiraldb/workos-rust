use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::organizations::OrganizationId;
use crate::user_management::types::user::UserId;
use crate::{Timestamp, Timestamps};

/// The ID of an [`Invitation`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct InvitationId(String);

impl Display for InvitationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for InvitationId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for InvitationId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InvitationState {
    /// The invitation is pending.
    Pending,

    /// The invitation has been accepted.
    Accepted,

    /// The invitation has been revoked.
    Revoked,

    /// The invitation has expired.
    Expired,
}

/// [WorkOS Docs: Invitation](https://workos.com/docs/reference/user-management/invitation)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Invitation {
    /// The ID of the invitation.
    pub id: InvitationId,

    /// The name of the organization.
    pub email: String,

    pub state: InvitationState,
    pub organization_id: OrganizationId,
    pub inviter_user_id: UserId,
    pub token: String,
    pub accept_invitation_url: String,


    pub accepted_at: Option<Timestamp>,
    pub revoked_at: Option<Timestamp>,
    pub expires_at: Timestamp,

    /// The timestamps for the organization.
    #[serde(flatten)]
    pub timestamps: Timestamps,
}
