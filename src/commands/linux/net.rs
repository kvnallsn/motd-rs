//! Returns information on established vs listening connections

use crate::{error::{MotdResult, Error, ParsingError}, commands::linux::netlink};
use regex::Regex;
use std::{fs::File, io::Read};

const TCP_RE_STR: &'static str = r"TCP:\s+inuse\s+(?P<inuse>\d+)\s+orphan\s+(?P<orphan>\d+)\s+tw\s+(?P<tw>\d+)\s+alloc\s+(?P<alloc>\d+)\s+mem\s+(?P<mem>\d+)";

const UDP_RE_STR: &'static str = r"UDP:\s+inuse\s+(?P<inuse>\d+)\s+mem\s+(?P<mem>\d+)";

// Returns number of listening and established connections (IPv4 TCP only)
//
// Reads from /proc/net/sockstat and /proc/net/sockstat6
pub fn connections(_args: Option<String>) -> MotdResult<(usize, usize)> {
    let mut fd = File::open("/proc/net/sockstat")?;
    let mut contents = String::new();
    fd.read_to_string(&mut contents);

    // Regex to parse
    let tcp_re = Regex::new(TCP_RE_STR)?;
    let udp_re = Regex::new(TCP_RE_STR)?;

    let tcp_caps = tcp_re.captures(&contents)
        .ok_or(ParsingError::RegexFailed)?;

    let tcp_inuse = match tcp_caps.name("inuse") {
        Some(m) => m.as_str().parse::<u32>()?,
        None => 0,
    };

    let tcp_tw = match tcp_caps.name("tw") {
        Some(m) => m.as_str().parse::<u32>()?,
        None => 0,
    };

    let udp_caps = udp_re.captures(&contents);

    netlink::socket_test();

    Ok((0, 0))
}
