use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{Timestamp, Timestamps};

/// The ID of a [`User`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct UserId(String);

impl Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for UserId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for UserId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

/// [WorkOS Docs: User](https://workos.com/docs/reference/user-management/user)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    /// The ID of the user.
    pub id: UserId,

    /// The email address of the user.
    pub email: String,

    /// The first name of the user.
    pub first_name: Option<String>,

    /// The last name of the user.
    pub last_name: Option<String>,

    /// Whether the user's email has been verified.
    pub email_verified: bool,

    /// The URL of the user's profile picture.
    pub profile_picture_url: Option<String>,

    /// The timestamp of the user's last sign in.
    pub last_sign_in_at: Option<Timestamp>,

    /// The external ID of the user.
    pub external_id: Option<String>,

    /// Additional metadata about the user.
    pub metadata: serde_json::Value,

    /// The timestamps for the user.
    #[serde(flatten)]
    pub timestamps: Timestamps,
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use super::User;

    #[test]
    fn it_deserializes_a_user_with_null_names() {
        let user: User = serde_json::from_str(
            &json!({
                "object": "user",
                "id": "user_01EHZNVPK3SFK441A1RGBFSHRT",
                "email": "user@example.com",
                "email_verified": true,
                "first_name": null,
                "last_name": null,
                "profile_picture_url": null,
                "last_sign_in_at": "2021-06-25T19:07:33.155Z",
                "external_id": null,
                "metadata": {},
                "created_at": "2021-06-25T19:07:33.155Z",
                "updated_at": "2021-06-25T19:07:33.155Z"
            })
            .to_string(),
        )
        .unwrap();

        assert_eq!(user.first_name, None);
        assert_eq!(user.last_name, None);
    }
}
