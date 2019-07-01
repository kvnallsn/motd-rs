//! Networking related information

use crate::commands;

/// All networking-related fields, to include interfaces,
/// IP addresses, hostnames, etc.
#[derive(Default)]
pub struct Net;

impl Net {
    pub fn new() -> Net {
        Net::default()
    }

    /// Returns the hostname for this computer, or localhost if
    /// a hostname is not set or the command fails
    pub fn hostname(&self) -> String {
        match commands::hostname(None) {
            Ok(host) => host,
            Err(_) => "localhost".to_string(),
        }
    }

    /// Returns a formatted list of IPs assocated with network interfaces
    ///
    /// For example, a computer with 2 interfaces (lo0, en0) with 1 IP would be:
    /// [lo0] 127.0.0.1, [en0] 192.168.1.10,
    pub fn ips(&self) -> String {
        let mut s = String::new();

        let interfaces = commands::interfaces(None);
        for (name, ips) in interfaces.iter() {
            if ips.len() > 0 {
                s.push_str(&format!("[{}]: ", name));

                for ip in ips {
                    s.push_str(&format!("{}", ip));
                }
            }
        }

        s
    }

    /// Returns a formatted string listing the number of listening connections
    /// and the number of established connections
    pub fn connections(&self) -> String {
        let (listen, established) = commands::connections(None);
        format!("{} listening, {} established", listen, established)
    }
}
