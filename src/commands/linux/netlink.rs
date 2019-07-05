//! Support for sending netlink messages

use log::debug;
use netlink_sys::{Protocol, Socket};
use std::{
    mem,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
};

macro_rules! advance {
    ($v:expr, $a:expr) => {
        $v.drain(0..$a).collect::<Vec<u8>>()
    };
}

macro_rules! u8 {
    ($v:expr) => {
        $v.remove(0) as u8
    };
}

macro_rules! u16 {
    ($v:expr) => {{
        let a: u16 = $v.remove(0) as u16;
        let b: u16 = $v.remove(0) as u16;
        (a | b << 8) as u16
    }};
}

macro_rules! u16_be {
    ($v:expr) => {{
        let a: u16 = $v.remove(0) as u16;
        let b: u16 = $v.remove(0) as u16;
        (b | a << 8) as u16
    }};
}

macro_rules! u32 {
    ($v:expr) => {{
        let a: u32 = $v.remove(0) as u32;
        let b: u32 = $v.remove(0) as u32;
        let c: u32 = $v.remove(0) as u32;
        let d: u32 = $v.remove(0) as u32;
        (a | b << 8 | c << 16 | d << 24) as u32
    }};
}

// Flags Values
//
const NLM_F_REQUEST: u16 = 0x01; /* It is request message.     */
const NLM_F_MULTI: u16 = 0x02; /* Multipart message, terminated by NLMSG_DONE */
const NLM_F_ACK: u16 = 0x04; /* Reply with ack, with zero or error code */
const NLM_F_ECHO: u16 = 0x08; /* Echo this request         */
const NLM_F_DUMP_INTR: u16 = 0x10; /* Dump was inconsistent due to sequence change */
const NLM_F_DUMP_FILTERED: u16 = 0x20; /* Dump was filtered as requested */

/* Modifiers to GET request */
const NLM_F_ROOT: u16 = 0x100; /* specify tree    root    */
const NLM_F_MATCH: u16 = 0x200; /* return all matching    */
const NLM_F_ATOMIC: u16 = 0x400; /* atomic GET        */
const NLM_F_DUMP: u16 = (NLM_F_ROOT | NLM_F_MATCH);

/* Modifiers to NEW request */
const NLM_F_REPLACE: u16 = 0x100; /* Override existing        */
const NLM_F_EXCL: u16 = 0x200; /* Do not touch, if it exists    */
const NLM_F_CREATE: u16 = 0x400; /* Create, if it does not exist    */
const NLM_F_APPEND: u16 = 0x800; /* Add to end of list        */

/* Modifiers to DELETE request */
const NLM_F_NONREC: u16 = 0x100; /* Do not delete recursively    */

/* Flags for ACK message */
const NLM_F_CAPPED: u16 = 0x100; /* request was capped */
const NLM_F_ACK_TLVS: u16 = 0x200; /* extended ACK TVLs were included */

/// All messsage types that can be send to a AF_NETLINK socket
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NlType {
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

impl NlType {
    pub fn from_u16(t: u16) -> NlType {
        match t {
            0x00 => NlType::Done,
            0x01 => NlType::NoOp,
            0x02 => NlType::Error,
            0x03 => NlType::Done,
            0x04 => NlType::Overrun,
            0x14 => NlType::SockDiagByFamily,
            0x15 => NlType::SockDestroy,
            x => panic!("Unknown NETLINK message: {}", x),
        }
    }

    pub fn as_bytes(&self) -> [u8; 2] {
        (*self as u16).to_le_bytes()
    }
}

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
pub enum IpProtocol {
    /// Transmission Control Protocol
    Tcp = 0x06,

    /// User Datagaram Protocol
    Udp = 0x11,

    /// User Datagaram Protocol Lite
    UdpLite = 136,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct NlMsgHeader {
    /// Length of message including header
    nlmsg_len: u32,

    /// Type of message content
    nlmsg_type: NlType,

    /// Additional flags
    nlmsg_flags: u16,

    /// Sequence number
    nlmsg_seq: u32,

    /// Sender port ID
    nlmsg_pid: u32,
}

impl std::default::Default for NlMsgHeader {
    fn default() -> NlMsgHeader {
        NlMsgHeader {
            nlmsg_len: 0,
            nlmsg_type: NlType::Done,
            nlmsg_flags: 0,
            nlmsg_seq: 0,
            nlmsg_pid: 0,
        }
    }
}

impl NlMsgHeader {
    pub fn new(ty: NlType, flags: u16) -> NlMsgHeader {
        let mut hdr = NlMsgHeader::default();
        hdr.nlmsg_type = ty;
        hdr.nlmsg_flags = NLM_F_REQUEST | flags;
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
        hdr.nlmsg_type = NlType::from_u16(u16!(v));
        hdr.nlmsg_flags = u16!(v);
        hdr.nlmsg_seq = u32!(v);
        hdr.nlmsg_pid = u32!(v);

        Some(hdr)
    }

    pub fn build(mut self, req: NlINetDiagReqV2) -> Vec<u8> {
        self.nlmsg_len = 72; // expected size of header + NlINetDiagReqV2
        println!("Msg size: {}", sz);

        let mut msg = self.to_vec();
        msg.append(&mut req.to_vec());

        msg
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct NlINetDiagReqV2 {
    /// This should be set to either AF_INET or AF_INET6 for IPv4 or
    /// IPv6 sockets respectively.
    sdiag_family: AddressFamily,

    /// What network protocol to inspect (TCP, UDP, or UDPLITE)
    sdiag_protocol: IpProtocol,

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
            sdiag_protocol: IpProtocol::Tcp,
            idiag_ext: 0,
            pad: 0,
            idiag_states: (1 << 10), // LISTEN only
            id: NlINetDiagSockId::default(),
        }
    }
}

impl NlINetDiagReqV2 {
    pub fn new(family: AddressFamily, protocol: IpProtocol) -> NlINetDiagReqV2 {
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

#[derive(Clone, Debug)]
pub struct NlSockDiagMsg(Vec<u8>);

impl NlSockDiagMsg {
    pub fn new(hdr: &NlMsgHeader, v: &mut Vec<u8>) -> NlSockDiagMsg {
        let sz = mem::size_of::<Self>();
        let b = v.drain(0..sz).collect();
        NlSockDiagMsg(b)
    }
}

#[derive(Clone, Debug)]
pub enum NlMsgPayload {
    None,
    SockDiag(NlINetDiagMsg),
}

#[derive(Clone, Debug)]
pub struct NlMessage {
    pub header: NlMsgHeader,
    pub payload: NlMsgPayload,
    pub attrs: Vec<u8>,
}

impl NlMessage {
    /// Creates a new Netlink Message
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

            println!("Header: {:?}", header);
            let payload = match header.nlmsg_type {
                NlType::SockDiagByFamily => {
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

pub fn socket_test() {
    let s = Socket::new(Protocol::SockDiag).unwrap();
    let hdr = NlMsgHeader::new(NlType::SockDiagByFamily, NLM_F_DUMP);
    let inet = NlINetDiagReqV2::new(AddressFamily::Inet, IpProtocol::Tcp);

    debug!("{:?}", hdr);
    debug!("{:?}", inet);

    let msg = hdr.build(inet);
    debug!("{:?}", msg.len());

    let err = s.send(&msg, 0);
    match err {
        Ok(b) => debug!("Success: Sent {} bytes", b),
        Err(e) => panic!("Failed to send msg: {}", e),
    }

    let mut buff = vec![0u8; 16384];
    let err = s.recv(&mut buff, 0);
    match err {
        Ok(b) => debug!("Success: Recv {} bytes", b),
        Err(e) => panic!("Failed to recv msg: {}", e),
    }

    // break into types (iter?)
    debug!("Recv: {:?}", buff);

    while true {
        let msg = NlMessage::new(&mut buff);
        if let Some(msg) = msg {
            if msg.header.nlmsg_type == NlType::Done {
                break;
            }

            println!("{:?}", msg);
        } else {
            break;
        }
    }
}
