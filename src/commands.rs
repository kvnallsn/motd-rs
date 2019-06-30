//! All available commands (filters) that can be used to replaced captured information

use chrono::{
    naive::NaiveDateTime,
    offset::{Local, TimeZone},
};
use regex::Regex;
use std::process::Command;

/// Generates the current date and returns a formatted string
pub fn date(args: Option<String>) -> String {
    let local = Local::now();
    let format_str = args.unwrap_or("%Y-%m-%d %H:%M:%S".to_owned());
    local.format(&format_str).to_string()
}

/// Generates the number of users currently logged in
pub fn users(args: Option<String>) -> String {
    format!("{} users logged in", 1)
}

/// Returns the local machine IP addressses
pub fn ipaddr(args: Option<String>) -> String {
    let mut addrs = String::new();

    let interfaces = pnet::datalink::interfaces();
    for interface in interfaces {
        let mut should_add = false;

        let mut s = String::new();
        s.push_str(&format!("{}: ", interface.name));
        for ip in interface.ips {
            if ip.is_ipv4() {
                s.push_str(&format!("{}, ", ip));
                should_add = true;
            }
        }

        if should_add {
            addrs.push_str(&s);
        }
    }

    addrs
}

// Returns number of listening and established connections
pub fn net(args: Option<String>) -> String {
    let listen_re = Regex::new("LISTEN").unwrap();
    let established_re = Regex::new("ESTABLISHED").unwrap();

    let output = Command::new("lsof")
        .arg("-nP")
        .arg("-i4TCP")
        .output()
        .map(|out| out.stdout)
        .map(|out| String::from_utf8(out).unwrap());

    if let Ok(o) = output {
        let listen_count = listen_re.find_iter(&o).count();
        let established_count = established_re.find_iter(&o).count();

        format!(
            "{} listening, {} established",
            listen_count, established_count
        )
    } else {
        format!("none")
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
