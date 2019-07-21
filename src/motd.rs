//! Representation of an motd itself

use crate::commands;
use chrono::{offset::Local, DateTime};
use regex::{Captures, Regex};
use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

mod net;
mod system;
mod user;

pub struct Motd {
    /// Information about the current user
    pub user: user::User,
    pub net: net::Net,
    pub sys: system::System,
    pub date: DateTime<Local>,
}

impl Motd {
    pub fn new() -> Motd {
        let now = Local::now();
        Motd {
            user: user::User::new(),
            net: net::Net::new(),
            sys: system::System::new(),
            date: now,
        }
    }

    /// Renders an Message of the Day Template
    ///
    /// # Arguments
    ///
    /// * `path` - Path to MotD template
    pub fn render<P: AsRef<Path>>(&self, path: P) -> Result<String, io::Error> {
        // Regex to find all commands to substitute
        let re = Regex::new(r"\{\{ (?P<cmd>[[:alpha:]]+)(\((?P<args>.*)\))? \}\}").unwrap();

        // Read template file in
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // Replace all command occurances
        let result = re.replace_all(&contents, |caps: &Captures| self.replace(caps));

        Ok(result.to_string())
    }

    /// Performs the replacement for each successfull capture
    ///
    /// # Arguments
    ///
    /// * `cap` - The current captured value, available via the named-group "cmd"
    fn replace(&self, caps: &Captures) -> String {
        let cmd = caps.name("cmd").unwrap();
        let args = caps.name("args").map(|m| m.as_str());
        match cmd.as_str() {
            "user" => self.user.name.clone(),
            "tty" => self.user.tty.clone(),
            "date" => self.date(args),
            "uptime" => self.sys.uptime(),
            "hostname" => self.net.hostname(),
            "users" => self.sys.users(),
            "ipaddr" => self.net.ips(args),
            "conns" => self.net.connections(),
            "process" => self.sys.processes(),
            "fortune" => commands::fortune(None),
            _ => panic!("Unrecognized command!"),
        }
    }

    /// Returns the current date and time, formatted as specified by the
    /// user, or via the default format
    pub fn date(&self, fmt: Option<&str>) -> String {
        self.date
            .format(&fmt.unwrap_or("%a, %d %b %Y %T %z"))
            .to_string()
    }
}
