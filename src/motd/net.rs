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
    /// Arguments: Comma-separated list of strings
    /// * `name_only` - Only show interface names, not IP addresses
    /// * `addr_only` - Only show interface ips, not names
    /// * `hide_loopback` - Hides the loopback address
    /// * `hide_private` - Hide all private ips (10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16)
    /// * `hide_public` - Only show private ips
    ///
    pub fn ips(&self, args: Option<&str>) -> String {
        let mut s = String::new();

        let mut show_name = true;
        let mut show_addr = true;
        let mut hide_loopback = false;
        let mut hide_private = false;
        let mut hide_public = false;

        if let Some(arg_str) = args {
            for arg in arg_str.split(',') {
                if arg == "name_only" {
                    show_addr = false;
                }

                if arg == "addr_only" {
                    show_name = false;
                }

                if arg == "hide_loopback" {
                    hide_loopback = true;
                }

                if arg == "hide_private" {
                    hide_private = true;
                }

                if arg == "hide_public" {
                    hide_public = true;
                }
            }
        }

        let interfaces = commands::interfaces(hide_loopback, hide_public, hide_private);
        for (name, ips) in interfaces.iter() {
            if ips.len() > 0 {
                if show_name {
                    s.push_str(&format!("[{}]", name));
                }

                if show_addr {
                    if show_name {
                        s.push_str(": ");
                    }

                    for ip in ips {
                        s.push_str(&format!("{}", ip));
                    }
                } else {
                    s.push_str(", ")
                }
            }
        }

        s
    }

    /// Returns a formatted string listing the number of listening connections
    /// and the number of established connections
    pub fn connections(&self) -> String {
        let (listen, established) = commands::connections(None).unwrap_or((0, 0));
        format!("{} listening, {} established", listen, established)
    }
}
