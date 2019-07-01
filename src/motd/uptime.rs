//! Represts how long a system has been up (time since last reboot)

use crate::commands;
use chrono::{offset::Local, DateTime};
use std::fmt;

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
    pub fn new(_now: &DateTime<Local>) -> Uptime {
        let mut uptime = Uptime::default();

        let mut seconds = commands::uptime();

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
