//! A module for interacting with users within WorkOS.

mod operations;
mod types;

pub use operations::*;
pub use types::*;

use crate::WorkOs;

/// User management.
pub struct UserManagement<'a> {
    workos: &'a WorkOs,
}

impl<'a> UserManagement<'a> {
    /// Returns a new [`UserManagement`] instance for the provided WorkOS client.
    pub fn new(workos: &'a WorkOs) -> Self {
        Self { workos }
    }
}
