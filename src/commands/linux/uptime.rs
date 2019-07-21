//! Command to get uptime on a Mac OS X box

use chrono::offset::{Local, TimeZone};
use regex::Regex;
use std::{fs::File, io::Read};

/// On Linux, reads `/proc/uptime`.  The `/proc/uptime` file is formatted
/// as two decimal integers, the first represents the total number of seconds
/// the system has been online, the second represents the sum of how much
/// time each core has spent idle.  In multi-core systems, the second number
/// may be larger than the first.
///
/// Example:
/// 3600.32 4321.0
///
/// This means the system has been online for just over 3600 seconds, or
/// 1 hour, while the sum of the time the cores have spent idle is
/// 4321 seconds, or 1 hour, 12 minutes
///
/// Returns the number of seconds since last boot
pub fn uptime() -> u64 {
    // Read /proc/uptime
    if let Ok(mut f) = File::open("/proc/uptime") {
        let mut contents = String::new();
        if let Ok(_) = f.read_to_string(&mut contents) {
            // Parse command
            let re =
                Regex::new(r"(?P<ups>\d+).(?P<upu>\d+)\s(?P<idles>\d+).(?P<idleu>\d+)").unwrap();

            let caps = re.captures(&contents).unwrap();
            let uptime: u64 = caps["ups"].parse().unwrap_or(0);

            return uptime;
        }
    }

    0
}
