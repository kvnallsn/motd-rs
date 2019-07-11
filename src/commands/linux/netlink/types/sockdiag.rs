//! All functions/structs/etc in this module correspond to functionality the socket diagnostics
//! interface for NETLINK.  For more information, see sock_diag(7)

pub mod inet;
pub mod unix;

use crate::commands::linux::netlink::AddressFamily;

#[derive(Clone, Debug)]
pub enum Response {
    None,
    Inet(inet::Response),
    Unix(unix::Response),
}

impl Response {
    /// Reads a response from the buffer and returns the appropriate
    /// response type: `inet::Response` for IPv4 and IPv6 sockets and
    /// `unix::Response` for Unix sockets
    ///
    /// # Arguments
    ///
    /// * `v` - Buffer to build response from
    pub fn new(v: &mut Vec<u8>) -> Response {
        let family = AddressFamily::from(v[0]);

        match family {
            AddressFamily::Inet | AddressFamily::Inet6 => Response::Inet(inet::Response::new(v)),
            AddressFamily::Unix => Response::Unix(unix::Response::new(v)),
            _ => Response::None, // Encounted an Unknown family type
        }
    }
}
