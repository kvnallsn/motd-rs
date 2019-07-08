//! Enum convering what message types can be sent via a NETLINK socket

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
