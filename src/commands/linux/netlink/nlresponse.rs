//! The overall NETLINK message container

use crate::commands::linux::netlink::{types::sockdiag, NlMsgHeader, NlMsgType};
use std::mem;

/// A payload (or defined request type) to embed in the NETLINK message
#[derive(Clone, Debug)]
pub enum NlResponsePayload {
    /// No payload
    None,

    /// Socket Diagnostic Response
    SockDiag(sockdiag::Response),
}

/// Container to hold a message received from the system
#[derive(Clone, Debug)]
pub struct NetlinkResponse {
    /// Header, defining size and other characteristics
    pub header: NlMsgHeader,

    /// The payload, wrapping the response information
    pub payload: NlResponsePayload,

    /// Miscellanous attributes received
    pub attrs: Vec<u8>,
}

impl NetlinkResponse {
    /// Creates a new Netlink Response Message from a buffer, returning the
    /// message if one could be created, or None if creation failed.
    ///
    /// # Arguments
    ///
    /// * `v` - Vec to extract message from (and advance)
    pub fn new(v: &mut Vec<u8>) -> Option<NetlinkResponse> {
        let hdr = NlMsgHeader::from_vec(v);

        if let Some(header) = hdr {
            let payload_sz = header.nlmsg_len as usize;
            if payload_sz < mem::size_of::<NlMsgHeader>() {
                return None;
            }

            let sz = payload_sz - mem::size_of::<NlMsgHeader>();
            let mut data = v.drain(0..sz).collect();

            let payload = match header.msg_type() {
                NlMsgType::SockDiagByFamily => {
                    NlResponsePayload::SockDiag(sockdiag::Response::new(&mut data))
                }
                _ => NlResponsePayload::None,
            };

            Some(NetlinkResponse {
                header,
                payload,
                attrs: data,
            })
        } else {
            None
        }
    }

    /// Returns true if this is the last response in a series of resposnes
    /// (aka, the header identifies as Done)
    pub fn is_last(&self) -> bool {
        self.header.msg_type() == NlMsgType::Done
    }
}
