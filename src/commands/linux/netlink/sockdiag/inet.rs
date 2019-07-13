//! All sockdiag(7) related functions and structs

use crate::commands::linux::netlink::{
    flag::Flag,
    header::{Header, MessageType},
    sockdiag::AddressFamily,
    NetlinkFamily, NetlinkRequest,
};
use std::{
    mem,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
};

/// Supported L4 protocols
#[derive(Clone, Copy, Debug)]
pub enum Protocol {
    /// Transmission Control Protocol
    Tcp = libc::IPPROTO_TCP as isize,

    /// User Datagaram Protocol
    Udp = libc::IPPROTO_UDP as isize,

    /// User Datagaram Protocol Lite
    UdpLite = libc::IPPROTO_UDPLITE as isize,
}

/// Various TCP states that a socket can be in
pub enum SocketState {
    Established,
    SynSent,
    SynRecv,
    FinWait1,
    FinWait2,
    TimeWait,
    Close,
    CloseWait,
    LastAck,
    Listen,
    Closing,
    NewSynRecv,
}

impl SocketState {
    /// Returns this enum as a 32-bit representation
    pub fn as_u32(&self) -> u32 {
        match self {
            SocketState::Established => 0x01,
            SocketState::SynSent => 0x02,
            SocketState::SynRecv => 0x03,
            SocketState::FinWait1 => 0x04,
            SocketState::FinWait2 => 0x05,
            SocketState::TimeWait => 0x06,
            SocketState::Close => 0x07,
            SocketState::CloseWait => 0x08,
            SocketState::LastAck => 0x09,
            SocketState::Listen => 0x0A,
            SocketState::Closing => 0xB,
            SocketState::NewSynRecv => 0x0C,
        }
    }

    /// Return this state in flag form
    pub fn as_flag(&self) -> u32 {
        1 << self.as_u32()
    }
}

/// Public facing struct to request internet socket (aka TCP, UDP, etc.)
/// socket information
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Request {
    hdr: Header,
    msg: NlINetDiagReqV2,
}

impl Request {
    /// Creates a new request to return information about internet sockets
    /// (TCP, UDP, and UDPLITE) on this machine.
    ///
    /// Defaults to:
    ///     AddressFamily: Inet (i.e., IPv4)
    ///     Protocol: TCP
    ///     Socket State: None
    pub fn new() -> Request {
        let hdr = Header::new(MessageType::SockDiagByFamily, 56).flag(Flag::Dump);

        Request {
            hdr,
            msg: NlINetDiagReqV2::default(),
        }
    }

    /// Sets the states the sockets must be in.  Valid states are:
    /// * `LISTEN`
    /// * `CONNECTION_ESTABLISHED`
    pub fn socket_state(mut self, state: SocketState) -> Self {
        self.msg.idiag_states |= state.as_flag();
        self
    }

    /// Sets the address family for this request.  Valid options are:
    /// * `Inet` - IPv4 Address Space
    /// * `Inet6` - IPv6 Addres Space
    ///
    /// # Arguments
    ///
    /// * `family` - Address family for this request
    pub fn address_family(mut self, family: AddressFamily) -> Self {
        self.msg.sdiag_family = family as u8;
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
    pub fn protocol(mut self, proto: Protocol) -> Self {
        self.msg.sdiag_protocol = proto as u8;
        self
    }
}

impl NetlinkRequest for Request {
    /// Returns the family/kernel module to use for this request
    fn family(&self) -> NetlinkFamily {
        NetlinkFamily::SockDiag
    }
}

/// An Internet (INet) Diagnostics request.  Returns all information
/// regarding IPv4 and IPv6 sockets on this computer
#[repr(C)]
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct NlINetDiagReqV2 {
    /// This should be set to either AF_INET or AF_INET6 for IPv4 or
    /// IPv6 sockets respectively.
    sdiag_family: u8,

    /// What network protocol to inspect (TCP, UDP, or UDPLITE)
    sdiag_protocol: u8,

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
    /// The source port (big endian)
    idiag_sport: u16,

    /// The destination port (big endian)
    idiag_dport: u16,

    /// The source address (big endian)
    idiag_src: [u32; 4],

    /// The destination address (big endian)
    idiag_dst: [u32; 4],

    /// The interface number the socket is bound to
    idiag_if: u32,

    /// This is an array of opaque identifiers that could be used
    /// along with other fields of this structure to specify an indi‐
    /// vidual socket.  It is ignored when querying for a list of
    /// sockets, as well as when all its elements are set
    /// to -1.
    idiag_cookie: [u32; 2],
}

impl std::default::Default for NlINetDiagReqV2 {
    fn default() -> NlINetDiagReqV2 {
        NlINetDiagReqV2 {
            sdiag_family: AddressFamily::Inet as u8,
            sdiag_protocol: Protocol::Tcp as u8,
            idiag_ext: 0,
            pad: 0,
            idiag_states: 0,
            idiag_sport: 0,
            idiag_dport: 0,
            idiag_src: [0, 0, 0, 0],
            idiag_dst: [0, 0, 0, 0],
            idiag_if: 0,
            idiag_cookie: [0, 2],
        }
    }
}

impl NlINetDiagReqV2 {
    /// Creates a new request for IPv4 or IPv6 sockets with the specified
    /// protocol
    ///
    /// # Arguments
    ///
    /// * `family` - Inet or Inet6 (Unix will cause failure)
    /// * `protocol` - Layer 4 protocol return
    pub fn new(family: AddressFamily, protocol: Protocol) -> NlINetDiagReqV2 {
        let mut req = NlINetDiagReqV2::default();
        req.sdiag_family = family as u8;
        req.sdiag_protocol = protocol as u8;
        req
    }
}

/// The internet socket connection information, including source
/// and destination ports and IP addresses
#[repr(C)]
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
    /// Builds the representation of a internet socket from the buffer for the
    /// specified address family
    ///
    /// # Arguments
    ///
    /// * `family` - Inet or Inet6 (Unix will return IPv4 with address 0.0.0.0)
    /// * `v` - Buffer of u8 byte to build from
    pub fn parse(family: &AddressFamily, v: &mut Vec<u8>) -> NlINetDiagSockId {
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
}

/// Response to a INet socket request message
#[derive(Clone, Debug)]
pub struct Response {
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

impl Response {
    pub fn new(v: &mut Vec<u8>) -> Response {
        let sz = mem::size_of::<Self>();
        let mut b: Vec<u8> = v.drain(0..sz).collect();

        let mut msg = Response {
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
        msg.id = NlINetDiagSockId::parse(&msg.idiag_family, &mut b);
        msg.idiag_expires = u32!(b);
        msg.idiag_rqueue = u32!(b);
        msg.idiag_wqueue = u32!(b);
        msg.idiag_uid = u32!(b);
        msg.idiag_inode = u32!(b);

        msg
    }
}
