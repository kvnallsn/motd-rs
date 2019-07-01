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

use std::io;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;

#[cfg(target_os = "macos")]
mod osx;
#[cfg(target_os = "macos")]
pub use osx::*;

#[cfg(target_family = "unix")]
mod unix;
#[cfg(target_family = "unix")]
pub use unix::*;

/// Place all commands that will run across all oses here
mod common;
pub use common::*;

/// Returns the hostname of this computer
pub fn hostname(args: Option<String>) -> Result<String, io::Error> {
    cmd!("hostname", args)
}

/// Returns the currently logged in user
pub fn user(args: Option<String>) -> Result<String, io::Error> {
    cmd!("whoami", args)
}
