use async_trait::async_trait;
use serde::Serialize;
use thiserror::Error;

use crate::organizations::OrganizationId;
use crate::user_management::types::{OrganizationMembership, OrganizationMembershipStatus, UserId};
use crate::{PaginatedList, PaginationParams, ResponseExt, UrlEncodableVec, WorkOsError, WorkOsResult};

/// The parameters for [`ListOrganizationMemberships`].
///
/// At least one of `organization_id` or `user_id` must be provided.
#[derive(Debug, Default)]
pub struct ListOrganizationMembershipsParams<'a> {
    /// The ID of the organization to list memberships for.
    pub organization_id: Option<&'a OrganizationId>,

    /// The ID of the user to list memberships for.
    pub user_id: Option<&'a UserId>,

    /// Filter by membership status. Defaults to active only when not specified.
    pub statuses: Option<Vec<OrganizationMembershipStatus>>,

    /// The pagination parameters to use when listing organization memberships.
    pub pagination: PaginationParams<'a>,
}

/// Internal query struct for serialization.
#[derive(Debug, Serialize)]
struct ListOrganizationMembershipsQuery<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    organization_id: Option<&'a OrganizationId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<&'a UserId>,

    #[serde(rename = "statuses[]", skip_serializing_if = "Option::is_none")]
    statuses: Option<UrlEncodableVec<OrganizationMembershipStatus>>,

    #[serde(flatten)]
    pagination: PaginationParams<'a>,
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
        let query = ListOrganizationMembershipsQuery {
            organization_id: params.organization_id,
            user_id: params.user_id,
            statuses: params.statuses.clone().map(UrlEncodableVec::from),
            pagination: params.pagination.clone(),
        };
        let memberships = self
            .workos
            .client()
            .get(url)
            .bearer_auth(self.workos.key())
            .query(&query)
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
            .match_query(Matcher::UrlEncoded(
                "organization_id".to_string(),
                "org_01EHZNVPK3SFK441A1RGBFSHRT".to_string(),
            ))
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
                organization_id: Some(&OrganizationId::from("org_01EHZNVPK3SFK441A1RGBFSHRT")),
                ..Default::default()
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
