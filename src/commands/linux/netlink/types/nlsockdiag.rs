//! Unix socket related functions

use super::super::{
    AddressFamily, L4Protocol, NetlinkFamily, NetlinkRequest, NetlinkResponse, NetlinkSocket,
    NlGetFlag, NlMsgHeader, NlMsgType,
};
use std::mem;

/// Requests specific information about unix sockets
#[repr(C)]
#[derive(Clone, Debug)]
pub struct UnixSocketRequest {
    hdr: NlMsgHeader,
    msg: NlUnixDiagReq,
}

impl UnixSocketRequest {
    /// Creates a new unix socket request that can be sent over
    /// a NETLINK socket
    pub fn new() -> UnixSocketRequest {
        let mut req = UnixSocketRequest {
            hdr: NlMsgHeader::new(
                NlMsgType::SockDiagByFamily,
                flags!(NlGetFlag::Dump),
                std::mem::size_of::<NlUnixDiagReq>() as u32,
            ),
            msg: NlUnixDiagReq::default(),
        };

        req
    }

    /// Sets an attribute to respond with on the request
    ///
    /// # Arguments
    ///
    /// * `s` - Attribute to add to request
    pub fn attribute(mut self, attr: UnixDiagState) -> UnixSocketRequest {
        self.msg.show |= attr.as_u32();
        self
    }

    /// Sets what information to return in this request
    ///
    /// # Arguments
    ///
    /// v - Vector of different attribtes/information to return
    pub fn attributes(mut self, v: Vec<UnixDiagState>) -> UnixSocketRequest {
        self.msg.show |= v.iter().fold(0, |acc, s| acc | s.as_u32());
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub enum UnixDiagState {
    ShowName = 0x01 as isize,
    ShowVfs = 0x02 as isize,
    ShowPeer = 0x04 as isize,
    ShowIcons = 0x08 as isize,
    ShowRQLen = 0x10 as isize,
    ShowMemInfo = 0x20 as isize,
    Shutdown = 0x21 as isize,
}

impl UnixDiagState {
    pub fn as_u32(&self) -> u32 {
        *self as u32
    }
}

impl From<u32> for UnixDiagState {
    fn from(u: u32) -> UnixDiagState {
        match u {
            0x01 => UnixDiagState::ShowName,
            0x02 => UnixDiagState::ShowVfs,
            0x04 => UnixDiagState::ShowPeer,
            0x08 => UnixDiagState::ShowIcons,
            0x10 => UnixDiagState::ShowRQLen,
            0x20 => UnixDiagState::ShowMemInfo,
            0x21 => UnixDiagState::Shutdown,
            _ => panic!("Unknown State"),
        }
    }
}

/// C-representation of the unix diagnostic request
#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct NlUnixDiagReq {
    /// Should be AF_UNIX
    family: u8,

    protocol: u8,

    /// Zero's
    pad: u16,

    /// Bit mask of socket states.  Supported values:
    /// * `1 << TCP_ESTABLISHED`
    /// * `1 << TCP_LISTEN`
    states: u32,

    /// inode number when querying for an individual socket, ignored when
    /// querying for a list of sockets
    ino: u32,

    /// What information to report back:
    ///
    /// Accepted:
    /// * `UDIAG_SHOW_NAME`
    /// * `UDIAG_SHOW_VFS`
    /// * `UDIAG_SHOW_PEER`
    /// * `UDIAG_SHOW_RQLEN
    /// * `UDIAG_SHOW_MEMINFO`
    /// * `UDIAG_SHUTDOWN
    show: u32,

    /// Array of opaque identifiers that could be used alongside `ino` to
    /// specify and individual socket. Ignored when querying for a list,
    /// or when set to [-1, -1]
    cookie: [u32; 2],
}

impl std::default::Default for NlUnixDiagReq {
    fn default() -> NlUnixDiagReq {
        NlUnixDiagReq {
            family: AddressFamily::Unix as u8,
            protocol: 0,
            pad: 0,
            states: 0,
            ino: 0,
            show: 0,
            cookie: [0, 0],
        }
    }
}
