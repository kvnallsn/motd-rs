//! Returns information on established vs listening connections

use crate::error::MotdResult;
use regex::Regex;

// Returns number of listening and established connections (IPv4 TCP only)
//
// Command: `lsof -nP -i4TCP'
pub fn connections(_args: Option<String>) -> MotdResult<(usize, usize)> {
    let listen_re = Regex::new("LISTEN")?;
    let established_re = Regex::new("ESTABLISHED")?;

    let output = cmd!("lsof", Some("-nP -i4TCP"))?;
    let listen_count = listen_re.find_iter(&output).count();
    let established_count = established_re.find_iter(&output).count();

    Ok((listen_count, established_count))
}
