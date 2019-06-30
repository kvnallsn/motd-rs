//! Represts how long a system has been up (time since last reboot)

use chrono::{
    offset::{Local, TimeZone},
    DateTime,
};
use regex::Regex;
use std::{fmt, process::Command};

const SECONDS_PER_WEEK: u64 = 604800;
const SECONDS_PER_DAY: u64 = 86400;
const SECONDS_PER_HOUR: u64 = 3600;
const SECONDS_PER_MINUTE: u64 = 60;

/// System Uptime
#[derive(Default)]
pub struct Uptime {
    pub weeks: u64,
    pub days: u64,
    pub hours: u64,
    pub minutes: u64,
    pub seconds: u64,
}

impl Uptime {
    pub fn new(now: &DateTime<Local>) -> Uptime {
        let mut uptime = Uptime::default();

        let re = Regex::new(r".*\{\s*sec\s*=\s*(?P<secs>\d+),\s*usec\s*=\s*(?P<usecs>\d+)\s*\}")
            .unwrap();

        let output = Command::new("sysctl")
            .arg("kern.boottime")
            .output()
            .map(|out| out.stdout)
            .map(|out| String::from_utf8(out).unwrap());

        if let Ok(o) = output {
            if let Some(caps) = re.captures(&o) {
                let secs: i64 = match caps.name("secs") {
                    Some(s) => s.as_str().parse().unwrap_or(0),
                    None => 0,
                };

                let usecs: u32 = match caps.name("usecs") {
                    Some(u) => u.as_str().parse().unwrap_or(0),
                    None => 0,
                };

                //let naive = NaiveDateTime::from_timestamp(secs, usecs);
                let naive = Local.timestamp(secs, usecs);
                let elapsed = Local::now() - naive;

                let mut seconds = elapsed.num_seconds() as u64;
                if seconds > SECONDS_PER_WEEK {
                    uptime.weeks = seconds / SECONDS_PER_WEEK;
                    seconds -= uptime.weeks * SECONDS_PER_WEEK;
                }

                if seconds > SECONDS_PER_DAY {
                    uptime.days = seconds / SECONDS_PER_DAY;
                    seconds -= uptime.days * SECONDS_PER_DAY;
                }

                if seconds > SECONDS_PER_HOUR {
                    uptime.hours = seconds / SECONDS_PER_HOUR;
                    seconds -= uptime.hours * SECONDS_PER_HOUR;
                }

                if seconds > SECONDS_PER_MINUTE {
                    uptime.minutes = seconds / SECONDS_PER_MINUTE;
                    seconds -= uptime.minutes * SECONDS_PER_MINUTE;
                }

                uptime.seconds = seconds;
            }
        }

        uptime
    }
}

impl fmt::Display for Uptime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} weeks, {} days, {} hours, {} minutes, {} seconds",
            self.weeks, self.days, self.hours, self.minutes, self.seconds
        )
    }
}
