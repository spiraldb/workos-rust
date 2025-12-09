use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::vault::{Vault, VaultObjectId};
use crate::{ResponseExt, WorkOsError, WorkOsResult};

/// The parameters for [`DeleteVaultObject`].
#[derive(Debug, Serialize)]
pub struct DeleteVaultObjectParams<'a> {
    /// The ID of the object to delete.
    #[serde(skip_serializing)]
    pub id: &'a VaultObjectId,

    /// Optional version check to ensure the delete is applied to the expected version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_check: Option<&'a str>,
}

/// Response from deleting a vault object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeleteVaultObjectResponse {
    /// Whether the deletion was successful.
    pub success: bool,

    /// The name of the deleted object.
    pub name: String,
}

/// The error returned from the [`DeleteVaultObject`] trait.
#[derive(Debug, Error)]
pub enum DeleteVaultObjectError {}

impl From<DeleteVaultObjectError> for WorkOsError<DeleteVaultObjectError> {
    fn from(err: DeleteVaultObjectError) -> Self {
        Self::Operation(err)
    }
}

/// [WorkOS Docs: Delete an object](https://workos.com/docs/reference/vault/delete-object)
#[async_trait]
pub trait DeleteVaultObject {
    /// Permanently deletes an object.
    ///
    /// [WorkOS Docs: Delete an object](https://workos.com/docs/reference/vault/delete-object)
    ///
    /// # Examples
    ///
    /// ```
    /// # use workos::{ApiKey, WorkOs};
    /// # use workos::vault::*;
    /// #
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let workos = WorkOs::new(&ApiKey::from("sk_example_123456789"));
    ///
    /// let params = DeleteVaultObjectParams {
    ///     id: &VaultObjectId::from("secret_51B0AC67C2FB4247AC5ABDDD3C701BDC"),
    ///     version_check: None,
    /// };
    ///
    /// let response = workos.vault().delete_vault_object(&params).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn delete_vault_object(
        &self,
        params: &DeleteVaultObjectParams<'_>,
    ) -> WorkOsResult<DeleteVaultObjectResponse, DeleteVaultObjectError>;
}

#[async_trait]
impl DeleteVaultObject for Vault<'_> {
    async fn delete_vault_object(
        &self,
        params: &DeleteVaultObjectParams<'_>,
    ) -> WorkOsResult<DeleteVaultObjectResponse, DeleteVaultObjectError> {
        let url = self
            .workos
            .base_url()
            .join(&format!("/vault/v1/kv/{}", params.id))?;
        let response = self
            .workos
            .client()
            .delete(url)
            .bearer_auth(self.workos.key())
            .json(&params)
            .send()
            .await?
            .handle_unauthorized_or_generic_error()?
            .json::<DeleteVaultObjectResponse>()
            .await?;

        Ok(response)
    }
}

#[cfg(test)]
mod test {
    use mockito;
    use serde_json::json;
    use tokio;

    use crate::vault::{DeleteVaultObject, DeleteVaultObjectParams, VaultObjectId};
    use crate::{ApiKey, WorkOs};

    #[tokio::test]
    async fn it_calls_the_delete_endpoint() {
        let mut server = mockito::Server::new_async().await;
        let api_key = ApiKey::from("sk_example_123456789");
        let workos = WorkOs::builder(&api_key)
            .base_url(&server.url())
            .unwrap()
            .build();

        let object_id = "secret_51B0AC67C2FB4247AC5ABDDD3C701BDC";
        let _mock = server
            .mock("DELETE", format!("/vault/v1/kv/{}", object_id).as_str())
            .match_header("Authorization", format!("Bearer {api_key}").as_str())
            .with_status(200)
            .with_body(
                json!({
                  "success": true,
                  "name": "secret-name"
                })
                .to_string(),
            )
            .create_async()
            .await;

        let params = DeleteVaultObjectParams {
            id: &VaultObjectId::from(object_id),
            version_check: None,
        };

        let response = workos.vault().delete_vault_object(&params).await.unwrap();

        assert!(response.success);
        assert_eq!(response.name, "secret-name");
    }
}
