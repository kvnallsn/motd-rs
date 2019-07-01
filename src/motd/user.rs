//! All information regarding the user that logged in, either via tty, console, etc.

use crate::commands;

/// Represents the logged in user.  Contains information like the username,
/// tty or where the user is logged in, etc.
#[derive(Default)]
pub struct User {
    /// User's username
    pub name: String,

    /// User's tty
    pub tty: String,
}

impl User {
    /// Creates a new user, gleaning information from the system
    pub fn new() -> User {
        let mut user = User::default();

        if let Ok(name) = commands::user(None) {
            user.name = name;
        }

        user
    }
}
