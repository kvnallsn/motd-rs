//! The overall NETLINK message container

use super::{NlINetDiagMsg, NlMsgHeader, NlMsgType};
use std::mem;

/// A payload (or defined request type) to embed in the NETLINK message
#[derive(Clone, Debug)]
pub enum NlMsgPayload {
    /// No payload
    None,

    /// Socket Diagnostic Response
    SockDiag(NlINetDiagMsg),
}

/// Container to hold a message received from the system
#[derive(Clone, Debug)]
pub struct NlMessage {
    /// Header, defining size and other characteristics
    pub header: NlMsgHeader,

    /// The payload, wrapping the response information
    pub payload: NlMsgPayload,

    /// Miscellanous attributes received
    pub attrs: Vec<u8>,
}

impl NlMessage {
    /// Creates a new Netlink Message from a buffer, returning the message
    /// if one could be created, or None if creation failed.
    ///
    /// # Arguments
    ///
    /// * `v` - Vec to extract message from (and advance)
    pub fn new(v: &mut Vec<u8>) -> Option<NlMessage> {
        let hdr = NlMsgHeader::from_vec(v);

        if let Some(header) = hdr {
            let payload_sz = header.nlmsg_len as usize;
            if payload_sz < mem::size_of::<NlMsgHeader>() {
                return None;
            }

            let sz = payload_sz - mem::size_of::<NlMsgHeader>();
            let mut data = v.drain(0..sz).collect();

            let payload = match header.nlmsg_type {
                NlMsgType::SockDiagByFamily => {
                    NlMsgPayload::SockDiag(NlINetDiagMsg::new(&header, &mut data))
                }
                _ => NlMsgPayload::None,
            };

            Some(NlMessage {
                header,
                payload,
                attrs: data,
            })
        } else {
            None
        }
    }
}
