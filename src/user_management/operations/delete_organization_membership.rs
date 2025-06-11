use async_trait::async_trait;
use serde::Serialize;
use thiserror::Error;

use crate::{ResponseExt, WorkOsError, WorkOsResult};

/// The parameters for [`DeleteOrganizationMembership`].
#[derive(Debug, Serialize)]
pub struct DeleteOrganizationMembershipParams<'a> {
    /// The ID of the organization membership to delete.
    pub organization_membership_id: &'a str,
}

/// An error returned from [`DeleteOrganizationMembership`].
#[derive(Debug, Error)]
pub enum DeleteOrganizationMembershipError {}

impl From<DeleteOrganizationMembershipError> for WorkOsError<DeleteOrganizationMembershipError> {
    fn from(err: DeleteOrganizationMembershipError) -> Self {
        Self::Operation(err)
    }
}

/// [WorkOS Docs: Delete Organization Membership](https://workos.com/docs/reference/user-management/organization-membership/delete)
#[async_trait]
pub trait DeleteOrganizationMembership {
    /// Deletes an organization membership.
    ///
    /// [WorkOS Docs: Delete Organization Membership](https://workos.com/docs/reference/user-management/organization-membership/delete)
    async fn delete_organization_membership(
        &self,
        params: &DeleteOrganizationMembershipParams<'_>,
    ) -> WorkOsResult<(), DeleteOrganizationMembershipError>;
}

#[async_trait]
impl DeleteOrganizationMembership for crate::user_management::UserManagement<'_> {
    async fn delete_organization_membership(
        &self,
        params: &DeleteOrganizationMembershipParams<'_>,
    ) -> WorkOsResult<(), DeleteOrganizationMembershipError> {
        let url = self.workos.base_url().join(&format!(
            "/user_management/organization_memberships/{}",
            params.organization_membership_id
        ))?;
        self.workos
            .client()
            .delete(url)
            .bearer_auth(self.workos.key())
            .send()
            .await?
            .handle_unauthorized_or_generic_error()?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use tokio;

    use super::*;
    use crate::{ApiKey, WorkOs};

    #[tokio::test]
    async fn it_calls_the_delete_organization_membership_endpoint() {
        let mut server = mockito::Server::new_async().await;
        let workos = WorkOs::builder(&ApiKey::from("sk_example_123456789"))
            .base_url(&server.url())
            .unwrap()
            .build();

        let _mock = server.mock(
            "DELETE",
            "/user_management/organization_memberships/org_membership_01EHZNVPK3SFK441A1RGBFSHRT",
        )
        .match_header("Authorization", "Bearer sk_example_123456789")
        .with_status(204)
        .create_async().await;

        workos
            .user_management()
            .delete_organization_membership(&DeleteOrganizationMembershipParams {
                organization_membership_id: "org_membership_01EHZNVPK3SFK441A1RGBFSHRT",
            })
            .await
            .unwrap();
    }
}
