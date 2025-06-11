use async_trait::async_trait;
use serde::Serialize;
use thiserror::Error;

use crate::organizations::OrganizationId;
use crate::user_management::types::OrganizationMembership;
use crate::{PaginatedList, PaginationParams, ResponseExt, WorkOsError, WorkOsResult};

/// The parameters for [`ListOrganizationMemberships`].
#[derive(Debug, Serialize)]
pub struct ListOrganizationMembershipsParams<'a> {
    /// The ID of the organization to list memberships for.
    pub organization_id: &'a OrganizationId,

    /// The pagination parameters to use when listing organization memberships.
    #[serde(flatten)]
    pub pagination: PaginationParams<'a>,
}

/// An error returned from [`ListOrganizationMemberships`].
#[derive(Debug, Error)]
pub enum ListOrganizationMembershipsError {}

impl From<ListOrganizationMembershipsError> for WorkOsError<ListOrganizationMembershipsError> {
    fn from(err: ListOrganizationMembershipsError) -> Self {
        Self::Operation(err)
    }
}

/// [WorkOS Docs: List Organization Memberships](https://workos.com/docs/reference/user-management/organization-membership/list)
#[async_trait]
pub trait ListOrganizationMemberships {
    /// Lists organization memberships.
    ///
    /// [WorkOS Docs: List Organization Memberships](https://workos.com/docs/reference/user-management/organization-membership/list)
    async fn list_organization_memberships(
        &self,
        params: &ListOrganizationMembershipsParams<'_>,
    ) -> WorkOsResult<PaginatedList<OrganizationMembership>, ListOrganizationMembershipsError>;
}

#[async_trait]
impl ListOrganizationMemberships for crate::user_management::UserManagement<'_> {
    async fn list_organization_memberships(
        &self,
        params: &ListOrganizationMembershipsParams<'_>,
    ) -> WorkOsResult<PaginatedList<OrganizationMembership>, ListOrganizationMembershipsError> {
        let url = self
            .workos
            .base_url()
            .join("/user_management/organization_memberships")?;
        let memberships = self
            .workos
            .client()
            .get(url)
            .bearer_auth(self.workos.key())
            .query(&params)
            .send()
            .await?
            .handle_unauthorized_or_generic_error()?
            .json::<PaginatedList<OrganizationMembership>>()
            .await?;

        Ok(memberships)
    }
}

#[cfg(test)]
mod test {
    use mockito::Matcher;
    use serde_json::json;
    use tokio;

    use super::*;
    use crate::{ApiKey, WorkOs};

    #[tokio::test]
    async fn it_calls_the_list_organization_memberships_endpoint() {
        let mut server = mockito::Server::new_async().await;
        let workos = WorkOs::builder(&ApiKey::from("sk_example_123456789"))
            .base_url(&server.url())
            .unwrap()
            .build();

        let _mock = server
            .mock("GET", "/user_management/organization_memberships")
            .match_query(Matcher::UrlEncoded("order".to_string(), "desc".to_string()))
            .match_header("Authorization", "Bearer sk_example_123456789")
            .with_status(200)
            .with_body(
                json!({
                  "data": [
                    {
                      "id": "org_membership_01EHZNVPK3SFK441A1RGBFSHRT",
                      "object": "organization_membership",
                      "organization_id": "org_01EHZNVPK3SFK441A1RGBFSHRT",
                      "user_id": "user_01EHZNVPK3SFK441A1RGBFSHRT",
                      "role": {
                        "slug": "owner"
                      },
                        "status": "active",
                      "created_at": "2021-06-25T19:07:33.155Z",
                      "updated_at": "2021-06-25T19:07:33.155Z"
                    }
                  ],
                  "list_metadata": {
                    "before": "org_membership_01EHZNVPK3SFK441A1RGBFSHRT",
                    "after": "org_membership_01EJBGJT2PC6638TN5Y380M40Z",
                  }
                })
                .to_string(),
            )
            .create_async()
            .await;

        let paginated_list = workos
            .user_management()
            .list_organization_memberships(&ListOrganizationMembershipsParams {
                organization_id: &OrganizationId::from("org_01EHZNVPK3SFK441A1RGBFSHRT"),
                pagination: Default::default(),
            })
            .await
            .unwrap();

        assert_eq!(paginated_list.data.len(), 1);
        assert_eq!(
            paginated_list.data[0].id,
            "org_membership_01EHZNVPK3SFK441A1RGBFSHRT".into()
        );
        assert_eq!(
            paginated_list.metadata.after,
            Some("org_membership_01EJBGJT2PC6638TN5Y380M40Z".to_string())
        );
    }
}
