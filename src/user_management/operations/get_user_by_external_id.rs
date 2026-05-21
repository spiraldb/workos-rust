use async_trait::async_trait;
use serde::Serialize;
use thiserror::Error;

use crate::user_management::types::User;
use crate::{ResponseExt, WorkOsError, WorkOsResult};

/// The parameters for [`GetUserByExternalId`].
#[derive(Debug, Serialize)]
pub struct GetUserByExternalIdParams<'a> {
    /// The external ID of the user to retrieve.
    pub external_id: &'a str,
}

/// An error returned from [`GetUserByExternalId`].
#[derive(Debug, Error)]
pub enum GetUserByExternalIdError {}

impl From<GetUserByExternalIdError> for WorkOsError<GetUserByExternalIdError> {
    fn from(err: GetUserByExternalIdError) -> Self {
        Self::Operation(err)
    }
}

/// [WorkOS Docs: Get a User by External ID](https://workos.com/docs/reference/user-management/user/get-by-external-id)
#[async_trait]
pub trait GetUserByExternalId {
    /// Gets a user by their external ID.
    ///
    /// [WorkOS Docs: Get a User by External ID](https://workos.com/docs/reference/user-management/user/get-by-external-id)
    async fn get_user_by_external_id(
        &self,
        params: &GetUserByExternalIdParams<'_>,
    ) -> WorkOsResult<User, GetUserByExternalIdError>;
}

#[async_trait]
impl GetUserByExternalId for crate::user_management::UserManagement<'_> {
    async fn get_user_by_external_id(
        &self,
        params: &GetUserByExternalIdParams<'_>,
    ) -> WorkOsResult<User, GetUserByExternalIdError> {
        let url = self.workos.base_url().join(&format!(
            "/user_management/users/external_id/{}",
            params.external_id
        ))?;
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
    use crate::user_management::UserId;
    use crate::{ApiKey, WorkOs};

    #[tokio::test]
    async fn it_calls_the_get_user_by_external_id_endpoint() {
        let mut server = mockito::Server::new_async().await;
        let workos = WorkOs::builder(&ApiKey::from("sk_example_123456789"))
            .base_url(&server.url())
            .unwrap()
            .build();

        let _mock = server
            .mock(
                "GET",
                "/user_management/users/external_id/ext_user_123",
            )
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
                    "external_id": "ext_user_123",
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
            .get_user_by_external_id(&GetUserByExternalIdParams {
                external_id: "ext_user_123",
            })
            .await
            .unwrap();

        assert_eq!(user.id, UserId::from("user_01EHZNVPK3SFK441A1RGBFSHRT"));
    }
}
