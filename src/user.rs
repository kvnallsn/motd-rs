//! All information regarding the user that logged in, either via tty, console, etc.

use regex::Regex;
use std::process::Command;

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

        let output = Command::new("whoami")
            .output()
            .map(|out| out.stdout)
            .map(|out| String::from_utf8(out).unwrap());

        if let Ok(o) = output {
            user.name = o.trim().to_string();
        }

        let output = Command::new("tty")
            .output()
            .map(|out| out.stdout)
            .map(|out| String::from_utf8(out).unwrap());

        if let Ok(o) = output {
            user.tty = o.trim().to_string();
        }

        user
    }
}
