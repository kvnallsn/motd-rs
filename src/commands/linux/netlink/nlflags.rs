//! All flags for NETLINK messages

/// Gerenic NETLINK flags
#[allow(dead_code)]
pub enum NlFlag {
    /// Denotes a request message.  Required for all requests
    Request = 0x01,

    /// Multipath message, terminate messages with a message type of NlMsgType::Done
    Multi = 0x02,

    /// Reply wiht an acknowledgment (with zero or error code)
    Ack = 0x04,

    /// Echo this request message
    Echo = 0x08,

    /// Dump was inconsistaent due to sequence change
    DumpInconsistent = 0x10,

    /// Dump was filitered as requested
    DumpFiltered = 0x20,
}

/// NETLINK flags specific to GET reqeusts
#[allow(dead_code)]
pub enum NlGetFlag {
    /// Specify tree root (aka retreive all)
    Root = 0x100,

    /// Return all that match filter (if provided)
    Match = 0x200,

    /// Combination of Root | Match
    Dump = 0x300,

    /// Atomic GET
    Atomic = 0x400,
}

/// NETLINK flags specific to NEW/CREATE reqeusts
#[allow(dead_code)]
pub enum NlNewFlags {
    /// Override/overwrite if already exists
    Replace = 0x100,

    /// Dn not touch if already exists
    Exclusive = 0x200,

    /// Create if not exists
    Create = 0x400,

    /// Add to end of list
    Append = 0x800,
}

/// NETLINK flags specific to DELETE reqeusts
#[allow(dead_code)]
pub enum NlDeleteFlags {
    /// Do not delete recursively
    NonRecursive = 0x100,
}

/// NETLINK flags specific to ACKNOWLEDGEMENT reqeusts
#[allow(dead_code)]
pub enum NlAckFlags {
    /// Request was capped
    Capped = 0x100,

    /// Extended ACK TLVs were included
    AckTlvs = 0x200,
}
