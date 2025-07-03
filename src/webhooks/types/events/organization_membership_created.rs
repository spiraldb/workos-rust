use serde::Deserialize;

use crate::user_management::OrganizationMembership;

/// [WorkOS Docs: `organization_membership.created` Webhook](https://workos.com/docs/events/organization-membership#organization-membership-created)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct OrganizationMembershipCreatedWebhook(pub OrganizationMembership);

#[cfg(test)]
mod test {
    use serde_json::json;

    use super::*;
    use crate::organizations::OrganizationId;
    use crate::user_management::{
        OrganizationMembershipId, OrganizationMembershipStatus, OrganizationRole, UserId,
    };
    use crate::webhooks::{Webhook, WebhookEvent, WebhookId};
    use crate::{Timestamp, Timestamps};

    #[test]
    fn it_deserializes_an_organization_membership_created_webhook() {
        let webhook: Webhook = serde_json::from_str(
            &json!({
              "id": "wh_01G699XH8F3MAJJWSHZFQ3WWVX",
              "event": "organization_membership.created",
              "data": {
                "object": "organization_membership",
                "id": "om_01EHWNCE74X7JSDV0X3SZ3KJNY",
                "user_id": "user_01EHWNCE74X7JSDV0X3SZ3KJNY",
                "organization_id": "org_01EHWNCE74X7JSDV0X3SZ3KJNY",
                "role": {
                  "slug": "admin"
                },
                "status": "active",
                "created_at": "2021-06-25T19:07:33.155Z",
                "updated_at": "2021-06-25T19:07:33.155Z"
              }
            })
            .to_string(),
        )
        .unwrap();

        assert_eq!(
            webhook,
            Webhook {
                id: WebhookId::from("wh_01G699XH8F3MAJJWSHZFQ3WWVX"),
                event: WebhookEvent::OrganizationMembershipCreated(
                    OrganizationMembershipCreatedWebhook(OrganizationMembership {
                        id: OrganizationMembershipId::from("om_01EHWNCE74X7JSDV0X3SZ3KJNY"),
                        user_id: UserId::from("user_01EHWNCE74X7JSDV0X3SZ3KJNY"),
                        organization_id: OrganizationId::from("org_01EHWNCE74X7JSDV0X3SZ3KJNY"),
                        role: OrganizationRole {
                            slug: "admin".to_string()
                        },
                        status: OrganizationMembershipStatus::Active,
                        timestamps: Timestamps {
                            created_at: Timestamp::try_from("2021-06-25T19:07:33.155Z").unwrap(),
                            updated_at: Timestamp::try_from("2021-06-25T19:07:33.155Z").unwrap()
                        }
                    })
                )
            }
        )
    }
}
