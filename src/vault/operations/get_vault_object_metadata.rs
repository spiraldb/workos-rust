use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::vault::{Vault, VaultObjectId, VaultObjectMetadata};
use crate::{ResponseExt, WorkOsError, WorkOsResult};

/// Response from retrieving vault object metadata (without the value).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VaultObjectMetadataResponse {
    /// The unique identifier for the object.
    pub id: VaultObjectId,

    /// The name of the secret.
    pub name: String,

    /// Metadata about the object.
    pub metadata: VaultObjectMetadata,
}

/// The error returned from the [`GetVaultObjectMetadata`] trait.
#[derive(Debug, Error)]
pub enum GetVaultObjectMetadataError {}

impl From<GetVaultObjectMetadataError> for WorkOsError<GetVaultObjectMetadataError> {
    fn from(err: GetVaultObjectMetadataError) -> Self {
        Self::Operation(err)
    }
}

/// [WorkOS Docs: Retrieve metadata](https://workos.com/docs/reference/vault/get-metadata)
#[async_trait]
pub trait GetVaultObjectMetadata {
    /// Retrieves metadata about an object. The value itself is not returned.
    ///
    /// [WorkOS Docs: Retrieve metadata](https://workos.com/docs/reference/vault/get-metadata)
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
    /// let metadata = workos
    ///     .vault()
    ///     .get_vault_object_metadata(&VaultObjectId::from("secret_51B0AC67C2FB4247AC5ABDDD3C701BDC"))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn get_vault_object_metadata(
        &self,
        id: &VaultObjectId,
    ) -> WorkOsResult<VaultObjectMetadataResponse, GetVaultObjectMetadataError>;
}

#[async_trait]
impl GetVaultObjectMetadata for Vault<'_> {
    async fn get_vault_object_metadata(
        &self,
        id: &VaultObjectId,
    ) -> WorkOsResult<VaultObjectMetadataResponse, GetVaultObjectMetadataError> {
        let url = self
            .workos
            .base_url()
            .join(&format!("/vault/v1/kv/{id}/metadata"))?;
        let metadata = self
            .workos
            .client()
            .get(url)
            .bearer_auth(self.workos.key())
            .send()
            .await?
            .handle_unauthorized_or_generic_error()?
            .json::<VaultObjectMetadataResponse>()
            .await?;

        Ok(metadata)
    }
}

#[cfg(test)]
mod test {
    use mockito;
    use serde_json::json;
    use tokio;

    use crate::vault::{GetVaultObjectMetadata, VaultObjectId};
    use crate::{ApiKey, WorkOs};

    #[tokio::test]
    async fn it_calls_the_get_metadata_endpoint() {
        let mut server = mockito::Server::new_async().await;
        let api_key = ApiKey::from("sk_example_123456789");
        let workos = WorkOs::builder(&api_key)
            .base_url(&server.url())
            .unwrap()
            .build();

        let object_id = "secret_51B0AC67C2FB4247AC5ABDDD3C701BDC";
        let _mock = server
            .mock(
                "GET",
                format!("/vault/v1/kv/{}/metadata", object_id).as_str(),
            )
            .match_header("Authorization", format!("Bearer {api_key}").as_str())
            .with_status(200)
            .with_body(
                json!({
                  "id": "secret_51B0AC67C2FB4247AC5ABDDD3C701BDC",
                  "name": "secret-name",
                  "metadata": {
                    "id": "secret_51B0AC67C2FB4247AC5ABDDD3C701BDC",
                    "environment_id": "environment_example_23456789",
                    "context": {
                      "organization_id": "org_01EHZNVPK3SFK441A1RGBFSHRT"
                    },
                    "key_id": "e2084ada-50c1-5f9a-b1c7-fa868d506e5a",
                    "updated_at": "2024-02-21T12:04:09.165291Z",
                    "updated_by": {
                      "id": "user_01E4ZCR3C56J083X43JQXF3JK5",
                      "name": "Marcelina Davis"
                    },
                    "version_id": "Wq49AmJIR7QI0kSwfY9BZ6vNsOq6AO_X"
                  }
                })
                .to_string(),
            )
            .create_async()
            .await;

        let metadata = workos
            .vault()
            .get_vault_object_metadata(&VaultObjectId::from(object_id))
            .await
            .unwrap();

        assert_eq!(metadata.id.to_string(), object_id);
        assert_eq!(metadata.name, "secret-name");
    }
}
