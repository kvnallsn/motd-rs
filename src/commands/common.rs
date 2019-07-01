//! All commands that will work on every operating system

use crate::error::MotdResult;
use std::collections::HashMap;

// Returns the IPs associated with this device
pub fn interfaces(_args: Option<String>) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    let interfaces = pnet_datalink::interfaces();
    for interface in interfaces {
        let mut addrs: Vec<String> = Vec::new();
        for ip in interface.ips {
            if ip.is_ipv4() {
                addrs.push(format!("{}, ", ip.ip()));
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
