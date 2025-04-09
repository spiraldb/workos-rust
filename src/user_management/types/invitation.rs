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

/// The state of an [`Invitation`].
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

    /// The email address of the user being invited.
    pub email: String,

    /// The state of the invitation.
    pub state: InvitationState,

    /// The ID of the organization the user is being invited to.
    pub organization_id: OrganizationId,

    /// The ID of the user sending the invitation.
    pub inviter_user_id: UserId,

    /// The token used to accept the invitation.
    pub token: String,

    /// The URL used to accept the invitation.
    pub accept_invitation_url: String,

    /// The timestamp when the invitation was accepted.
    pub accepted_at: Option<Timestamp>,

    /// The timestamp when the invitation was revoked.
    pub revoked_at: Option<Timestamp>,

    /// The timestamp when the invitation expires.
    pub expires_at: Timestamp,

    /// The timestamps for the invitation.
    #[serde(flatten)]
    pub timestamps: Timestamps,
}
