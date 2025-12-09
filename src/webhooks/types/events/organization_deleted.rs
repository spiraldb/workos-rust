use serde::Deserialize;

use crate::organizations::Organization;

/// [WorkOS Docs: `organization.deleted` Webhook](https://workos.com/docs/events/organization#organization-deleted)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct OrganizationDeletedWebhook(pub Organization);

#[cfg(test)]
mod test {
    use serde_json::json;

    use super::*;
    use crate::organizations::{
        OrganizationDomain, OrganizationDomainId, OrganizationDomainState, OrganizationId,
        VerificationStrategy,
    };
    use crate::webhooks::{Webhook, WebhookEvent, WebhookId};
    use crate::{Timestamp, Timestamps};

    #[test]
    fn it_deserializes_an_organization_deleted_webhook() {
        let webhook: Webhook = serde_json::from_str(
            &json!({
              "id": "wh_01G699XH8F3MAJJWSHZFQ3WWVX",
              "event": "organization.deleted",
              "data": {
                "object": "organization",
                "id": "org_01EHWNCE74X7JSDV0X3SZ3KJNY",
                "name": "Foo Corp",
                "domains": [
                  {
                    "object": "organization_domain",
                    "id": "org_domain_01EHWNFTAFCF3CQAE5A9Q0P1YB",
                    "organization_id": "org_01EHWNCE74X7JSDV0X3SZ3KJNY",
                    "domain": "foo-corp.com",
                    "state": "verified",
                    "verification_strategy": "dns",
                    "verification_token": "token123"
                  }
                ],
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
                event: WebhookEvent::OrganizationDeleted(OrganizationDeletedWebhook(
                    Organization {
                        id: OrganizationId::from("org_01EHWNCE74X7JSDV0X3SZ3KJNY"),
                        name: "Foo Corp".to_string(),
                        domains: vec![OrganizationDomain {
                            id: OrganizationDomainId::from("org_domain_01EHWNFTAFCF3CQAE5A9Q0P1YB"),
                            organization_id: OrganizationId::from("org_01EHWNCE74X7JSDV0X3SZ3KJNY"),
                            domain: "foo-corp.com".to_string(),
                            state: OrganizationDomainState::Verified,
                            verification_strategy: VerificationStrategy::Dns,
                            verification_token: Some("token123".to_string()),
                        }],
                        stripe_customer_id: None,
                        external_id: None,
                        metadata: None,
                        timestamps: Timestamps {
                            created_at: Timestamp::try_from("2021-06-25T19:07:33.155Z").unwrap(),
                            updated_at: Timestamp::try_from("2021-06-25T19:07:33.155Z").unwrap()
                        }
                    }
                ))
            }
        )
    }
}
