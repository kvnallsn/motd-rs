//! Networking related information

use crate::commands;
use std::{collections::HashMap, fmt};

/// All networking-related fields, to include interfaces,
/// IP addresses, hostnames, etc.
#[derive(Default)]
pub struct Net {
    pub hostname: String,
    pub listen: usize,
    pub established: usize,
    pub interfaces: HashMap<String, Vec<String>>,
}

impl Net {
    pub fn new() -> Net {
        let mut net = Net::default();

        if let Ok(hostname) = commands::hostname(None) {
            net.hostname = hostname;
        }

        let (listen, established) = commands::connections(None);
        net.listen = listen;
        net.established = established;
        net.interfaces = commands::interfaces(None);

        net
    }

    pub fn ips(&self) -> String {
        let mut s = String::new();
        for (name, ips) in self.interfaces.iter() {
            if ips.len() > 0 {
                s.push_str(&format!("[{}]: ", name));

                for ip in ips {
                    s.push_str(&format!("{}", ip));
                }
            }
        }

        s
    }
}

impl fmt::Display for Net {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.hostname)
    }
}
