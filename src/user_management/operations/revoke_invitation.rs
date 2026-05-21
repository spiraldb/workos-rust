use async_trait::async_trait;
use serde::Serialize;
use thiserror::Error;

use crate::user_management::types::{Invitation, InvitationId};
use crate::{ResponseExt, WorkOsError, WorkOsResult};

/// The parameters for [`RevokeInvitation`].
#[derive(Debug, Serialize)]
pub struct RevokeInvitationParams<'a> {
    /// The ID of the invitation to revoke.
    pub invitation_id: &'a InvitationId,
}

/// An error returned from [`RevokeInvitation`].
#[derive(Debug, Error)]
pub enum RevokeInvitationError {}

impl From<RevokeInvitationError> for WorkOsError<RevokeInvitationError> {
    fn from(err: RevokeInvitationError) -> Self {
        Self::Operation(err)
    }
}

/// [WorkOS Docs: Revoke an Invitation](https://workos.com/docs/reference/user-management/invitation/revoke)
#[async_trait]
pub trait RevokeInvitation {
    /// Revokes an invitation.
    ///
    /// [WorkOS Docs: Revoke an Invitation](https://workos.com/docs/reference/user-management/invitation/revoke)
    async fn revoke_invitation(
        &self,
        params: &RevokeInvitationParams<'_>,
    ) -> WorkOsResult<Invitation, RevokeInvitationError>;
}

#[async_trait]
impl RevokeInvitation for crate::user_management::UserManagement<'_> {
    async fn revoke_invitation(
        &self,
        params: &RevokeInvitationParams<'_>,
    ) -> WorkOsResult<Invitation, RevokeInvitationError> {
        let url = self.workos.base_url().join(&format!(
            "/user_management/invitations/{}/revoke",
            params.invitation_id
        ))?;
        let invitation = self
            .workos
            .client()
            .post(url)
            .bearer_auth(self.workos.key())
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
    use serde_json::json;
    use tokio;

    use super::*;
    use crate::{ApiKey, WorkOs};

    #[tokio::test]
    async fn it_calls_the_revoke_invitation_endpoint() {
        let mut server = mockito::Server::new_async().await;
        let workos = WorkOs::builder(&ApiKey::from("sk_example_123456789"))
            .base_url(&server.url())
            .unwrap()
            .build();

        let _mock = server
            .mock(
                "POST",
                "/user_management/invitations/inv_01EHZNVPK3SFK441A1RGBFSHRT/revoke",
            )
            .match_header("Authorization", "Bearer sk_example_123456789")
            .with_status(200)
            .with_body(
                json!({
                    "id": "inv_01EHZNVPK3SFK441A1RGBFSHRT",
                    "object": "invitation",
                    "email": "user@example.com",
                    "state": "revoked",
                    "organization_id": null,
                    "inviter_user_id": null,
                    "token": "some_token",
                    "accept_invitation_url": "https://workos.com/invitations/inv_01EHZNVPK3SFK441A1RGBFSHRT",
                    "accepted_at": null,
                    "revoked_at": "2021-06-26T19:07:33.155Z",
                    "expires_at": "2021-06-25T19:07:33.155Z",
                    "created_at": "2021-06-25T19:07:33.155Z",
                    "updated_at": "2021-06-26T19:07:33.155Z"
                })
                .to_string(),
            )
            .create_async()
            .await;

        let invitation = workos
            .user_management()
            .revoke_invitation(&RevokeInvitationParams {
                invitation_id: &InvitationId::from("inv_01EHZNVPK3SFK441A1RGBFSHRT"),
            })
            .await
            .unwrap();

        assert_eq!(
            invitation.id,
            InvitationId::from("inv_01EHZNVPK3SFK441A1RGBFSHRT")
        );
    }
}
