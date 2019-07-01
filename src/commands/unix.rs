//! Unix-family specific commands.  Any function in this file should run on ALL
//! Unix variants (Mac OS X, Linux, FreeBSD, OpenBSD, etc)

use std::collections::HashSet;

/// Returns a list of logged in users
pub fn users(args: Option<String>) -> HashSet<String> {
    let mut set: HashSet<String> = HashSet::new();

    let output = cmd!("users", args);

    if let Ok(output) = output {
        for user in output.split_whitespace() {
            if !user.starts_with('#') {
                set.insert(user.to_owned());
            }
        }
    }

    set
}

/// Runs the fortune command
pub fn fortune(_args: Option<String>) -> String {
    let output = cmd!("fortune", Some("-a"));

    if let Ok(o) = output {
        o.trim().to_string()
    } else {
        "No Fortune".to_string()
    }
}
