//! All system-related commands (processes, networking, etc)

use crate::commands;

const SECONDS_PER_WEEK: u64 = 604800;
const SECONDS_PER_DAY: u64 = 86400;
const SECONDS_PER_HOUR: u64 = 3600;
const SECONDS_PER_MINUTE: u64 = 60;

/// Contains commands to retrieve the system state
#[derive(Default)]
pub struct System;

impl System {
    pub fn new() -> System {
        System::default()
    }

    /// Returns how long the system has been "up".  Otherwise known as
    /// time since last reboot or power-on
    pub fn uptime(&self) -> String {
        let mut weeks = 0;
        let mut days = 0;
        let mut hours = 0;
        let mut minutes = 0;
        let mut seconds = commands::uptime();

        if seconds > SECONDS_PER_WEEK {
            weeks = seconds / SECONDS_PER_WEEK;
            seconds -= weeks * SECONDS_PER_WEEK;
        }

        if seconds > SECONDS_PER_DAY {
            days = seconds / SECONDS_PER_DAY;
            seconds -= days * SECONDS_PER_DAY;
        }

        if seconds > SECONDS_PER_HOUR {
            hours = seconds / SECONDS_PER_HOUR;
            seconds -= hours * SECONDS_PER_HOUR;
        }

        if seconds > SECONDS_PER_MINUTE {
            minutes = seconds / SECONDS_PER_MINUTE;
            seconds -= minutes * SECONDS_PER_MINUTE;
        }

        seconds = seconds;

        format!(
            "{} weeks, {} days, {} hours, {} minutes, {} seconds",
            weeks, days, hours, minutes, seconds
        )
    }

    /// Formats the string for printing the active users on the system
    pub fn users(&self) -> String {
        let mut usrs = String::new();

        let users = commands::users(None);
        for (i, user) in users.iter().enumerate() {
            usrs.push_str(user);
            if (i + 1) < users.len() {
                usrs.push_str(", ");
            }
        }

        format!("{} users ({})", users.len(), usrs)
    }
}
