//! Unix socket related functions

use crate::commands::linux::netlink::{
    sockdiag::MemInfo, AddressFamily, NetlinkAttribute, NetlinkFamily, NetlinkRequest,
    NetlinkResponse, NetlinkSocket, NlGetFlag, NlMsgHeader, NlMsgType,
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

    /// Sets an RequestAttribute to respond with on the request
    ///
    /// # Arguments
    ///
    /// * `s` - RequestAttribute to add to request
    pub fn attribute(mut self, attr: RequestAttribute) -> Request {
        self.msg.show |= attr.as_u32();
        self
    }

    /// Sets what information to return in this request
    ///
    /// # Arguments
    ///
    /// v - Vector of different attribtes/information to return
    pub fn attributes(mut self, v: Vec<RequestAttribute>) -> Request {
        self.msg.show |= v.iter().fold(0, |acc, s| acc | s.as_u32());
        self
    }
}

impl NetlinkRequest for Request {
    /// Returns the family/kernel module to use for this request
    fn family(&self) -> NetlinkFamily {
        NetlinkFamily::SockDiag
    }
}

/// Represents an attribute than be added to a given request, returning
/// the requested information in a NETLINK attribute (rtattr) Structure
#[derive(Clone, Copy, Debug)]
pub enum RequestAttribute {
    /// Show name of socket (not path)
    ShowName = 0x01 as isize,

    /// Show VFS (Virtual File System) inode information
    ShowVfs = 0x02 as isize,

    /// Show peer socket information
    ShowPeer = 0x04 as isize,

    /// Show pending connections,
    ShowIcons = 0x08 as isize,

    /// Show skb receive queue length
    ShowRQLen = 0x10 as isize,

    /// Show memory info of a socket
    ShowMemInfo = 0x20 as isize,
}

impl RequestAttribute {
    pub fn as_u32(&self) -> u32 {
        *self as u32
    }
}

impl From<u32> for RequestAttribute {
    fn from(u: u32) -> RequestAttribute {
        match u {
            0x01 => RequestAttribute::ShowName,
            0x02 => RequestAttribute::ShowVfs,
            0x04 => RequestAttribute::ShowPeer,
            0x08 => RequestAttribute::ShowIcons,
            0x10 => RequestAttribute::ShowRQLen,
            0x20 => RequestAttribute::ShowMemInfo,
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
            states: 0xffffffff,
            ino: 0,
            show: 0,
            cookie: [0, 0],
        }
    }
}

/// All possible attributes that can appear in a unix socket response
#[doc(hidden)]
const RESP_ATTR_NAME: u16 = 0x00;
const RESP_ATTR_VFS: u16 = 0x01;
const RESP_ATTR_PEER: u16 = 0x02;
const RESP_ATTR_ICONS: u16 = 0x03;
const RESP_ATTR_RQLEN: u16 = 0x04;
const RESP_ATTR_MEMINFO: u16 = 0x05;
const RESP_ATTR_SHUTDOWN: u16 = 0x06;

/// Response from submitting an NlUnixDiagReq message
#[derive(Clone, Debug)]
pub struct Response {
    /// Address family this socket belongs to (should be Unix)
    family: AddressFamily,

    ty: u8,
    state: u8,
    pad: u8,
    ino: u32,
    cookie: [u32; 2],

    // attributes are below here
    /// Pathname to which this socket is bound
    name: Option<String>,

    /// Virtual file system information
    vfs: Option<Vfs>,

    /// inode associated with this socket's peer
    /// only reported for connected sockets
    peer: Option<u32>,

    /// inode numbers of sockets that have passed the `connect(2)` call
    /// but haven't been processed with `accept(2)` yet. Only reported
    /// for listening sockets
    icons: Option<Vec<u32>>,

    /// Read and write queue information
    queue: Option<Queue>,

    /// Memory information for this socket
    mem: Option<MemInfo>,

    /// Internal shutdown state of socket
    shutdown: Option<u8>,
}

impl Response {
    /// Creates a new response, extracting values from a buffer `v`
    ///
    /// # Arguments
    ///
    /// * `v` - Buffer to build response from
    pub fn new(v: &mut Vec<u8>) -> Response {
        let mut resp = Response {
            family: AddressFamily::from(u8!(v)),
            ty: u8!(v),
            state: u8!(v),
            pad: u8!(v),
            ino: u32!(v),
            cookie: [u32!(v), u32!(v)],

            // Initialze all attributes to None by default
            name: None,
            vfs: None,
            peer: None,
            icons: None,
            queue: None,
            mem: None,
            shutdown: None,
        };

        while let Some(mut attr) = NetlinkAttribute::new(v) {
            if attr.ty == RESP_ATTR_NAME {
                // Name Attribute

                // consumes the NULL byte on the end
                let _ = attr.data.pop();

                // Converts a cstring into a Rust String
                if let Ok(cstr) = std::ffi::CString::new(attr.data) {
                    resp.name = cstr.into_string().ok();
                }
            } else if attr.ty == RESP_ATTR_VFS {
                if attr.data.len() >= 8 {
                    resp.vfs = Some(Vfs::new(u32!(attr.data), u32!(attr.data)));
                }
            } else if attr.ty == RESP_ATTR_PEER {
                if attr.data.len() >= 4 {
                    resp.peer = Some(u32!(attr.data));
                }
            } else if attr.ty == RESP_ATTR_ICONS {
                let mut inodes = vec![];
                while attr.data.len() > 4 {
                    inodes.push(u32!(attr.data));
                }

                if inodes.len() > 0 {
                    resp.icons = Some(inodes);
                }
            } else if attr.ty == RESP_ATTR_RQLEN {
                if attr.data.len() >= 8 {
                    resp.queue = Some(Queue::new(u32!(attr.data), u32!(attr.data)));
                }
            } else if attr.ty == RESP_ATTR_MEMINFO {
                if attr.data.len() >= 32 {
                    resp.mem = Some(MemInfo::new(&mut attr.data));
                }
            } else if attr.ty == RESP_ATTR_SHUTDOWN {
                // Shutdown State
                resp.shutdown = Some(u8!(attr.data));
            }
        }

        resp
    }
}

/// Virtual File System information about this Unix socket
#[derive(Clone, Debug)]
pub struct Vfs {
    /// The device number of the corresponding on-disk socket inode
    pub device_number: u32,

    /// The inode number of the corresponding on-disk seocket inode
    pub inode: u32,
}

impl Vfs {
    /// Creates a new VFS structure with the provided information
    ///
    /// # Arguments
    ///
    /// * `dev` - device number
    /// * `inode` - inode Number
    pub fn new(dev: u32, inode: u32) -> Vfs {
        Vfs {
            device_number: dev,
            inode,
        }
    }
}

/// Read and write queue information for listening and established sockets
#[derive(Clone, Debug)]
pub struct Queue {
    /// For listening sockets: Number of pending connections
    ///
    /// For established sockets: Amount of data in incoming queue
    pub read: u32,

    /// For listening sockets: Backlog length wich equals the value
    /// passed as the second argument to `listen(2)`
    ///
    /// For established sockets: Amount of memory available for sending
    pub write: u32,
}

impl Queue {
    /// Creates a new queue structe with the provided information
    ///
    /// # Arguments
    ///
    /// * `read` - See docs on read field
    /// * `write` - See docs onw write field
    pub fn new(read: u32, write: u32) -> Queue {
        Queue { read, write }
    }
}
