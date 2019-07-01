//! Returns information on established vs listening connections

use regex::Regex;

// Returns number of listening and established connections
pub fn connections(_args: Option<String>) -> (usize, usize) {
    let listen_re = Regex::new("LISTEN").unwrap();
    let established_re = Regex::new("ESTABLISHED").unwrap();

    if let Ok(o) = cmd!("lsof", Some("-nP -i4TCP")) {
        let listen_count = listen_re.find_iter(&o).count();
        let established_count = established_re.find_iter(&o).count();

        (listen_count, established_count)
    } else {
        (0, 0)
    }
}
