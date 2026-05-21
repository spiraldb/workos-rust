use async_trait::async_trait;
use serde::Serialize;
use thiserror::Error;

use crate::user_management::types::UserId;
use crate::{ResponseExt, WorkOsError, WorkOsResult};

/// The parameters for [`DeleteUser`].
#[derive(Debug, Serialize)]
pub struct DeleteUserParams<'a> {
    /// The ID of the user to delete.
    pub user_id: &'a UserId,
}

/// An error returned from [`DeleteUser`].
#[derive(Debug, Error)]
pub enum DeleteUserError {}

impl From<DeleteUserError> for WorkOsError<DeleteUserError> {
    fn from(err: DeleteUserError) -> Self {
        Self::Operation(err)
    }
}

/// [WorkOS Docs: Delete a User](https://workos.com/docs/reference/user-management/user/delete)
#[async_trait]
pub trait DeleteUser {
    /// Deletes a user.
    ///
    /// [WorkOS Docs: Delete a User](https://workos.com/docs/reference/user-management/user/delete)
    async fn delete_user(&self, params: &DeleteUserParams<'_>) -> WorkOsResult<(), DeleteUserError>;
}

#[async_trait]
impl DeleteUser for crate::user_management::UserManagement<'_> {
    async fn delete_user(
        &self,
        params: &DeleteUserParams<'_>,
    ) -> WorkOsResult<(), DeleteUserError> {
        let url = self
            .workos
            .base_url()
            .join(&format!("/user_management/users/{}", params.user_id))?;
        self.workos
            .client()
            .delete(url)
            .bearer_auth(self.workos.key())
            .send()
            .await?
            .handle_unauthorized_or_generic_error()?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use tokio;

    use super::*;
    use crate::{ApiKey, WorkOs};

    #[tokio::test]
    async fn it_calls_the_delete_user_endpoint() {
        let mut server = mockito::Server::new_async().await;
        let workos = WorkOs::builder(&ApiKey::from("sk_example_123456789"))
            .base_url(&server.url())
            .unwrap()
            .build();

        let _mock = server
            .mock(
                "DELETE",
                "/user_management/users/user_01EHZNVPK3SFK441A1RGBFSHRT",
            )
            .match_header("Authorization", "Bearer sk_example_123456789")
            .with_status(204)
            .create_async()
            .await;

        workos
            .user_management()
            .delete_user(&DeleteUserParams {
                user_id: &UserId::from("user_01EHZNVPK3SFK441A1RGBFSHRT"),
            })
            .await
            .unwrap();
    }
}
