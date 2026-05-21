use async_trait::async_trait;
use serde::Serialize;

use crate::organizations::OrganizationId;
use crate::user_management::types::Invitation;
use crate::{PaginatedList, PaginationParams, ResponseExt, WorkOsResult};

/// The parameters for [`ListInvitations`].
#[derive(Debug, Default, Serialize)]
pub struct ListInvitationsParams<'a> {
    /// Filter by organization ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<&'a OrganizationId>,

    /// Filter by recipient email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,

    /// The pagination parameters to use when listing invitations.
    #[serde(flatten)]
    pub pagination: PaginationParams<'a>,
}

/// [WorkOS Docs: List Invitations](https://workos.com/docs/reference/user-management/invitation/list)
#[async_trait]
pub trait ListInvitations {
    /// Lists invitations.
    ///
    /// [WorkOS Docs: List Invitations](https://workos.com/docs/reference/user-management/invitation/list)
    async fn list_invitations(
        &self,
        params: &ListInvitationsParams<'_>,
    ) -> WorkOsResult<PaginatedList<Invitation>, ()>;
}

#[async_trait]
impl ListInvitations for crate::user_management::UserManagement<'_> {
    async fn list_invitations(
        &self,
        params: &ListInvitationsParams<'_>,
    ) -> WorkOsResult<PaginatedList<Invitation>, ()> {
        let url = self
            .workos
            .base_url()
            .join("/user_management/invitations")?;
        let invitations = self
            .workos
            .client()
            .get(url)
            .bearer_auth(self.workos.key())
            .query(&params)
            .send()
            .await?
            .handle_unauthorized_or_generic_error()?
            .json::<PaginatedList<Invitation>>()
            .await?;

        Ok(invitations)
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
    async fn it_calls_the_list_invitations_endpoint() {
        let mut server = mockito::Server::new_async().await;
        let workos = WorkOs::builder(&ApiKey::from("sk_example_123456789"))
            .base_url(&server.url())
            .unwrap()
            .build();

        let _mock = server
            .mock("GET", "/user_management/invitations")
            .match_header("Authorization", "Bearer sk_example_123456789")
            .with_status(200)
            .with_body(
                json!({
                    "data": [
                        {
                            "id": "inv_01EHZNVPK3SFK441A1RGBFSHRT",
                            "object": "invitation",
                            "email": "user@example.com",
                            "state": "pending",
                            "organization_id": null,
                            "inviter_user_id": null,
                            "token": "some_token",
                            "accept_invitation_url": "https://workos.com/invitations/inv_01EHZNVPK3SFK441A1RGBFSHRT",
                            "accepted_at": null,
                            "revoked_at": null,
                            "expires_at": "2021-06-25T19:07:33.155Z",
                            "created_at": "2021-06-25T19:07:33.155Z",
                            "updated_at": "2021-06-25T19:07:33.155Z"
                        }
                    ],
                    "list_metadata": {
                        "before": null,
                        "after": null
                    }
                })
                .to_string(),
            )
            .create_async()
            .await;

        let paginated_list = workos
            .user_management()
            .list_invitations(&Default::default())
            .await
            .unwrap();

        assert_eq!(paginated_list.data.len(), 1);
        assert_eq!(
            paginated_list.data[0].id,
            InvitationId::from("inv_01EHZNVPK3SFK441A1RGBFSHRT")
        );
    }
}
