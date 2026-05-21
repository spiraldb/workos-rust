use async_trait::async_trait;
use serde::Serialize;
use thiserror::Error;

use crate::user_management::types::{User, UserId};
use crate::{ResponseExt, WorkOsError, WorkOsResult};

/// The parameters for [`GetUser`].
#[derive(Debug, Serialize)]
pub struct GetUserParams<'a> {
    /// The ID of the user to retrieve.
    pub user_id: &'a UserId,
}

/// An error returned from [`GetUser`].
#[derive(Debug, Error)]
pub enum GetUserError {}

impl From<GetUserError> for WorkOsError<GetUserError> {
    fn from(err: GetUserError) -> Self {
        Self::Operation(err)
    }
}

/// [WorkOS Docs: Get a User](https://workos.com/docs/reference/user-management/user/get)
#[async_trait]
pub trait GetUser {
    /// Gets a user.
    ///
    /// [WorkOS Docs: Get a User](https://workos.com/docs/reference/user-management/user/get)
    async fn get_user(&self, params: &GetUserParams<'_>) -> WorkOsResult<User, GetUserError>;
}

#[async_trait]
impl GetUser for crate::user_management::UserManagement<'_> {
    async fn get_user(&self, params: &GetUserParams<'_>) -> WorkOsResult<User, GetUserError> {
        let url = self
            .workos
            .base_url()
            .join(&format!("/user_management/users/{}", params.user_id))?;
        let user = self
            .workos
            .client()
            .get(url)
            .bearer_auth(self.workos.key())
            .send()
            .await?
            .handle_unauthorized_or_generic_error()?
            .json::<User>()
            .await?;

        Ok(user)
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;
    use tokio;

    use super::*;
    use crate::{ApiKey, WorkOs};

    #[tokio::test]
    async fn it_calls_the_get_user_endpoint() {
        let mut server = mockito::Server::new_async().await;
        let workos = WorkOs::builder(&ApiKey::from("sk_example_123456789"))
            .base_url(&server.url())
            .unwrap()
            .build();

        let _mock = server
            .mock("GET", "/user_management/users/user_01EHZNVPK3SFK441A1RGBFSHRT")
            .match_header("Authorization", "Bearer sk_example_123456789")
            .with_status(200)
            .with_body(
                json!({
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
                })
                .to_string(),
            )
            .create_async()
            .await;

        let user = workos
            .user_management()
            .get_user(&GetUserParams {
                user_id: &UserId::from("user_01EHZNVPK3SFK441A1RGBFSHRT"),
            })
            .await
            .unwrap();

        assert_eq!(user.id, UserId::from("user_01EHZNVPK3SFK441A1RGBFSHRT"));
    }
}
