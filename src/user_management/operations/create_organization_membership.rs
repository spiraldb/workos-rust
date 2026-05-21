use async_trait::async_trait;
use serde::Serialize;
use thiserror::Error;

use crate::organizations::OrganizationId;
use crate::user_management::types::{OrganizationMembership, UserId};
use crate::{ResponseExt, WorkOsError, WorkOsResult};

/// The parameters for [`CreateOrganizationMembership`].
#[derive(Debug, Serialize)]
pub struct CreateOrganizationMembershipParams<'a> {
    /// The ID of the user to add to the organization.
    pub user_id: &'a UserId,

    /// The ID of the organization.
    pub organization_id: &'a OrganizationId,

    /// The role slug to assign to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_slug: Option<&'a str>,
}

/// An error returned from [`CreateOrganizationMembership`].
#[derive(Debug, Error)]
pub enum CreateOrganizationMembershipError {}

impl From<CreateOrganizationMembershipError>
    for WorkOsError<CreateOrganizationMembershipError>
{
    fn from(err: CreateOrganizationMembershipError) -> Self {
        Self::Operation(err)
    }
}

/// [WorkOS Docs: Create an Organization Membership](https://workos.com/docs/reference/authkit/organization-membership/create)
#[async_trait]
pub trait CreateOrganizationMembership {
    /// Creates an organization membership.
    ///
    /// [WorkOS Docs: Create an Organization Membership](https://workos.com/docs/reference/authkit/organization-membership/create)
    async fn create_organization_membership(
        &self,
        params: &CreateOrganizationMembershipParams<'_>,
    ) -> WorkOsResult<OrganizationMembership, CreateOrganizationMembershipError>;
}

#[async_trait]
impl CreateOrganizationMembership for crate::user_management::UserManagement<'_> {
    async fn create_organization_membership(
        &self,
        params: &CreateOrganizationMembershipParams<'_>,
    ) -> WorkOsResult<OrganizationMembership, CreateOrganizationMembershipError> {
        let url = self
            .workos
            .base_url()
            .join("/user_management/organization_memberships")?;
        let membership = self
            .workos
            .client()
            .post(url)
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
    async fn it_calls_the_create_organization_membership_endpoint() {
        let mut server = mockito::Server::new_async().await;
        let workos = WorkOs::builder(&ApiKey::from("sk_example_123456789"))
            .base_url(&server.url())
            .unwrap()
            .build();

        let _mock = server
            .mock("POST", "/user_management/organization_memberships")
            .match_header("Authorization", "Bearer sk_example_123456789")
            .with_status(201)
            .with_body(
                json!({
                    "id": "org_membership_01EHZNVPK3SFK441A1RGBFSHRT",
                    "object": "organization_membership",
                    "organization_id": "org_01EHZNVPK3SFK441A1RGBFSHRT",
                    "user_id": "user_01EHZNVPK3SFK441A1RGBFSHRT",
                    "role": {
                        "slug": "member"
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
            .create_organization_membership(&CreateOrganizationMembershipParams {
                user_id: &UserId::from("user_01EHZNVPK3SFK441A1RGBFSHRT"),
                organization_id: &OrganizationId::from("org_01EHZNVPK3SFK441A1RGBFSHRT"),
                role_slug: Some("member"),
            })
            .await
            .unwrap();

        assert_eq!(
            membership.id,
            "org_membership_01EHZNVPK3SFK441A1RGBFSHRT".into()
        );
    }
}
