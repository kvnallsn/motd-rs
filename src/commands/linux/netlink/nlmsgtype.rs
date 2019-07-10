//! Enum convering what message types can be sent via a NETLINK socket

/// All messsage types that can be send to a AF_NETLINK socket
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NlMsgType {
    /// When no message type was detected
    None = 0x00,

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

impl NlMsgType {
    /// Creates an instance of a NETLINK message from an integer.  Integer
    /// values are mapped from the netlink(7) man page/source code
    ///
    /// # Arguments
    ///
    /// * `t` - Integer value corresponding to a NETLINK message type
    pub fn new(t: u16) -> NlMsgType {
        match t {
            0x00 => NlMsgType::None,
            0x01 => NlMsgType::NoOp,
            0x02 => NlMsgType::Error,
            0x03 => NlMsgType::Done,
            0x04 => NlMsgType::Overrun,
            0x14 => NlMsgType::SockDiagByFamily,
            0x15 => NlMsgType::SockDestroy,
            x => panic!("Unknown NETLINK message: {}", x),
        }
    }

    /// Converts a NETLINK message type to a 2-byte array
    ///
    /// Used to convert the message into a vec before writing to
    /// a socket
    pub fn as_bytes(&self) -> [u8; 2] {
        (*self as u16).to_le_bytes()
    }
}
