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

/// Memory information about Unix, Inet, and Inet6 sockets
#[derive(Clone, Debug, Default)]
pub struct MemInfo {
    /// Amount of data in the receive queue
    pub rmem_alloc: u32,

    /// Receive socket buffer as set by SO_RCVBUF
    pub rcv_buf: u32,

    /// Amount of data in the send queue
    pub wmem_alloc: u32,

    /// Send socket buffer as set by SO_SNDBUF
    pub snd_buf: u32,

    /// Amount of memory scheduled for future use (TCP only)
    pub fwd_alloc: u32,

    /// Amount of data queued by TCP, but not yet sent
    pub wmem_queued: u32,

    /// Amount of memory allocated for the socket's service needs (e.g., socket filter)
    pub opt_mem: u32,

    /// Amount of packets in the backlng (not yet processed)
    pub backlog: u32,
}

impl MemInfo {
    /// Creates a new meminfo structure from a buffer of information
    pub fn new(v: &mut Vec<u8>) -> MemInfo {
        MemInfo {
            rmem_alloc: u32!(v),
            rcv_buf: u32!(v),
            wmem_alloc: u32!(v),
            snd_buf: u32!(v),
            fwd_alloc: u32!(v),
            wmem_queued: u32!(v),
            opt_mem: u32!(v),
            backlog: u32!(v),
        }
    }
}
