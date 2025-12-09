use async_trait::async_trait;
use thiserror::Error;

use crate::vault::{Vault, VaultObjectId, VaultObjectVersion};
use crate::{PaginatedList, ResponseExt, WorkOsError, WorkOsResult};

/// The error returned from the [`ListVaultObjectVersions`] trait.
#[derive(Debug, Error)]
pub enum ListVaultObjectVersionsError {}

impl From<ListVaultObjectVersionsError> for WorkOsError<ListVaultObjectVersionsError> {
    fn from(err: ListVaultObjectVersionsError) -> Self {
        Self::Operation(err)
    }
}

/// [WorkOS Docs: List object versions](https://workos.com/docs/reference/vault/list-versions)
#[async_trait]
pub trait ListVaultObjectVersions {
    /// Gets a list of versions for an object stored in Vault.
    ///
    /// [WorkOS Docs: List object versions](https://workos.com/docs/reference/vault/list-versions)
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
    /// let versions = workos
    ///     .vault()
    ///     .list_vault_object_versions(&VaultObjectId::from("secret_51B0AC67C2FB4247AC5ABDDD3C701BDC"))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn list_vault_object_versions(
        &self,
        id: &VaultObjectId,
    ) -> WorkOsResult<PaginatedList<VaultObjectVersion>, ListVaultObjectVersionsError>;
}

#[async_trait]
impl ListVaultObjectVersions for Vault<'_> {
    async fn list_vault_object_versions(
        &self,
        id: &VaultObjectId,
    ) -> WorkOsResult<PaginatedList<VaultObjectVersion>, ListVaultObjectVersionsError> {
        let url = self
            .workos
            .base_url()
            .join(&format!("/vault/v1/kv/{id}/versions"))?;
        let versions = self
            .workos
            .client()
            .get(url)
            .bearer_auth(self.workos.key())
            .send()
            .await?
            .handle_unauthorized_or_generic_error()?
            .json::<PaginatedList<VaultObjectVersion>>()
            .await?;

        Ok(versions)
    }
}

#[cfg(test)]
mod test {
    use mockito;
    use serde_json::json;
    use tokio;

    use crate::vault::{ListVaultObjectVersions, VaultObjectId};
    use crate::{ApiKey, WorkOs};

    #[tokio::test]
    async fn it_calls_the_list_versions_endpoint() {
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
                format!("/vault/v1/kv/{}/versions", object_id).as_str(),
            )
            .match_header("Authorization", format!("Bearer {api_key}").as_str())
            .with_status(200)
            .with_body(
                json!({
                  "data": [
                    {
                      "created_at": "2024-02-21T12:04:09.165291Z",
                      "current_version": true,
                      "etag": "\"62b747b941ceefd67dacc026724044e4\"",
                      "id": "Wq49AmJIR7QI0kSwfY9BZ6vNsOq6AO_X",
                      "size": 271
                    }
                  ],
                  "list_metadata": {
                    "before": null,
                    "after": null
                  }
                })
                .to_string(),
            )
            .create_async()
            .await;

        let versions = workos
            .vault()
            .list_vault_object_versions(&VaultObjectId::from(object_id))
            .await
            .unwrap();

        assert_eq!(versions.data.len(), 1);
        assert_eq!(versions.data[0].id, "Wq49AmJIR7QI0kSwfY9BZ6vNsOq6AO_X");
        assert!(versions.data[0].current_version);
    }
}
