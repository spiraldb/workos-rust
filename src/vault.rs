//! Vault API for managing encrypted objects.
//!
//! [WorkOS Docs: Vault](https://workos.com/docs/reference/vault)

mod operations;
mod types;

pub use operations::*;
pub use types::*;

use crate::WorkOs;

/// Vault API for managing encrypted objects.
///
/// [WorkOS Docs: Vault](https://workos.com/docs/reference/vault)
pub struct Vault<'a> {
    workos: &'a WorkOs,
}

impl<'a> Vault<'a> {
    /// Creates a new `Vault` instance.
    pub fn new(workos: &'a WorkOs) -> Self {
        Self { workos }
    }
}
