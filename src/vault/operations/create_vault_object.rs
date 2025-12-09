use async_trait::async_trait;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use thiserror::Error;

use crate::vault::{Vault, VaultObjectMetadata};
use crate::{ResponseExt, WorkOsError, WorkOsResult};

/// The parameters for [`CreateVaultObject`].
#[derive(Debug, Serialize)]
pub struct CreateVaultObjectParams<'a> {
    /// The name of the secret.
    pub name: &'a str,

    /// The value to encrypt and store.
    pub value: &'a str,

    /// The key context for encryption.
    pub key_context: HashMap<&'a str, Value>,
}

/// The error returned from the [`CreateVaultObject`] trait.
#[derive(Debug, Error)]
pub enum CreateVaultObjectError {}

impl From<CreateVaultObjectError> for WorkOsError<CreateVaultObjectError> {
    fn from(err: CreateVaultObjectError) -> Self {
        Self::Operation(err)
    }
}

/// [WorkOS Docs: Create an object](https://workos.com/docs/reference/vault/create-object)
#[async_trait]
pub trait CreateVaultObject {
    /// Creates a new encrypted object in Vault.
    ///
    /// [WorkOS Docs: Create an object](https://workos.com/docs/reference/vault/create-object)
    ///
    /// # Examples
    ///
    /// ```
    /// # use workos::{ApiKey, WorkOs};
    /// # use workos::vault::*;
    /// # use std::collections::HashMap;
    /// # use serde_json::json;
    /// #
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let workos = WorkOs::new(&ApiKey::from("sk_example_123456789"));
    ///
    /// let mut key_context = HashMap::new();
    /// key_context.insert("organization_id", json!("org_01EHZNVPK3SFK441A1RGBFSHRT"));
    ///
    /// let params = CreateVaultObjectParams {
    ///     name: "secret-name",
    ///     value: "my secret value",
    ///     key_context,
    /// };
    ///
    /// let metadata = workos.vault().create_vault_object(&params).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn create_vault_object(
        &self,
        params: &CreateVaultObjectParams<'_>,
    ) -> WorkOsResult<VaultObjectMetadata, CreateVaultObjectError>;
}

#[async_trait]
impl CreateVaultObject for Vault<'_> {
    async fn create_vault_object(
        &self,
        params: &CreateVaultObjectParams<'_>,
    ) -> WorkOsResult<VaultObjectMetadata, CreateVaultObjectError> {
        let url = self.workos.base_url().join("/vault/v1/kv")?;
        let metadata = self
            .workos
            .client()
            .post(url)
            .bearer_auth(self.workos.key())
            .json(&params)
            .send()
            .await?
            .handle_unauthorized_or_generic_error()?
            .json::<VaultObjectMetadata>()
            .await?;

        Ok(metadata)
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use mockito;
    use serde_json::json;
    use tokio;

    use crate::vault::{CreateVaultObject, CreateVaultObjectParams};
    use crate::{ApiKey, WorkOs};

    #[tokio::test]
    async fn it_calls_the_create_endpoint() {
        let mut server = mockito::Server::new_async().await;
        let api_key = ApiKey::from("sk_example_123456789");
        let workos = WorkOs::builder(&api_key)
            .base_url(&server.url())
            .unwrap()
            .build();

        let _mock = server
            .mock("POST", "/vault/v1/kv")
            .match_header("Authorization", format!("Bearer {api_key}").as_str())
            .with_status(200)
            .with_body(
                json!({
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
                })
                .to_string(),
            )
            .create_async()
            .await;

        let mut key_context = HashMap::new();
        key_context.insert("organization_id", json!("org_01EHZNVPK3SFK441A1RGBFSHRT"));

        let params = CreateVaultObjectParams {
            name: "secret-name",
            value: "my secret value",
            key_context,
        };

        let metadata = workos
            .vault()
            .create_vault_object(&params)
            .await
            .unwrap();

        assert_eq!(
            metadata.id.to_string(),
            "secret_51B0AC67C2FB4247AC5ABDDD3C701BDC"
        );
    }
}
