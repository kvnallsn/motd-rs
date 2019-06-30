//! Networking related information

use std::{fmt, process::Command};

/// All networking-related fields, to include interfaces,
/// IP addresses, hostnames, etc.
#[derive(Default)]
pub struct Net {
    pub hostname: String,
}

impl Net {
    pub fn new() -> Net {
        let mut net = Net::default();

        let output = Command::new("hostname")
            .output()
            .map(|out| out.stdout)
            .map(|out| String::from_utf8(out).unwrap());

        if let Ok(o) = output {
            net.hostname = o.trim().to_string();
        }

        net
    }
}

impl fmt::Display for Net {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.hostname)
    }
}
