//! MOTD Utility
//!
//! Parses motd templates (*.motd) and displays them to the console

use chrono::{offset::Local, DateTime};
use regex::{Captures, Regex};
use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

mod commands;
mod net;
mod uptime;
mod user;

pub struct Motd {
    /// Information about the current user
    pub user: user::User,
    pub uptime: uptime::Uptime,
    pub net: net::Net,
    pub date: DateTime<Local>,
}

impl Motd {
    pub fn new() -> Motd {
        let now = Local::now();
        Motd {
            user: user::User::new(),
            uptime: uptime::Uptime::new(&now),
            net: net::Net::new(),
            date: now,
        }
    }

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
        let args = caps.name("args").map(|m| m.as_str().to_owned());
        match cmd.as_str() {
            "user" => self.user.name.clone(),
            "tty" => self.user.tty.clone(),
            "date" => self
                .date
                .format(&args.unwrap_or("%a, %d %b %Y %T %z".to_owned()))
                .to_string(),
            "uptime" => format!("{}", self.uptime),
            "hostname" => format!("{}", self.net.hostname),
            "users" => commands::users(args),
            "ipaddr" => self.net.ips(),
            "conns" => format!(
                "{} listening, {} established",
                self.net.listen, self.net.established
            ),
            "fortune" => commands::fortune(args),
            _ => panic!("Unrecognized command!"),
        }
    }
}

fn main() {
    //let template = read_template("templates/falcon.motd").expect("failed to open file");
    //let template = process_template(template);
    let motd = Motd::new();
    let template = motd.render("templates/falcon.motd").unwrap();

    println!("{}", template);
}
