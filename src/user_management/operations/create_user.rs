use async_trait::async_trait;
use serde::Serialize;
use thiserror::Error;

use crate::user_management::types::User;
use crate::{ResponseExt, WorkOsError, WorkOsResult};

/// The parameters for [`CreateUser`].
#[derive(Debug, Serialize)]
pub struct CreateUserParams<'a> {
    /// The email address of the user.
    pub email: &'a str,

    /// The password of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<&'a str>,

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

    /// A pre-hashed password for the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_hash: Option<&'a str>,

    /// The algorithm used to hash the password (e.g. "bcrypt").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_hash_type: Option<&'a str>,
}

/// An error returned from [`CreateUser`].
#[derive(Debug, Error)]
pub enum CreateUserError {}

impl From<CreateUserError> for WorkOsError<CreateUserError> {
    fn from(err: CreateUserError) -> Self {
        Self::Operation(err)
    }
}

/// [WorkOS Docs: Create a User](https://workos.com/docs/reference/user-management/user/create)
#[async_trait]
pub trait CreateUser {
    /// Creates a user.
    ///
    /// [WorkOS Docs: Create a User](https://workos.com/docs/reference/user-management/user/create)
    async fn create_user(
        &self,
        params: &CreateUserParams<'_>,
    ) -> WorkOsResult<User, CreateUserError>;
}

#[async_trait]
impl CreateUser for crate::user_management::UserManagement<'_> {
    async fn create_user(
        &self,
        params: &CreateUserParams<'_>,
    ) -> WorkOsResult<User, CreateUserError> {
        let url = self.workos.base_url().join("/user_management/users")?;
        let user = self
            .workos
            .client()
            .post(url)
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
    use crate::user_management::UserId;
    use crate::{ApiKey, WorkOs};

    #[tokio::test]
    async fn it_calls_the_create_user_endpoint() {
        let mut server = mockito::Server::new_async().await;
        let workos = WorkOs::builder(&ApiKey::from("sk_example_123456789"))
            .base_url(&server.url())
            .unwrap()
            .build();

        let _mock = server
            .mock("POST", "/user_management/users")
            .match_header("Authorization", "Bearer sk_example_123456789")
            .with_status(201)
            .with_body(
                json!({
                    "id": "user_01EHZNVPK3SFK441A1RGBFSHRT",
                    "object": "user",
                    "email": "user@example.com",
                    "first_name": "Jane",
                    "last_name": "Doe",
                    "email_verified": false,
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
            .create_user(&CreateUserParams {
                email: "user@example.com",
                password: None,
                first_name: Some("Jane"),
                last_name: Some("Doe"),
                email_verified: None,
                external_id: None,
                metadata: None,
                password_hash: None,
                password_hash_type: None,
            })
            .await
            .unwrap();

        assert_eq!(user.id, UserId::from("user_01EHZNVPK3SFK441A1RGBFSHRT"));
    }
}
