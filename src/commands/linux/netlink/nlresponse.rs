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

            Some(NetlinkResponse { header, payload })
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

/// A NETLINK attribute that is returned alongside a given response
pub struct NetlinkAttribute {
    /// The size of this attribute, include this header
    pub size: u16,

    /// The type of this attribute (varies from subsystem to subsystem)
    pub ty: u16,

    /// The rest od the data contained inside this attribute
    pub data: Vec<u8>,
}

impl NetlinkAttribute {
    /// Creates a new NETLINK attribute from a buffer, consuming the first n
    /// bytes as dictacted by the attribute
    ///
    /// # Arguments
    ///
    /// * `v` - Buffer to create attribute from
    pub fn new(v: &mut Vec<u8>) -> Option<NetlinkAttribute> {
        // first make sure there is enough data left.  The minimum data required
        // is 4 bytes (2 16-bit values, `len` and `ty`)
        if v.len() > 4 {
            let size = u16!(v);
            let ty = u16!(v);

            // Now extract the rest of the data for this attribute
            let len = size as usize;
            let data: Vec<u8> = v.drain(0..(len - 4)).collect();

            // NETLINK messages are aligned to 4-byte increments
            // Discard any extra data up front
            let discard = 4 - (len % 4);
            if discard != 4 {
                let _ = advance!(v, discard);
            }

            return Some(NetlinkAttribute { size, ty, data });
        }

        None
    }
}
