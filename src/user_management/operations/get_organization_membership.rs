use async_trait::async_trait;
use serde::Serialize;
use thiserror::Error;

use crate::user_management::types::OrganizationMembership;
use crate::{ResponseExt, WorkOsError, WorkOsResult};

/// The parameters for [`GetOrganizationMembership`].
#[derive(Debug, Serialize)]
pub struct GetOrganizationMembershipParams<'a> {
    /// The ID of the organization membership to get.
    pub organization_membership_id: &'a str,
}

/// An error returned from [`GetOrganizationMembership`].
#[derive(Debug, Error)]
pub enum GetOrganizationMembershipError {}

impl From<GetOrganizationMembershipError> for WorkOsError<GetOrganizationMembershipError> {
    fn from(err: GetOrganizationMembershipError) -> Self {
        Self::Operation(err)
    }
}

/// [WorkOS Docs: Get Organization Membership](https://workos.com/docs/reference/user-management/organization-membership/get)
#[async_trait]
pub trait GetOrganizationMembership {
    /// Gets an organization membership.
    ///
    /// [WorkOS Docs: Get Organization Membership](https://workos.com/docs/reference/user-management/organization-membership/get)
    async fn get_organization_membership(
        &self,
        params: &GetOrganizationMembershipParams<'_>,
    ) -> WorkOsResult<OrganizationMembership, GetOrganizationMembershipError>;
}

#[async_trait]
impl GetOrganizationMembership for crate::user_management::UserManagement<'_> {
    async fn get_organization_membership(
        &self,
        params: &GetOrganizationMembershipParams<'_>,
    ) -> WorkOsResult<OrganizationMembership, GetOrganizationMembershipError> {
        let url = self.workos.base_url().join(&format!(
            "/user_management/organization_memberships/{}",
            params.organization_membership_id
        ))?;
        let membership = self
            .workos
            .client()
            .get(url)
            .bearer_auth(self.workos.key())
            .send()
            .await?
            .handle_unauthorized_or_generic_error()?
            .json::<OrganizationMembership>()
            .await?;

        Ok(membership)
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;
    use tokio;

    use super::*;
    use crate::{ApiKey, WorkOs};

    #[tokio::test]
    async fn it_calls_the_get_organization_membership_endpoint() {
        let mut server = mockito::Server::new_async().await;
        let workos = WorkOs::builder(&ApiKey::from("sk_example_123456789"))
            .base_url(&server.url())
            .unwrap()
            .build();

        let _mock = server.mock(
            "GET",
            "/user_management/organization_memberships/org_membership_01EHZNVPK3SFK441A1RGBFSHRT",
        )
        .match_header("Authorization", "Bearer sk_example_123456789")
        .with_status(200)
        .with_body(
            json!({
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
            })
            .to_string(),
        )
        .create_async().await;

        let membership = workos
            .user_management()
            .get_organization_membership(&GetOrganizationMembershipParams {
                organization_membership_id: "org_membership_01EHZNVPK3SFK441A1RGBFSHRT",
            })
            .await
            .unwrap();

        assert_eq!(
            membership.id,
            "org_membership_01EHZNVPK3SFK441A1RGBFSHRT".into()
        );
    }
}
