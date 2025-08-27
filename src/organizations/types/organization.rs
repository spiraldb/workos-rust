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

/// [WorkOS Docs: Organization Domain](https://workos.com/docs/reference/organization-domain)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationDomain {
    /// The ID of the organization domain.
    pub id: OrganizationDomainId,

    /// The domain.
    pub domain: String,
}
