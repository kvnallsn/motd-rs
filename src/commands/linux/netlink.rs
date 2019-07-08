//! Support for sending netlink messages

use log::debug;
use netlink_sys::{Protocol, Socket};

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

macro_rules! flags {
    // base case, just cast as u16
    ($flag:expr) => ($flag as u16);

    // Repetition case
    ($v:expr, $($flag:expr),+) => (($v as u16) | flags!($($flag),+));
}

mod nlflags;
mod nlmsg;
mod nlmsgheader;
mod nlmsgtype;
mod nlsocketdiag;

pub use nlflags::{AsFlag, NlFlag, NlGetFlag};
pub use nlmsg::{NlMessage, NlMsgPayload};
pub use nlmsgheader::NlMsgHeader;
pub use nlmsgtype::NlMsgType;
pub use nlsocketdiag::{AddressFamily, L4Protocol, NlINetDiagMsg, NlINetDiagReqV2};

pub fn socket_test() {
    let s = Socket::new(Protocol::SockDiag).unwrap();
    let hdr = NlMsgHeader::new(NlMsgType::SockDiagByFamily, flags!(NlGetFlag::Dump));
    let inet = NlINetDiagReqV2::new(AddressFamily::Inet, L4Protocol::Tcp);

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

    loop {
        let msg = NlMessage::new(&mut buff);
        if let Some(msg) = msg {
            if msg.header.nlmsg_type == NlMsgType::Done {
                break;
            }

            println!("{:?}", msg);
        } else {
            break;
        }
    }
}
