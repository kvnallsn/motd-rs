//! All commands that will work on every operating system

use crate::error::MotdResult;
use std::{collections::HashMap, net::IpAddr};

// Returns the IPs associated with this device
pub fn interfaces(
    hide_loopback: bool,
    hide_public: bool,
    hide_private: bool,
) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    let interfaces = pnet_datalink::interfaces();
    for interface in interfaces {
        let mut addrs: Vec<String> = Vec::new();
        for ip in interface.ips {
            match ip.ip() {
                IpAddr::V4(ip) => {
                    if hide_loopback && ip.is_loopback() {
                        continue;
                    }

                    if hide_private && ip.is_private() {
                        continue;
                    }

                    if hide_public && !ip.is_private() {
                        continue;
                    }

                    addrs.push(format!("{}, ", ip));
                }
                IpAddr::V6(ip6) => {}
            }
        }

        map.insert(interface.name, addrs);
    }

    map
}

/// Returns the hostname of this computer
pub fn hostname(args: Option<String>) -> MotdResult<String> {
    cmd!("hostname", args)
}

/// Returns the currently logged in user
pub fn user(args: Option<String>) -> MotdResult<String> {
    cmd!("whoami", args)
}
