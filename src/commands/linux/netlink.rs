//! Support for sending netlink messages

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

pub(self) mod flag;
pub(self) mod header;
mod nlrequest;
mod nlresponse;
mod nlsocket;
//mod types;

pub use nlrequest::NetlinkRequest;
pub use nlresponse::{NetlinkAttribute, NetlinkResponse, Payload};
pub use nlsocket::{NetlinkFamily, NetlinkSocket};

pub mod sockdiag;
//pub use types::sockdiag;

use log::{debug, info};

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
    debug!("---------------------------------------");

    let mut s = String::new();
    let mut i: usize = 0;
    while i < b.len() {
        s.push_str(&format!("0x{:02x} ", b[i]));

        i += 1;
        if i % 8 == 0 {
            debug!("{}", s);
            s.clear();
        }
    }

    if i % 8 != 0 {
        debug!("\n");
    }

    debug!("---------------------------------------");
}

pub fn socket_test() {
    /*
    let req = types::InternetSocketRequest::new();
    let resps = req.send();

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
    debug!("{:#?}", req);

    let resps = req.send().unwrap();
    debug!("{:#?}", resps);

    debug!("-----------------------------------------");

    let req = sockdiag::inet::Request::new();
    examine_bytes(&req);
    let resps = req.send().unwrap();
    info!("# TCPv4 Listen: {}", resps.len());
}
