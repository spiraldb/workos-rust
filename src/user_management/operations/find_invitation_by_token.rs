use async_trait::async_trait;
use serde::Serialize;
use thiserror::Error;

use crate::user_management::types::Invitation;
use crate::{ResponseExt, WorkOsError, WorkOsResult};

/// The parameters for [`FindInvitationByToken`].
#[derive(Debug, Serialize)]
pub struct FindInvitationByTokenParams<'a> {
    /// The token of the invitation to retrieve.
    pub token: &'a str,
}

/// An error returned from [`FindInvitationByToken`].
#[derive(Debug, Error)]
pub enum FindInvitationByTokenError {}

impl From<FindInvitationByTokenError> for WorkOsError<FindInvitationByTokenError> {
    fn from(err: FindInvitationByTokenError) -> Self {
        Self::Operation(err)
    }
}

/// [WorkOS Docs: Find an Invitation by Token](https://workos.com/docs/reference/user-management/invitation/find-by-token)
#[async_trait]
pub trait FindInvitationByToken {
    /// Finds an invitation by its token value.
    ///
    /// [WorkOS Docs: Find an Invitation by Token](https://workos.com/docs/reference/user-management/invitation/find-by-token)
    async fn find_invitation_by_token(
        &self,
        params: &FindInvitationByTokenParams<'_>,
    ) -> WorkOsResult<Invitation, FindInvitationByTokenError>;
}

#[async_trait]
impl FindInvitationByToken for crate::user_management::UserManagement<'_> {
    async fn find_invitation_by_token(
        &self,
        params: &FindInvitationByTokenParams<'_>,
    ) -> WorkOsResult<Invitation, FindInvitationByTokenError> {
        let url = self.workos.base_url().join(&format!(
            "/user_management/invitations/by_token/{}",
            params.token
        ))?;
        let invitation = self
            .workos
            .client()
            .get(url)
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
    use crate::user_management::InvitationId;
    use crate::{ApiKey, WorkOs};

    #[tokio::test]
    async fn it_calls_the_find_invitation_by_token_endpoint() {
        let mut server = mockito::Server::new_async().await;
        let workos = WorkOs::builder(&ApiKey::from("sk_example_123456789"))
            .base_url(&server.url())
            .unwrap()
            .build();

        let _mock = server
            .mock("GET", "/user_management/invitations/by_token/secret_token_abc")
            .match_header("Authorization", "Bearer sk_example_123456789")
            .with_status(200)
            .with_body(
                json!({
                    "id": "inv_01EHZNVPK3SFK441A1RGBFSHRT",
                    "object": "invitation",
                    "email": "user@example.com",
                    "state": "pending",
                    "organization_id": null,
                    "inviter_user_id": null,
                    "token": "secret_token_abc",
                    "accept_invitation_url": "https://workos.com/invitations/inv_01EHZNVPK3SFK441A1RGBFSHRT",
                    "accepted_at": null,
                    "revoked_at": null,
                    "expires_at": "2021-06-25T19:07:33.155Z",
                    "created_at": "2021-06-25T19:07:33.155Z",
                    "updated_at": "2021-06-25T19:07:33.155Z"
                })
                .to_string(),
            )
            .create_async()
            .await;

        let invitation = workos
            .user_management()
            .find_invitation_by_token(&FindInvitationByTokenParams {
                token: "secret_token_abc",
            })
            .await
            .unwrap();

        assert_eq!(
            invitation.id,
            InvitationId::from("inv_01EHZNVPK3SFK441A1RGBFSHRT")
        );
    }
}
