//! All sockdiag(7) related functions and structs

use super::super::{
    AddressFamily, L4Protocol, NetlinkFamily, NetlinkRequest, NetlinkResponse, NetlinkSocket,
    NlGetFlag, NlMsgHeader, NlMsgType,
};
use std::{
    mem,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
};

/// Public facing struct to request internet socket (aka TCP, UDP, etc.)
/// socket information
pub struct InternetSocketRequest(NlMsgHeader, NlINetDiagReqV2);

impl InternetSocketRequest {
    /// Creates a new request to return information about internet sockets
    /// (TCP, UDP, and UDPLITE) on this machine.
    ///
    /// Defaults to:
    ///     AddressFamily: Inet (i.e., IPv4)
    ///     L4Protocol: TCP
    ///     Socket State: LISTEN
    pub fn new() -> InternetSocketRequest {
        InternetSocketRequest(
            NlMsgHeader::new(NlMsgType::SockDiagByFamily, flags!(NlGetFlag::Dump)),
            NlINetDiagReqV2::default(),
        )
    }

    /// Sets the address family for this request.  Valid options are:
    /// * `Inet` - IPv4 Address Space
    /// * `Inet6` - IPv6 Addres Space
    ///
    /// # Arguments
    ///
    /// * `family` - Address family for this request
    pub fn address_family(mut self, family: AddressFamily) -> Self {
        self.1.sdiag_family = family;
        self
    }

    /// Sets the layer 4 protocol for this request.  Valid options are:
    /// * `TCP` - Trasmission Control Protocol
    /// * `UDP` - User Datagram Protocol
    /// * `UDPLITE` - ???
    ///
    /// # Arguments
    ///
    /// * `proto` - Layer 4 protocol for this request
    pub fn protocol(mut self, proto: L4Protocol) -> Self {
        self.1.sdiag_protocol = proto;
        self
    }

    /// Sets the states the sockets must be in.  Valid states are:
    /// * `LISTEN`
    /// * `CONNECTION_ESTABLISHED`
    pub fn socket_state(mut self) -> Self {
        self
    }
}

impl NetlinkRequest for InternetSocketRequest {
    /// Builds a message as an vector of bytes
    fn build(self) -> Vec<u8> {
        let mut hdr = self.0;
        let payload = self.1;

        // expected size of header + NlINetDiagReqV2
        hdr.nlmsg_len = 72;
        let mut msg = hdr.to_vec();
        msg.append(&mut payload.to_vec());
        msg
    }

    /// Returns the family/kernel module to use for this request
    fn family(&self) -> NetlinkFamily {
        NetlinkFamily::SockDiag
    }
}

/// An Internet (INet) Diagnostics request.  Returns all information
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
pub struct InternetSocketResponse {
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

impl InternetSocketResponse {
    pub fn new(hdr: &NlMsgHeader, v: &mut Vec<u8>) -> InternetSocketResponse {
        let sz = mem::size_of::<Self>();
        let mut b: Vec<u8> = v.drain(0..sz).collect();

        let mut msg = InternetSocketResponse {
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

        //msg.idiag_family = AddressFamily::from(u8!(b));
        msg.idiag_family = u8!(b).into();
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
