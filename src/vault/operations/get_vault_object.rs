use async_trait::async_trait;
use thiserror::Error;

use crate::vault::{Vault, VaultObject, VaultObjectId};
use crate::{ResponseExt, WorkOsError, WorkOsResult};

/// The error returned from the [`GetVaultObject`] trait.
#[derive(Debug, Error)]
pub enum GetVaultObjectError {}

impl From<GetVaultObjectError> for WorkOsError<GetVaultObjectError> {
    fn from(err: GetVaultObjectError) -> Self {
        Self::Operation(err)
    }
}

/// [WorkOS Docs: Get an object](https://workos.com/docs/reference/vault/get-object)
#[async_trait]
pub trait GetVaultObject {
    /// Gets an existing object. The stored value will be decrypted and returned.
    ///
    /// [WorkOS Docs: Get an object](https://workos.com/docs/reference/vault/get-object)
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
    /// let object = workos
    ///     .vault()
    ///     .get_vault_object(&VaultObjectId::from("secret_51B0AC67C2FB4247AC5ABDDD3C701BDC"))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn get_vault_object(
        &self,
        id: &VaultObjectId,
    ) -> WorkOsResult<VaultObject, GetVaultObjectError>;
}

#[async_trait]
impl GetVaultObject for Vault<'_> {
    async fn get_vault_object(
        &self,
        id: &VaultObjectId,
    ) -> WorkOsResult<VaultObject, GetVaultObjectError> {
        let url = self
            .workos
            .base_url()
            .join(&format!("/vault/v1/kv/{id}"))?;
        let object = self
            .workos
            .client()
            .get(url)
            .bearer_auth(self.workos.key())
            .send()
            .await?
            .handle_unauthorized_or_generic_error()?
            .json::<VaultObject>()
            .await?;

        Ok(object)
    }
}

#[cfg(test)]
mod test {
    use mockito;
    use serde_json::json;
    use tokio;

    use crate::vault::{GetVaultObject, VaultObjectId};
    use crate::{ApiKey, WorkOs};

    #[tokio::test]
    async fn it_calls_the_get_endpoint() {
        let mut server = mockito::Server::new_async().await;
        let api_key = ApiKey::from("sk_example_123456789");
        let workos = WorkOs::builder(&api_key)
            .base_url(&server.url())
            .unwrap()
            .build();

        let object_id = "secret_51B0AC67C2FB4247AC5ABDDD3C701BDC";
        let _mock = server
            .mock("GET", format!("/vault/v1/kv/{}", object_id).as_str())
            .match_header("Authorization", format!("Bearer {api_key}").as_str())
            .with_status(200)
            .with_body(
                json!({
                  "id": "secret_51B0AC67C2FB4247AC5ABDDD3C701BDC",
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
                  },
                  "name": "secret-name",
                  "value": "my secret value"
                })
                .to_string(),
            )
            .create_async()
            .await;

        let object = workos
            .vault()
            .get_vault_object(&VaultObjectId::from(object_id))
            .await
            .unwrap();

        assert_eq!(object.id.to_string(), object_id);
        assert_eq!(object.name, "secret-name");
        assert_eq!(object.value, "my secret value");
    }
}
