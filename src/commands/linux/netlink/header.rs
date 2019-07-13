//! A NETLINK message header, present on all NETLINK messages

use super::flag::Flag;
use std::mem;

/// Represents the header sent on all NETLINK messages, to include the
/// NETLINK message type and other identifying information
#[repr(C)]
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct Header {
    /// Length of message including header
    nlmsg_len: u32,

    /// Type of message content
    nlmsg_type: u16,

    /// Additional flags
    nlmsg_flags: u16,

    /// Sequence number
    nlmsg_seq: u32,

    /// Sender port ID
    nlmsg_pid: u32,
}

impl Header {
    pub fn msg_type(&self) -> MessageType {
        MessageType::new(self.nlmsg_type)
    }
}

impl std::default::Default for Header {
    fn default() -> Header {
        Header {
            nlmsg_len: 0,
            nlmsg_type: 0,
            nlmsg_flags: 0,
            nlmsg_seq: 0,
            nlmsg_pid: 0,
        }
    }
}

impl Header {
    /// Creates a new NETLINK message header for the provided type, flags and size.
    ///
    /// # Arguments
    ///
    /// * `ty` - Type/Family of kernel module to talk to (e.g., SockDiagByFamily)
    /// * `size` - The size of the payload structure
    pub fn new(ty: MessageType, size: u32) -> Header {
        Header {
            nlmsg_len: size + (std::mem::size_of::<Self>() as u32),
            nlmsg_type: ty as u16,
            nlmsg_flags: Flag::Request.as_u16(),
            nlmsg_seq: 0,
            nlmsg_pid: 0,
        }
    }

    /// Set or Change the message type for this request.
    ///
    /// # Arguments
    ///
    /// * `msgTy` - Message Type/Family for this request
    pub fn ty(mut self, msg_ty: MessageType) -> Header {
        self.nlmsg_type = msg_ty as u16;
        self
    }

    /// Sets a flag in this request
    ///
    /// # Arguments
    ///
    /// * `flag` - Flag to set
    pub fn flag(mut self, flag: Flag) -> Header {
        self.nlmsg_flags |= flag.as_u16();
        self
    }

    /// Sets all flags passed in to this request
    ///
    /// # Arguments
    ///
    /// * `flags` - All flags to set in this request
    pub fn flags(mut self, flags: Vec<Flag>) -> Header {
        self.nlmsg_flags |= flags.iter().fold(0, |acc, flag| acc | flag.as_u16());
        self
    }

    /// Creates a message header from a buffer received after sending a request
    ///
    /// # Arguments
    ///
    /// * `v` - Vector/Buffer to parse into message header
    pub fn parse(v: &mut Vec<u8>) -> Option<Header> {
        let mut hdr = Header::default();
        if v.len() < mem::size_of::<Self>() {
            return None;
        }

        hdr.nlmsg_len = u32!(v);
        hdr.nlmsg_type = u16!(v);
        hdr.nlmsg_flags = u16!(v);
        hdr.nlmsg_seq = u32!(v);
        hdr.nlmsg_pid = u32!(v);

        Some(hdr)
    }

    /// Returns the overall size of this header, including it's payload
    pub fn size(&self) -> usize {
        self.nlmsg_len as usize
    }
}

/// All messsage types that can be send to a AF_NETLINK socket
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MessageType {
    /// When no message type was detected
    None = 0x00,

    /// No Operation
    NoOp = 0x01,

    /// Error message/return
    Error = 0x02,

    /// No more messages, doen!
    Done = 0x03,

    /// Too much data for buffer, data lost
    Overrun = 0x04,

    /// Query for socket information
    SockDiagByFamily = 0x14,

    /// Destroy a socket?
    SockDestroy = 0x015,
}

impl MessageType {
    /// Creates an instance of a NETLINK message from an integer.  Integer
    /// values are mapped from the netlink(7) man page/source code
    ///
    /// # Arguments
    ///
    /// * `t` - Integer value corresponding to a NETLINK message type
    pub fn new(t: u16) -> MessageType {
        match t {
            0x00 => MessageType::None,
            0x01 => MessageType::NoOp,
            0x02 => MessageType::Error,
            0x03 => MessageType::Done,
            0x04 => MessageType::Overrun,
            0x14 => MessageType::SockDiagByFamily,
            0x15 => MessageType::SockDestroy,
            x => panic!("Unknown NETLINK message: {}", x),
        }
    }

    /// Converts a NETLINK message type to a 2-byte array
    ///
    /// Used to convert the message into a vec before writing to
    /// a socket
    pub fn as_bytes(&self) -> [u8; 2] {
        (*self as u16).to_le_bytes()
    }
}
