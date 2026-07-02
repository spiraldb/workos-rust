use async_trait::async_trait;
use serde::Serialize;
use thiserror::Error;

use crate::user_management::types::{User, UserId};
use crate::{ResponseExt, WorkOsError, WorkOsResult};

/// The parameters for [`UpdateUser`].
#[derive(Debug, Serialize)]
pub struct UpdateUserParams<'a> {
    /// The ID of the user to update.
    #[serde(skip_serializing)]
    pub user_id: &'a UserId,

    /// The first name of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<&'a str>,

    /// The last name of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<&'a str>,

    /// Whether the user's email address has been verified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verified: Option<bool>,

    /// The external ID of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<&'a str>,

    /// Additional metadata about the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// An error returned from [`UpdateUser`].
#[derive(Debug, Error)]
pub enum UpdateUserError {}

impl From<UpdateUserError> for WorkOsError<UpdateUserError> {
    fn from(err: UpdateUserError) -> Self {
        Self::Operation(err)
    }
}

/// [WorkOS Docs: Update a User](https://workos.com/docs/reference/user-management/user/update)
#[async_trait]
pub trait UpdateUser {
    /// Updates a user.
    ///
    /// [WorkOS Docs: Update a User](https://workos.com/docs/reference/user-management/user/update)
    async fn update_user(
        &self,
        params: &UpdateUserParams<'_>,
    ) -> WorkOsResult<User, UpdateUserError>;
}

#[async_trait]
impl UpdateUser for crate::user_management::UserManagement<'_> {
    async fn update_user(
        &self,
        params: &UpdateUserParams<'_>,
    ) -> WorkOsResult<User, UpdateUserError> {
        let url = self
            .workos
            .base_url()
            .join(&format!("/user_management/users/{}", params.user_id))?;
        let user = self
            .workos
            .client()
            .put(url)
            .bearer_auth(self.workos.key())
            .json(&params)
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
    async fn it_calls_the_update_user_endpoint() {
        let mut server = mockito::Server::new_async().await;
        let workos = WorkOs::builder(&ApiKey::from("sk_example_123456789"))
            .base_url(&server.url())
            .unwrap()
            .build();

        let _mock = server
            .mock(
                "PUT",
                "/user_management/users/user_01EHZNVPK3SFK441A1RGBFSHRT",
            )
            .match_header("Authorization", "Bearer sk_example_123456789")
            .with_status(200)
            .with_body(
                json!({
                    "id": "user_01EHZNVPK3SFK441A1RGBFSHRT",
                    "object": "user",
                    "email": "user@example.com",
                    "first_name": "Jane",
                    "last_name": "Smith",
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
            .update_user(&UpdateUserParams {
                user_id: &UserId::from("user_01EHZNVPK3SFK441A1RGBFSHRT"),
                first_name: None,
                last_name: Some("Smith"),
                email_verified: None,
                external_id: None,
                metadata: None,
            })
            .await
            .unwrap();

        assert_eq!(user.last_name.as_deref(), Some("Smith"));
    }
}
