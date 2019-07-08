//! All sockdiag(7) related functions and structs

use super::NlMsgHeader;
use std::{
    mem,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
};

/// Supported IP protocols
#[derive(Clone, Copy, Debug)]
pub enum AddressFamily {
    /// Unknown address space
    Unknown = 0x00,

    /// IPv4 address space
    Inet = 0x02,

    /// IPv6 address space
    Inet6 = 0x0A,
}

impl AddressFamily {
    pub fn from_u8(u: u8) -> AddressFamily {
        match u {
            0x02 => AddressFamily::Inet,
            0x0A => AddressFamily::Inet6,
            _ => AddressFamily::Unknown,
        }
    }
}

/// Supported L4 protocols
#[derive(Clone, Copy, Debug)]
pub enum L4Protocol {
    /// Transmission Control Protocol
    Tcp = 0x06,

    /// User Datagaram Protocol
    Udp = 0x11,

    /// User Datagaram Protocol Lite
    UdpLite = 136,
}

/// A Internet (INet) Diagnostics request.  Returns all information
/// regarding IPv4 and IPv6 sockets on this computer
#[derive(Clone, Copy, Debug)]
pub struct NlINetDiagReqV2 {
    /// This should be set to either AF_INET or AF_INET6 for IPv4 or
    /// IPv6 sockets respectively.
    sdiag_family: AddressFamily,

    /// What network protocol to inspect (TCP, UDP, or UDPLITE)
    sdiag_protocol: L4Protocol,

    /// Set of flags defining what kind of extended information to report
    /// See sock_diag(7)
    idiag_ext: u8,

    /// Should be set to zero (0)
    pad: u8,

    /// This is a bit mask that defines a filter of socket states.
    /// Only those sockets whose states are in this mask will be
    /// reported.  Ignored when querying for an individual socket.
    idiag_states: u32,

    /// This is a socket ID object that is used in dump requests, in
    /// queries about individual sockets, and is reported back in each
    /// response.  Unlike UNIX domain sockets, IPv4 and IPv6 sockets
    /// are identified using addresses and ports.  All
    /// values are in network byte order network byte order
    id: NlINetDiagSockId,
}

impl std::default::Default for NlINetDiagReqV2 {
    fn default() -> NlINetDiagReqV2 {
        NlINetDiagReqV2 {
            sdiag_family: AddressFamily::Inet,
            sdiag_protocol: L4Protocol::Tcp,
            idiag_ext: 0,
            pad: 0,
            idiag_states: (1 << 10), // LISTEN only
            id: NlINetDiagSockId::default(),
        }
    }
}

impl NlINetDiagReqV2 {
    pub fn new(family: AddressFamily, protocol: L4Protocol) -> NlINetDiagReqV2 {
        let mut req = NlINetDiagReqV2::default();
        req.sdiag_family = family;
        req.sdiag_protocol = protocol;
        req
    }

    pub fn to_vec(self) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();
        vec.push(self.sdiag_family as u8);
        vec.push(self.sdiag_protocol as u8);
        vec.push(self.idiag_ext);
        vec.push(self.pad);
        vec.extend_from_slice(&self.idiag_states.to_le_bytes());
        vec.append(&mut self.id.to_vec());
        vec
    }
}

/// The internet socket connection information, including source
/// and destination ports and IP addresses
#[derive(Clone, Copy, Debug)]
pub struct NlINetDiagSockId {
    /// The source port (big endian)
    idiag_sport: u16,

    /// The destination port (big endian)
    idiag_dport: u16,

    /// The source address (big endian)
    idiag_src: IpAddr,

    /// The destination address (big endian)
    idiag_dst: IpAddr,

    /// The interface number the socket is bound to
    idiag_if: u32,

    /// This is an array of opaque identifiers that could be used
    /// along with other fields of this structure to specify an indi‐
    /// vidual socket.  It is ignored when querying for a list of
    /// sockets, as well as when all its elements are set
    /// to -1.
    idiag_cookie: [u32; 2],
}

impl std::default::Default for NlINetDiagSockId {
    fn default() -> NlINetDiagSockId {
        NlINetDiagSockId {
            idiag_sport: 0,
            idiag_dport: 0,
            idiag_src: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            idiag_dst: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            idiag_if: 0,
            idiag_cookie: [0, 0],
        }
    }
}

impl NlINetDiagSockId {
    pub fn from_msg(family: &AddressFamily, v: &mut Vec<u8>) -> NlINetDiagSockId {
        println!("Building Net Diag Msg");
        let src_port = u16_be!(v);
        let dst_port = u16_be!(v);
        let src_ip = match family {
            AddressFamily::Inet => {
                let ip = IpAddr::V4(Ipv4Addr::new(u8!(v), u8!(v), u8!(v), u8!(v)));
                u32!(v);
                u32!(v);
                u32!(v);
                ip
            }
            AddressFamily::Inet6 => IpAddr::V6(Ipv6Addr::new(
                u16!(v),
                u16!(v),
                u16!(v),
                u16!(v),
                u16!(v),
                u16!(v),
                u16!(v),
                u16!(v),
            )),
            _ => IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        };

        let dst_ip = match family {
            AddressFamily::Inet => {
                let ip = IpAddr::V4(Ipv4Addr::new(u8!(v), u8!(v), u8!(v), u8!(v)));
                u32!(v);
                u32!(v);
                u32!(v);
                ip
            }
            AddressFamily::Inet6 => IpAddr::V6(Ipv6Addr::new(
                u16!(v),
                u16!(v),
                u16!(v),
                u16!(v),
                u16!(v),
                u16!(v),
                u16!(v),
                u16!(v),
            )),
            _ => IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        };
        let interface = u32!(v);
        let cookie = [u32!(v), u32!(v)];

        NlINetDiagSockId {
            idiag_sport: src_port,
            idiag_dport: dst_port,
            idiag_src: src_ip,
            idiag_dst: dst_ip,
            idiag_if: interface,
            idiag_cookie: cookie,
        }
    }

    pub fn to_vec(self) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();
        vec.extend_from_slice(&self.idiag_sport.to_le_bytes());
        vec.extend_from_slice(&self.idiag_dport.to_le_bytes());
        vec.extend_from_slice(&[0u8; 16]); // idiag_src
        vec.extend_from_slice(&[0u8; 16]); // idiag_dst
        vec.extend_from_slice(&self.idiag_if.to_le_bytes());
        vec.extend_from_slice(&[0u8; 8]); // idiagcookie
        vec
    }
}

/// Response to a INet socket request message
#[derive(Clone, Debug)]
pub struct NlINetDiagMsg {
    idiag_family: AddressFamily,
    idiag_state: u8,
    idiag_time: u8,
    idiag_retrans: u8,
    id: NlINetDiagSockId,
    idiag_expires: u32,
    idiag_rqueue: u32,
    idiag_wqueue: u32,
    idiag_uid: u32,
    idiag_inode: u32,
}

impl NlINetDiagMsg {
    pub fn new(hdr: &NlMsgHeader, v: &mut Vec<u8>) -> NlINetDiagMsg {
        let sz = mem::size_of::<Self>();
        let mut b: Vec<u8> = v.drain(0..sz).collect();

        let mut msg = NlINetDiagMsg {
            idiag_family: AddressFamily::Unknown,
            idiag_state: 0,
            idiag_time: 0,
            idiag_retrans: 0,
            id: NlINetDiagSockId::default(),
            idiag_expires: 0,
            idiag_rqueue: 0,
            idiag_wqueue: 0,
            idiag_uid: 0,
            idiag_inode: 0,
        };

        msg.idiag_family = AddressFamily::from_u8(u8!(b));
        msg.idiag_state = u8!(b);
        msg.idiag_time = u8!(b);
        msg.idiag_retrans = u8!(b);
        msg.id = NlINetDiagSockId::from_msg(&msg.idiag_family, &mut b);
        msg.idiag_expires = u32!(b);
        msg.idiag_rqueue = u32!(b);
        msg.idiag_wqueue = u32!(b);
        msg.idiag_uid = u32!(b);
        msg.idiag_inode = u32!(b);

        msg
    }
}
