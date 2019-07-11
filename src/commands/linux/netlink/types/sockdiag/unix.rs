//! Unix socket related functions

use crate::commands::linux::netlink::{
    AddressFamily, L4Protocol, NetlinkFamily, NetlinkRequest, NetlinkResponse, NetlinkSocket,
    NlGetFlag, NlMsgHeader, NlMsgType,
};
use std::mem;

/// Requests specific information about unix sockets
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Request {
    hdr: NlMsgHeader,
    msg: NlUnixDiagReq,
}

impl Request {
    /// Creates a new unix socket request that can be sent over
    /// a NETLINK socket
    pub fn new() -> Request {
        let mut req = Request {
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
    pub fn attribute(mut self, attr: Attribute) -> Request {
        self.msg.show |= attr.as_u32();
        self
    }

    /// Sets what information to return in this request
    ///
    /// # Arguments
    ///
    /// v - Vector of different attribtes/information to return
    pub fn attributes(mut self, v: Vec<Attribute>) -> Request {
        self.msg.show |= v.iter().fold(0, |acc, s| acc | s.as_u32());
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Attribute {
    ShowName = 0x01 as isize,
    ShowVfs = 0x02 as isize,
    ShowPeer = 0x04 as isize,
    ShowIcons = 0x08 as isize,
    ShowRQLen = 0x10 as isize,
    ShowMemInfo = 0x20 as isize,
    Shutdown = 0x21 as isize,
}

impl Attribute {
    pub fn as_u32(&self) -> u32 {
        *self as u32
    }
}

impl From<u32> for Attribute {
    fn from(u: u32) -> Attribute {
        match u {
            0x01 => Attribute::ShowName,
            0x02 => Attribute::ShowVfs,
            0x04 => Attribute::ShowPeer,
            0x08 => Attribute::ShowIcons,
            0x10 => Attribute::ShowRQLen,
            0x20 => Attribute::ShowMemInfo,
            0x21 => Attribute::Shutdown,
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

/// Response from submitting an NlUnixDiagReq message
#[derive(Clone, Debug)]
pub struct Response {
    family: AddressFamily,
    ty: u8,
    state: u8,
    pad: u8,
    ino: u32,
    cookie: [u32; 2],
}

impl Response {
    /// Creates a new response, extracting values from a buffer `v`
    ///
    /// # Arguments
    ///
    /// * `v` - Buffer to build response from
    pub fn new(v: &mut Vec<u8>) -> Response {
        Response {
            family: AddressFamily::from(u8!(v)),
            ty: u8!(v),
            state: u8!(v),
            pad: u8!(v),
            ino: u32!(v),
            cookie: [u32!(v), u32!(v)],
        }
    }
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
