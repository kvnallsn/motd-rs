//! All available commands (filters) that can be used to replaced captured information

macro_rules! cmd {
    ($command:expr,$args:expr) => {{
        let mut cmd = std::process::Command::new($command);
        if let Some(args) = $args {
            cmd.args(args.split(' '));
        }

        cmd.output()
            .map(|out| out.stdout)
            .map(|out| String::from_utf8(out).unwrap())
            .map(|s| s.trim().to_string())
    }};
}

use std::{
    collections::{HashMap, HashSet},
    io,
    process::Command,
};

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "macos")]
mod osx;

#[cfg(target_family = "unix")]
mod unix;

/// Returns a list of logged in user's usernames
pub fn users(args: Option<String>) -> HashSet<String> {
    if cfg!(target_family = "unix") {
        unix::users(args)
    } else {
        HashSet::new()
    }
}

/// Returns the hostname of this computer
pub fn hostname(args: Option<String>) -> Result<String, io::Error> {
    cmd!("hostname", args)
}

/// Returns the number of seconds since the box last restarted/booted
pub fn uptime() -> u64 {
    if cfg!(target_os = "macos") {
        osx::uptime()
    } else {
        0
    }
}

/// Returns the currently logged in user
pub fn user(args: Option<String>) -> Result<String, io::Error> {
    cmd!("whoami", args)
}

/// Returns the local machine IP addressses
pub fn interfaces(args: Option<String>) -> HashMap<String, Vec<String>> {
    if cfg!(target_os = "macos") {
        osx::interfaces(args)
    } else {
        HashMap::new()
    }
}

// Returns number of listening and established connections
pub fn connections(args: Option<String>) -> (usize, usize) {
    if cfg!(target_os = "macos") {
        osx::connections(args)
    } else {
        (0, 0)
    }
}

pub fn fortune(args: Option<String>) -> String {
    let mut cmd = Command::new("fortune");

    if let Some(args) = args {
        cmd.args(args.split(' '));
    }

    let output = cmd
        .output()
        .map(|out| out.stdout)
        .map(|out| String::from_utf8(out).unwrap());

    if let Ok(o) = output {
        o.trim().to_string()
    } else {
        "No Fortune".to_string()
    }
}
