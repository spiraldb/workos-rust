use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::vault::Vault;
use crate::{PaginatedList, PaginationParams, ResponseExt, WorkOsError, WorkOsResult};

/// The parameters for [`ListVaultObjects`].
#[derive(Debug, Default, Serialize)]
pub struct ListVaultObjectsParams<'a> {
    /// Pagination parameters.
    #[serde(flatten)]
    pub pagination: PaginationParams<'a>,

    /// Filter objects updated after this timestamp (ISO 8601 format).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_after: Option<&'a str>,
}

/// A vault object name in the list response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VaultObjectListItem {
    /// The name of the object.
    pub name: String,
}

/// The error returned from the [`ListVaultObjects`] trait.
#[derive(Debug, Error)]
pub enum ListVaultObjectsError {}

impl From<ListVaultObjectsError> for WorkOsError<ListVaultObjectsError> {
    fn from(err: ListVaultObjectsError) -> Self {
        Self::Operation(err)
    }
}

/// [WorkOS Docs: List objects](https://workos.com/docs/reference/vault/list-objects)
#[async_trait]
pub trait ListVaultObjects {
    /// Gets a list of object names stored in Vault.
    ///
    /// [WorkOS Docs: List objects](https://workos.com/docs/reference/vault/list-objects)
    ///
    /// # Examples
    ///
    /// ```
    /// # use workos::{ApiKey, WorkOs};
    /// # use workos::vault::*;
    /// # use workos::PaginationParams;
    /// #
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let workos = WorkOs::new(&ApiKey::from("sk_example_123456789"));
    ///
    /// let params = ListVaultObjectsParams {
    ///     pagination: PaginationParams {
    ///         limit: Some(10),
    ///         ..Default::default()
    ///     },
    ///     updated_after: None,
    /// };
    ///
    /// let objects = workos.vault().list_vault_objects(&params).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn list_vault_objects(
        &self,
        params: &ListVaultObjectsParams<'_>,
    ) -> WorkOsResult<PaginatedList<VaultObjectListItem>, ListVaultObjectsError>;
}

#[async_trait]
impl ListVaultObjects for Vault<'_> {
    async fn list_vault_objects(
        &self,
        params: &ListVaultObjectsParams<'_>,
    ) -> WorkOsResult<PaginatedList<VaultObjectListItem>, ListVaultObjectsError> {
        let url = self.workos.base_url().join("/vault/v1/kv")?;
        let objects = self
            .workos
            .client()
            .get(url)
            .query(&params)
            .bearer_auth(self.workos.key())
            .send()
            .await?
            .handle_unauthorized_or_generic_error()?
            .json::<PaginatedList<VaultObjectListItem>>()
            .await?;

        Ok(objects)
    }
}

#[cfg(test)]
mod test {
    use mockito::{self, Matcher};
    use serde_json::json;
    use tokio;

    use crate::vault::{ListVaultObjects, ListVaultObjectsParams};
    use crate::{ApiKey, PaginationParams, WorkOs};

    #[tokio::test]
    async fn it_calls_the_list_endpoint() {
        let mut server = mockito::Server::new_async().await;
        let api_key = ApiKey::from("sk_example_123456789");
        let workos = WorkOs::builder(&api_key)
            .base_url(&server.url())
            .unwrap()
            .build();

        let _mock = server
            .mock("GET", "/vault/v1/kv")
            .match_query(Matcher::UrlEncoded("limit".to_string(), "10".to_string()))
            .match_header("Authorization", format!("Bearer {api_key}").as_str())
            .with_status(200)
            .with_body(
                json!({
                  "data": [
                    { "name": "secret-1" },
                    { "name": "secret-2" }
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

        let params = ListVaultObjectsParams {
            pagination: PaginationParams {
                limit: Some(10),
                ..Default::default()
            },
            updated_after: None,
        };

        let objects = workos.vault().list_vault_objects(&params).await.unwrap();

        assert_eq!(objects.data.len(), 2);
        assert_eq!(objects.data[0].name, "secret-1");
        assert_eq!(objects.data[1].name, "secret-2");
    }
}
