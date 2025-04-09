use async_trait::async_trait;
use serde::Serialize;
use thiserror::Error;

use crate::organizations::OrganizationId;
use crate::user_management::types::{Invitation, UserId};
use crate::{ResponseExt, WorkOsError, WorkOsResult};

/// The parameters for [`SendInvitation`].
#[derive(Debug, Serialize)]
pub struct SendInvitationParams<'a> {
    /// The email address of the user to invite.
    pub email: &'a str,

    /// The ID of the organization to invite the user to.
    pub organization_id: Option<&'a OrganizationId>,

    /// The number of days until the invitation expires.
    pub expires_in_days: Option<&'a i32>,

    /// The ID of the user sending the invitation.
    pub inviter_user_id: Option<&'a UserId>,

    /// The role slug to assign to the user.
    pub role_slug: Option<&'a str>,
}

/// An error returned from [`SendInvitation`].
#[derive(Debug, Error)]
pub enum SendInvitationError {}

impl From<SendInvitationError> for WorkOsError<SendInvitationError> {
    fn from(err: SendInvitationError) -> Self {
        Self::Operation(err)
    }
}

/// [WorkOS Docs: Send an Invitation](https://workos.com/docs/reference/user-management/invitation/send)
#[async_trait]
pub trait SendInvitation {
    /// Sends an invitation to a user.
    ///
    /// [WorkOS Docs: Send an Invitation](https://workos.com/docs/reference/user-management/invitation/send)
    async fn send_invitation(
        &self,
        params: &SendInvitationParams<'_>,
    ) -> WorkOsResult<Invitation, SendInvitationError>;
}

#[async_trait]
impl SendInvitation for crate::user_management::UserManagement<'_> {
    async fn send_invitation(
        &self,
        params: &SendInvitationParams<'_>,
    ) -> WorkOsResult<Invitation, SendInvitationError> {
        let url = self
            .workos
            .base_url()
            .join("/user_management/invitations")?;
        let invitation = self
            .workos
            .client()
            .post(url)
            .bearer_auth(self.workos.key())
            .json(&params)
            .send()
            .await?
            .handle_unauthorized_or_generic_error()?
            .json::<Invitation>()
            .await?;

        Ok(invitation)
    }
}

#[cfg(test)]
mod test {
    use mockito::{self, mock};
    use serde_json::json;
    use tokio;

    use super::*;
    use crate::user_management::types::InvitationId;
    use crate::{ApiKey, WorkOs};

    #[tokio::test]
    async fn it_calls_the_send_invitation_endpoint() {
        let workos = WorkOs::builder(&ApiKey::from("sk_example_123456789"))
            .base_url(&mockito::server_url())
            .unwrap()
            .build();

        let _mock = mock("POST", "/user_management/invitations")
            .match_header("Authorization", "Bearer sk_example_123456789")
            .with_status(201)
            .with_body(
                json!({
                    "id": "inv_01EHZNVPK3SFK441A1RGBFSHRT",
                    "object": "invitation",
                    "email": "user@example.com",
                    "state": "pending",
                    "organization_id": "org_01EHZNVPK3SFK441A1RGBFSHRT",
                    "inviter_user_id": "user_01EHZNVPK3SFK441A1RGBFSHRT",
                    "token": "inv_01EHZNVPK3SFK441A1RGBFSHRT",
                    "accept_invitation_url": "https://workos.com/invitations/inv_01EHZNVPK3SFK441A1RGBFSHRT",
                    "accepted_at": null,
                    "revoked_at": null,
                    "expires_at": "2021-06-25T19:07:33.155Z",
                    "created_at": "2021-06-25T19:07:33.155Z",
                    "updated_at": "2021-06-25T19:07:33.155Z"
                })
                .to_string(),
            )
            .create();

        let invitation = workos
            .user_management()
            .send_invitation(&SendInvitationParams {
                email: "user@example.com",
                organization_id: Some(&OrganizationId::from("org_01EHZNVPK3SFK441A1RGBFSHRT")),
                expires_in_days: Some(&7),
                inviter_user_id: None,
                role_slug: None,
            })
            .await
            .unwrap();

        assert_eq!(
            invitation.id,
            InvitationId::from("inv_01EHZNVPK3SFK441A1RGBFSHRT")
        )
    }
}
