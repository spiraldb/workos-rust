use async_trait::async_trait;
use serde::Serialize;
use thiserror::Error;

use crate::vault::{Vault, VaultObject, VaultObjectId};
use crate::{ResponseExt, WorkOsError, WorkOsResult};

/// The parameters for [`UpdateVaultObject`].
#[derive(Debug, Serialize)]
pub struct UpdateVaultObjectParams<'a> {
    /// The ID of the object to update.
    #[serde(skip_serializing)]
    pub id: &'a VaultObjectId,

    /// The new value to encrypt and store.
    pub value: &'a str,

    /// Optional version check to ensure the update is applied to the expected version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_check: Option<&'a str>,
}

/// The error returned from the [`UpdateVaultObject`] trait.
#[derive(Debug, Error)]
pub enum UpdateVaultObjectError {}

impl From<UpdateVaultObjectError> for WorkOsError<UpdateVaultObjectError> {
    fn from(err: UpdateVaultObjectError) -> Self {
        Self::Operation(err)
    }
}

/// [WorkOS Docs: Update an object value](https://workos.com/docs/reference/vault/update-object)
#[async_trait]
pub trait UpdateVaultObject {
    /// Updates the value for an object. The key context of the original object will be used to encrypt the new data.
    ///
    /// [WorkOS Docs: Update an object value](https://workos.com/docs/reference/vault/update-object)
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
    /// let params = UpdateVaultObjectParams {
    ///     id: &VaultObjectId::from("secret_51B0AC67C2FB4247AC5ABDDD3C701BDC"),
    ///     value: "my updated secret value",
    ///     version_check: Some("Wq49AmJIR7QI0kSwfY9BZ6vNsOq6AO_X"),
    /// };
    ///
    /// let object = workos.vault().update_vault_object(&params).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn update_vault_object(
        &self,
        params: &UpdateVaultObjectParams<'_>,
    ) -> WorkOsResult<VaultObject, UpdateVaultObjectError>;
}

#[async_trait]
impl UpdateVaultObject for Vault<'_> {
    async fn update_vault_object(
        &self,
        params: &UpdateVaultObjectParams<'_>,
    ) -> WorkOsResult<VaultObject, UpdateVaultObjectError> {
        let url = self
            .workos
            .base_url()
            .join(&format!("/vault/v1/kv/{}", params.id))?;
        let object = self
            .workos
            .client()
            .put(url)
            .bearer_auth(self.workos.key())
            .json(&params)
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

    use crate::vault::{UpdateVaultObject, UpdateVaultObjectParams, VaultObjectId};
    use crate::{ApiKey, WorkOs};

    #[tokio::test]
    async fn it_calls_the_update_endpoint() {
        let mut server = mockito::Server::new_async().await;
        let api_key = ApiKey::from("sk_example_123456789");
        let workos = WorkOs::builder(&api_key)
            .base_url(&server.url())
            .unwrap()
            .build();

        let object_id = "secret_51B0AC67C2FB4247AC5ABDDD3C701BDC";
        let _mock = server
            .mock("PUT", format!("/vault/v1/kv/{}", object_id).as_str())
            .match_header("Authorization", format!("Bearer {api_key}").as_str())
            .with_status(200)
            .with_body(
                json!({
                  "id": "secret_51B0AC67C2FB4247AC5ABDDD3C701BDC",
                  "name": "secret-name",
                  "value": "my updated secret value",
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

        let params = UpdateVaultObjectParams {
            id: &VaultObjectId::from(object_id),
            value: "my updated secret value",
            version_check: Some("Wq49AmJIR7QI0kSwfY9BZ6vNsOq6AO_X"),
        };

        let object = workos.vault().update_vault_object(&params).await.unwrap();

        assert_eq!(object.id.to_string(), object_id);
    }
}
