use serde::Deserialize;

use crate::organizations::Organization;

/// [WorkOS Docs: `organization.updated` Webhook](https://workos.com/docs/events/organization#organization-updated)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct OrganizationUpdatedWebhook(pub Organization);

#[cfg(test)]
mod test {
    use serde_json::json;

    use super::*;
    use crate::organizations::{OrganizationDomain, OrganizationDomainId, OrganizationId};
    use crate::webhooks::{Webhook, WebhookEvent, WebhookId};
    use crate::{Timestamp, Timestamps};

    #[test]
    fn it_deserializes_an_organization_updated_webhook() {
        let webhook: Webhook = serde_json::from_str(
            &json!({
              "id": "wh_01G699XH8F3MAJJWSHZFQ3WWVX",
              "event": "organization.updated",
              "data": {
                "object": "organization",
                "id": "org_01EHWNCE74X7JSDV0X3SZ3KJNY",
                "name": "Foo Corp Updated",
                "allow_profiles_outside_organization": true,
                "domains": [
                  {
                    "object": "organization_domain",
                    "id": "org_domain_01EHWNFTAFCF3CQAE5A9Q0P1YB",
                    "domain": "foo-corp.com"
                  },
                  {
                    "object": "organization_domain",
                    "id": "org_domain_02EHWNFTAFCF3CQAE5A9Q0P1YC",
                    "domain": "foo-corp.org"
                  }
                ],
                "created_at": "2021-06-25T19:07:33.155Z",
                "updated_at": "2021-06-25T19:10:45.200Z"
              }
            })
            .to_string(),
        )
        .unwrap();

        assert_eq!(
            webhook,
            Webhook {
                id: WebhookId::from("wh_01G699XH8F3MAJJWSHZFQ3WWVX"),
                event: WebhookEvent::OrganizationUpdated(OrganizationUpdatedWebhook(
                    Organization {
                        id: OrganizationId::from("org_01EHWNCE74X7JSDV0X3SZ3KJNY"),
                        name: "Foo Corp Updated".to_string(),
                        allow_profiles_outside_organization: true,
                        domains: vec![
                            OrganizationDomain {
                                id: OrganizationDomainId::from(
                                    "org_domain_01EHWNFTAFCF3CQAE5A9Q0P1YB"
                                ),
                                domain: "foo-corp.com".to_string()
                            },
                            OrganizationDomain {
                                id: OrganizationDomainId::from(
                                    "org_domain_02EHWNFTAFCF3CQAE5A9Q0P1YC"
                                ),
                                domain: "foo-corp.org".to_string()
                            }
                        ],
                        timestamps: Timestamps {
                            created_at: Timestamp::try_from("2021-06-25T19:07:33.155Z").unwrap(),
                            updated_at: Timestamp::try_from("2021-06-25T19:10:45.200Z").unwrap()
                        }
                    }
                ))
            }
        )
    }
}
