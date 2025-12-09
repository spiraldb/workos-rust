use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::Timestamps;

/// The ID of an [`Organization`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct OrganizationId(String);

impl Display for OrganizationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for OrganizationId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for OrganizationId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

/// [WorkOS Docs: Organization](https://workos.com/docs/reference/organization)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Organization {
    /// The ID of the organization.
    pub id: OrganizationId,

    /// The name of the organization.
    pub name: String,

    /// The list of user email domains for the organization.
    pub domains: Vec<OrganizationDomain>,

    /// The Stripe customer ID associated with the organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stripe_customer_id: Option<String>,

    /// The external ID of the organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,

    /// Custom metadata for the organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,

    /// The timestamps for the organization.
    #[serde(flatten)]
    pub timestamps: Timestamps,
}

/// The ID of an [`OrganizationDomain`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct OrganizationDomainId(String);

impl Display for OrganizationDomainId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for OrganizationDomainId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for OrganizationDomainId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

/// The state of an [`OrganizationDomain`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrganizationDomainState {
    /// The domain is pending verification.
    Pending,

    /// The domain is verified.
    Verified,

    /// The domain verification failed.
    Failed,

    /// The domain was verified using the legacy verification method.
    LegacyVerified,
}

/// The verification strategy for an [`OrganizationDomain`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationStrategy {
    /// Manual verification through the API or Dashboard.
    Manual,

    /// DNS verification using a verification token.
    Dns,
}

/// [WorkOS Docs: Organization Domain](https://workos.com/docs/reference/organization-domain)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationDomain {
    /// The ID of the organization domain.
    pub id: OrganizationDomainId,

    /// The ID of the organization this domain belongs to.
    pub organization_id: OrganizationId,

    /// The domain.
    pub domain: String,

    /// The state of the domain.
    pub state: OrganizationDomainState,

    /// The verification strategy for the domain.
    pub verification_strategy: VerificationStrategy,

    /// The verification token for the domain.
    /// When the verification strategy is DNS, this token must be present in a DNS record to verify the Organization Domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_token: Option<String>,
}
