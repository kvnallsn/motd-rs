//! Command to get uptime on a Mac OS X box

use chrono::offset::{Local, TimeZone};
use regex::Regex;

/// On Mac OS X, executes `sysctl kern.boottime` which returns the
/// UNIX timestamp of when this box last booted (or restarted). This
/// function converts that timestamp to a local time and subtracts it
/// from the current time to get the number of seconds since the box
/// last booted.
pub fn uptime() -> u64 {
    let re =
        Regex::new(r".*\{\s*sec\s*=\s*(?P<secs>\d+),\s*usec\s*=\s*(?P<usecs>\d+)\s*\}").unwrap();

    if let Ok(output) = cmd!("sysctl", Some("kern.boottime")) {
        if let Some(caps) = re.captures(&output) {
            let secs: i64 = match caps.name("secs") {
                Some(s) => s.as_str().parse().unwrap_or(0),
                None => 0,
            };

            let usecs: u32 = match caps.name("usecs") {
                Some(u) => u.as_str().parse().unwrap_or(0),
                None => 0,
            };

            let naive = Local.timestamp(secs, usecs);
            return (Local::now() - naive).num_seconds() as u64;
        }
    }

    0
}
