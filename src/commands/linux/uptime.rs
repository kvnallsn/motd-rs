//! Command to get uptime on a Mac OS X box

use chrono::offset::{Local, TimeZone};
use regex::Regex;

/// On Mac OS X, executes `sysctl kern.boottime` which returns the
/// UNIX timestamp of when this box last booted (or restarted). This
/// function converts that timestamp to a local time and subtracts it
/// from the current time to get the number of seconds since the box
/// last booted.
///
/// Command: `sysctl kern.boottime`
pub fn uptime() -> u64 {
    // Read /proc/uptime

    0
}
