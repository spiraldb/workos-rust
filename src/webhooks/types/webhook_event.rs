use serde::Deserialize;

use super::events::*;

/// The event of a [`Webhook`](crate::webhooks::Webhook).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "event", content = "data")]
pub enum WebhookEvent {
    /// [WorkOS Docs: `connection.activated` Webhook](https://workos.com/docs/reference/webhooks/connection#webhooks-sso.connection.activated)
    #[serde(rename = "connection.activated")]
    ConnectionActivated(ConnectionActivatedWebhook),

    /// [WorkOS Docs: `connection.deactivated` Webhook](https://workos.com/docs/reference/webhooks/connection#webhooks-sso.connection.deactivated)
    #[serde(rename = "connection.deactivated")]
    ConnectionDeactivated(ConnectionDeactivatedWebhook),

    /// [WorkOS Docs: `connection.deleted` Webhook](https://workos.com/docs/reference/webhooks/connection#webhooks-sso.connection.deleted)
    #[serde(rename = "connection.deleted")]
    ConnectionDeleted(ConnectionDeletedWebhook),

    /// [WorkOS Docs: `dsync.activated` Webhook](https://workos.com/docs/reference/webhooks/directory#webhooks-dsync.activated)
    #[serde(rename = "dsync.activated")]
    DirectoryActivated(DirectoryActivatedWebhook),

    /// [WorkOS Docs: `dsync.deactivated` Webhook](https://workos.com/docs/reference/webhooks/directory#webhooks-dsync.deactivated)
    #[serde(rename = "dsync.deactivated")]
    DirectoryDeactivated(DirectoryDeactivatedWebhook),

    /// [WorkOS Docs: `dsync.deleted` Webhook](https://workos.com/docs/reference/webhooks/directory#webhooks-dsync.deleted)
    #[serde(rename = "dsync.deleted")]
    DirectoryDeleted(DirectoryDeletedWebhook),

    /// [WorkOS Docs: `dsync.user.created` Webhook](https://workos.com/docs/reference/webhooks/directory-user#webhooks-dsync.user.created)
    #[serde(rename = "dsync.user.created")]
    DirectoryUserCreated(DirectoryUserCreatedWebhook),

    /// [WorkOS Docs: `dsync.user.created` Webhook](https://workos.com/docs/reference/webhooks/directory-user#webhooks-dsync.user.updated)
    #[serde(rename = "dsync.user.updated")]
    DirectoryUserUpdated(DirectoryUserUpdatedWebhook),

    /// [WorkOS Docs: `dsync.user.deleted` Webhook](https://workos.com/docs/reference/webhooks/directory-user#webhooks-dsync.user.deleted)
    #[serde(rename = "dsync.user.deleted")]
    DirectoryUserDeleted(DirectoryUserDeletedWebhook),

    /// [WorkOS Docs: `dsync.group.created` Webhook](https://workos.com/docs/reference/webhooks/directory-group#webhooks-dsync.group.created)
    #[serde(rename = "dsync.group.created")]
    DirectoryGroupCreated(DirectoryGroupCreatedWebhook),

    /// [WorkOS Docs: `dsync.group.updated` Webhook](https://workos.com/docs/reference/webhooks/directory-group#webhooks-dsync.group.updated)
    #[serde(rename = "dsync.group.updated")]
    DirectoryGroupUpdated(DirectoryGroupUpdatedWebhook),

    /// [WorkOS Docs: `dsync.group.deleted` Webhook](https://workos.com/docs/reference/webhooks/directory-group#webhooks-dsync.group.deleted)
    #[serde(rename = "dsync.group.deleted")]
    DirectoryGroupDeleted(DirectoryGroupDeletedWebhook),

    /// [WorkOS Docs: `dsync.group.user_added` Webhook](https://workos.com/docs/reference/webhooks/directory-group#webhooks-dsync.group.user_added)
    #[serde(rename = "dsync.group.user_added")]
    DirectoryUserAddedToGroup(DirectoryUserAddedToGroupWebhook),

    /// [WorkOS Docs: `dsync.group.user_removed` Webhook](https://workos.com/docs/reference/webhooks/directory-group#webhooks-dsync.group.user_removed)
    #[serde(rename = "dsync.group.user_removed")]
    DirectoryUserRemovedFromGroup(DirectoryUserRemovedFromGroupWebhook),

    /// [WorkOS Docs: `organization.created` Webhook](https://workos.com/docs/events/organization#organization-created)
    #[serde(rename = "organization.created")]
    OrganizationCreated(OrganizationCreatedWebhook),

    /// [WorkOS Docs: `organization.updated` Webhook](https://workos.com/docs/events/organization#organization-updated)
    #[serde(rename = "organization.updated")]
    OrganizationUpdated(OrganizationUpdatedWebhook),

    /// [WorkOS Docs: `organization.deleted` Webhook](https://workos.com/docs/events/organization#organization-deleted)
    #[serde(rename = "organization.deleted")]
    OrganizationDeleted(OrganizationDeletedWebhook),

    /// [WorkOS Docs: `organization_membership.created` Webhook](https://workos.com/docs/events/organization-membership#organization-membership-created)
    #[serde(rename = "organization_membership.created")]
    OrganizationMembershipCreated(OrganizationMembershipCreatedWebhook),

    /// [WorkOS Docs: `organization_membership.updated` Webhook](https://workos.com/docs/events/organization-membership#organization-membership-updated)
    #[serde(rename = "organization_membership.updated")]
    OrganizationMembershipUpdated(OrganizationMembershipUpdatedWebhook),

    /// [WorkOS Docs: `organization_membership.deleted` Webhook](https://workos.com/docs/events/organization-membership#organization-membership-deleted)
    #[serde(rename = "organization_membership.deleted")]
    OrganizationMembershipDeleted(OrganizationMembershipDeletedWebhook),
}
