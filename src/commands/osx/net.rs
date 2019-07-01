//! Returns information on established vs listening connections

use regex::Regex;
use std::collections::HashMap;

// Returns number of listening and established connections
pub fn connections(_args: Option<String>) -> (usize, usize) {
    let listen_re = Regex::new("LISTEN").unwrap();
    let established_re = Regex::new("ESTABLISHED").unwrap();

    if let Ok(o) = cmd!("lsof", Some("-nP -i4TCP")) {
        let listen_count = listen_re.find_iter(&o).count();
        let established_count = established_re.find_iter(&o).count();

        (listen_count, established_count)
    } else {
        (0, 0)
    }
}

// Returns the IPs associated with this device
pub fn interfaces(_args: Option<String>) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    let interfaces = pnet_datalink::interfaces();
    for interface in interfaces {
        let mut addrs: Vec<String> = Vec::new();
        for ip in interface.ips {
            if ip.is_ipv4() {
                addrs.push(format!("{}, ", ip));
            }
        }

        map.insert(interface.name, addrs);
    }

    map
}
