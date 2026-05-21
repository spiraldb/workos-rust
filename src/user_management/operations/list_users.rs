use async_trait::async_trait;
use serde::Serialize;

use crate::organizations::OrganizationId;
use crate::user_management::types::User;
use crate::{PaginatedList, PaginationParams, ResponseExt, WorkOsResult};

/// The parameters for [`ListUsers`].
#[derive(Debug, Default, Serialize)]
pub struct ListUsersParams<'a> {
    /// Filter by organization ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<&'a OrganizationId>,

    /// Filter by email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,

    /// The pagination parameters to use when listing users.
    #[serde(flatten)]
    pub pagination: PaginationParams<'a>,
}

/// [WorkOS Docs: List Users](https://workos.com/docs/reference/user-management/user/list)
#[async_trait]
pub trait ListUsers {
    /// Lists users.
    ///
    /// [WorkOS Docs: List Users](https://workos.com/docs/reference/user-management/user/list)
    async fn list_users(
        &self,
        params: &ListUsersParams<'_>,
    ) -> WorkOsResult<PaginatedList<User>, ()>;
}

#[async_trait]
impl ListUsers for crate::user_management::UserManagement<'_> {
    async fn list_users(
        &self,
        params: &ListUsersParams<'_>,
    ) -> WorkOsResult<PaginatedList<User>, ()> {
        let url = self.workos.base_url().join("/user_management/users")?;
        let users = self
            .workos
            .client()
            .get(url)
            .bearer_auth(self.workos.key())
            .query(&params)
            .send()
            .await?
            .handle_unauthorized_or_generic_error()?
            .json::<PaginatedList<User>>()
            .await?;

        Ok(users)
    }
}

#[cfg(test)]
mod test {
    use mockito::Matcher;
    use serde_json::json;
    use tokio;

    use super::*;
    use crate::user_management::UserId;
    use crate::{ApiKey, WorkOs};

    #[tokio::test]
    async fn it_calls_the_list_users_endpoint() {
        let mut server = mockito::Server::new_async().await;
        let workos = WorkOs::builder(&ApiKey::from("sk_example_123456789"))
            .base_url(&server.url())
            .unwrap()
            .build();

        let _mock = server
            .mock("GET", "/user_management/users")
            .match_header("Authorization", "Bearer sk_example_123456789")
            .with_status(200)
            .with_body(
                json!({
                    "data": [
                        {
                            "id": "user_01EHZNVPK3SFK441A1RGBFSHRT",
                            "object": "user",
                            "email": "user@example.com",
                            "first_name": "Jane",
                            "last_name": "Doe",
                            "email_verified": true,
                            "profile_picture_url": null,
                            "last_sign_in_at": null,
                            "external_id": null,
                            "metadata": {},
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
            .list_users(&Default::default())
            .await
            .unwrap();

        assert_eq!(paginated_list.data.len(), 1);
        assert_eq!(
            paginated_list.data[0].id,
            UserId::from("user_01EHZNVPK3SFK441A1RGBFSHRT")
        );
    }

    #[tokio::test]
    async fn it_filters_by_email() {
        let mut server = mockito::Server::new_async().await;
        let workos = WorkOs::builder(&ApiKey::from("sk_example_123456789"))
            .base_url(&server.url())
            .unwrap()
            .build();

        let _mock = server
            .mock("GET", "/user_management/users")
            .match_query(Matcher::UrlEncoded(
                "email".to_string(),
                "user@example.com".to_string(),
            ))
            .match_header("Authorization", "Bearer sk_example_123456789")
            .with_status(200)
            .with_body(
                json!({
                    "data": [],
                    "list_metadata": {
                        "before": null,
                        "after": null
                    }
                })
                .to_string(),
            )
            .create_async()
            .await;

        workos
            .user_management()
            .list_users(&ListUsersParams {
                email: Some("user@example.com"),
                ..Default::default()
            })
            .await
            .unwrap();
    }
}
