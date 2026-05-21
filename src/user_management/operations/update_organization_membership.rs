use async_trait::async_trait;
use serde::Serialize;
use thiserror::Error;

use crate::user_management::types::{OrganizationMembership, OrganizationMembershipId};
use crate::{ResponseExt, WorkOsError, WorkOsResult};

/// The parameters for [`UpdateOrganizationMembership`].
#[derive(Debug, Serialize)]
pub struct UpdateOrganizationMembershipParams<'a> {
    /// The ID of the organization membership to update.
    #[serde(skip_serializing)]
    pub organization_membership_id: &'a OrganizationMembershipId,

    /// The role slug to assign to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_slug: Option<&'a str>,
}

/// An error returned from [`UpdateOrganizationMembership`].
#[derive(Debug, Error)]
pub enum UpdateOrganizationMembershipError {}

impl From<UpdateOrganizationMembershipError>
    for WorkOsError<UpdateOrganizationMembershipError>
{
    fn from(err: UpdateOrganizationMembershipError) -> Self {
        Self::Operation(err)
    }
}

/// [WorkOS Docs: Update an Organization Membership](https://workos.com/docs/reference/authkit/organization-membership/update)
#[async_trait]
pub trait UpdateOrganizationMembership {
    /// Updates an organization membership.
    ///
    /// [WorkOS Docs: Update an Organization Membership](https://workos.com/docs/reference/authkit/organization-membership/update)
    async fn update_organization_membership(
        &self,
        params: &UpdateOrganizationMembershipParams<'_>,
    ) -> WorkOsResult<OrganizationMembership, UpdateOrganizationMembershipError>;
}

#[async_trait]
impl UpdateOrganizationMembership for crate::user_management::UserManagement<'_> {
    async fn update_organization_membership(
        &self,
        params: &UpdateOrganizationMembershipParams<'_>,
    ) -> WorkOsResult<OrganizationMembership, UpdateOrganizationMembershipError> {
        let url = self.workos.base_url().join(&format!(
            "/user_management/organization_memberships/{}",
            params.organization_membership_id
        ))?;
        let membership = self
            .workos
            .client()
            .put(url)
            .bearer_auth(self.workos.key())
            .json(&params)
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
    async fn it_calls_the_update_organization_membership_endpoint() {
        let mut server = mockito::Server::new_async().await;
        let workos = WorkOs::builder(&ApiKey::from("sk_example_123456789"))
            .base_url(&server.url())
            .unwrap()
            .build();

        let _mock = server
            .mock(
                "PUT",
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
                        "slug": "admin"
                    },
                    "status": "active",
                    "created_at": "2021-06-25T19:07:33.155Z",
                    "updated_at": "2021-06-25T19:07:33.155Z"
                })
                .to_string(),
            )
            .create_async()
            .await;

        let membership = workos
            .user_management()
            .update_organization_membership(&UpdateOrganizationMembershipParams {
                organization_membership_id: &OrganizationMembershipId::from(
                    "org_membership_01EHZNVPK3SFK441A1RGBFSHRT",
                ),
                role_slug: Some("admin"),
            })
            .await
            .unwrap();

        assert_eq!(
            membership.id,
            "org_membership_01EHZNVPK3SFK441A1RGBFSHRT".into()
        );
    }
}
