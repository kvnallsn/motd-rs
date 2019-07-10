//! A NETLINK message header, present on all NETLINK messages

use super::{NlFlag, NlMsgType};
use std::mem;

/// Represents the header sent on all NETLINK messages, to include the
/// NETLINK message type and other identifying information
#[derive(Clone, Copy, Debug)]
pub struct NlMsgHeader {
    /// Length of message including header
    pub nlmsg_len: u32,

    /// Type of message content
    pub nlmsg_type: NlMsgType,

    /// Additional flags
    pub nlmsg_flags: u16,

    /// Sequence number
    pub nlmsg_seq: u32,

    /// Sender port ID
    pub nlmsg_pid: u32,
}

impl std::default::Default for NlMsgHeader {
    fn default() -> NlMsgHeader {
        NlMsgHeader {
            nlmsg_len: 0,
            nlmsg_type: NlMsgType::Done,
            nlmsg_flags: 0,
            nlmsg_seq: 0,
            nlmsg_pid: 0,
        }
    }
}

impl NlMsgHeader {
    pub fn new(ty: NlMsgType, flags: u16) -> NlMsgHeader {
        let mut hdr = NlMsgHeader::default();
        hdr.nlmsg_type = ty;
        hdr.nlmsg_flags = flags!(NlFlag::Request, flags);
        hdr
    }

    pub fn to_vec(self) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();
        vec.extend_from_slice(&self.nlmsg_len.to_le_bytes());
        vec.extend_from_slice(&self.nlmsg_type.as_bytes());
        vec.extend_from_slice(&self.nlmsg_flags.to_le_bytes());
        vec.extend_from_slice(&self.nlmsg_seq.to_le_bytes());
        vec.extend_from_slice(&self.nlmsg_pid.to_le_bytes());
        vec
    }

    pub fn from_vec(v: &mut Vec<u8>) -> Option<NlMsgHeader> {
        let mut hdr = NlMsgHeader::default();
        if v.len() < mem::size_of::<Self>() {
            return None;
        }

        hdr.nlmsg_len = u32!(v);
        hdr.nlmsg_type = NlMsgType::new(u16!(v));
        hdr.nlmsg_flags = u16!(v);
        hdr.nlmsg_seq = u32!(v);
        hdr.nlmsg_pid = u32!(v);

        Some(hdr)
    }
}
