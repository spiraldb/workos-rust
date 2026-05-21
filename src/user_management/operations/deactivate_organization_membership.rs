use async_trait::async_trait;
use serde::Serialize;
use thiserror::Error;

use crate::user_management::types::{OrganizationMembership, OrganizationMembershipId};
use crate::{ResponseExt, WorkOsError, WorkOsResult};

/// The parameters for [`DeactivateOrganizationMembership`].
#[derive(Debug, Serialize)]
pub struct DeactivateOrganizationMembershipParams<'a> {
    /// The ID of the organization membership to deactivate.
    pub organization_membership_id: &'a OrganizationMembershipId,
}

/// An error returned from [`DeactivateOrganizationMembership`].
#[derive(Debug, Error)]
pub enum DeactivateOrganizationMembershipError {}

impl From<DeactivateOrganizationMembershipError>
    for WorkOsError<DeactivateOrganizationMembershipError>
{
    fn from(err: DeactivateOrganizationMembershipError) -> Self {
        Self::Operation(err)
    }
}

/// [WorkOS Docs: Deactivate an Organization Membership](https://workos.com/docs/reference/authkit/organization-membership/deactivate)
#[async_trait]
pub trait DeactivateOrganizationMembership {
    /// Deactivates an organization membership.
    ///
    /// [WorkOS Docs: Deactivate an Organization Membership](https://workos.com/docs/reference/authkit/organization-membership/deactivate)
    async fn deactivate_organization_membership(
        &self,
        params: &DeactivateOrganizationMembershipParams<'_>,
    ) -> WorkOsResult<OrganizationMembership, DeactivateOrganizationMembershipError>;
}

#[async_trait]
impl DeactivateOrganizationMembership for crate::user_management::UserManagement<'_> {
    async fn deactivate_organization_membership(
        &self,
        params: &DeactivateOrganizationMembershipParams<'_>,
    ) -> WorkOsResult<OrganizationMembership, DeactivateOrganizationMembershipError> {
        let url = self.workos.base_url().join(&format!(
            "/user_management/organization_memberships/{}/deactivate",
            params.organization_membership_id
        ))?;
        let membership = self
            .workos
            .client()
            .put(url)
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
    async fn it_calls_the_deactivate_organization_membership_endpoint() {
        let mut server = mockito::Server::new_async().await;
        let workos = WorkOs::builder(&ApiKey::from("sk_example_123456789"))
            .base_url(&server.url())
            .unwrap()
            .build();

        let _mock = server
            .mock(
                "PUT",
                "/user_management/organization_memberships/org_membership_01EHZNVPK3SFK441A1RGBFSHRT/deactivate",
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
                        "slug": "member"
                    },
                    "status": "inactive",
                    "created_at": "2021-06-25T19:07:33.155Z",
                    "updated_at": "2021-06-25T19:07:33.155Z"
                })
                .to_string(),
            )
            .create_async()
            .await;

        let membership = workos
            .user_management()
            .deactivate_organization_membership(&DeactivateOrganizationMembershipParams {
                organization_membership_id: &OrganizationMembershipId::from(
                    "org_membership_01EHZNVPK3SFK441A1RGBFSHRT",
                ),
            })
            .await
            .unwrap();

        assert_eq!(membership.status, crate::user_management::OrganizationMembershipStatus::Inactive);
    }
}
