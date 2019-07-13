//! Support for sending netlink messages

use log::debug;

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
mod nlmsgheader;
mod nlmsgtype;
mod nlrequest;
mod nlresponse;
mod nlsocket;
mod types;

pub use nlflags::{NlFlag, NlGetFlag};
pub use nlmsgheader::NlMsgHeader;
pub use nlmsgtype::NlMsgType;

pub use nlrequest::NetlinkRequest;
pub use nlresponse::{NetlinkAttribute, NetlinkResponse, NlResponsePayload};
pub use nlsocket::{AddressFamily, L4Protocol, NetlinkFamily, NetlinkSocket};

pub use types::sockdiag;

fn examine_bytes<T>(t: &T) {
    let b = to_bytes(t);
    print_bytes(b);
}

fn to_bytes<T>(t: &T) -> &[u8] {
    let p: *const T = t;
    let p = p as *const u8;

    unsafe { std::slice::from_raw_parts(p, std::mem::size_of::<T>()) }
}

fn print_bytes(b: &[u8]) {
    println!("-----------------------------------------");

    let mut i: usize = 0;
    while i < b.len() {
        print!("0x{:02x} ", b[i]);

        i += 1;
        if i % 8 == 0 {
            print!("\n");
        }
    }

    if i % 8 != 0 {
        print!("\n");
    }

    println!("-----------------------------------------");
}

pub fn socket_test() {
    /*
    let req = types::InternetSocketRequest::new();
    let resps = req.send();

    println!("{} Listen TCP IPv4 Sockets", resps.len());
    */
    let req = sockdiag::unix::Request::new().attributes(vec![
        sockdiag::unix::RequestAttribute::ShowName,
        sockdiag::unix::RequestAttribute::ShowVfs,
        sockdiag::unix::RequestAttribute::ShowPeer,
        sockdiag::unix::RequestAttribute::ShowIcons,
        sockdiag::unix::RequestAttribute::ShowRQLen,
        sockdiag::unix::RequestAttribute::ShowMemInfo,
    ]);
    examine_bytes(&req);
    println!("{:#?}", req);

    let resps = req.send();
    println!("{:#?}", resps);

    println!("-----------------------------------------");

    let req = sockdiag::inet::Request::new();
    examine_bytes(&req);
    let resps = req.send();
    println!("# TCPv4 Listen: {}", resps.len());
}

/*
pub fn socket_test2() {
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
        let msg = NlResponse::new(&mut buff);
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
*/
