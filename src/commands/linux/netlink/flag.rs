//! All flags for NETLINK messages

/// Gerenic NETLINK flags
#[allow(dead_code)]
pub enum Flag {
    /// Denotes a request message.  Required for all requests
    Request,

    /// Multipath message, terminate messages with a message type of MessageType::Done
    Multi,

    /// Reply wiht an acknowledgment (with zero or error code)
    Ack,

    /// Echo this request message
    Echo,

    /// Dump was inconsistaent due to sequence change
    DumpInconsistent,

    /// Dump was filitered as requested
    DumpFiltered,

    /// Specify tree root (aka retreive all) (Get only)
    Root,

    /// Return all that match filter (if provided) (Get only)
    Match,

    /// Combination of Root | Match (Get Only)
    Dump,

    /// Atomic GET (Get only)
    Atomic,

    /// Override/overwrite if already exists (New only)
    Replace,

    /// Dn not touch if already exists (New Only)
    Exclusive,

    /// Create if not exists (New Only)
    Create,

    /// Add to end of list (New Only)
    Append,

    /// Do not delete recursively (Delete Only)
    NonRecursiveDelete,

    /// Request was capped (ACK flag)
    Capped,

    /// Extended ACK TLVs were included (ACK flag)
    AckTlvs,
}

impl Flag {
    /// Convert this flag to it's 16-bit representation used to
    /// send messages in NETLINK headers.  Returns a 16-bit unsigned
    /// integer for each flag
    pub fn as_u16(&self) -> u16 {
        let flag = match self {
            Flag::Request => libc::NLM_F_REQUEST,
            Flag::Multi => libc::NLM_F_MULTI,
            Flag::Ack => libc::NLM_F_ACK,
            Flag::Echo => libc::NLM_F_ECHO,
            Flag::DumpInconsistent => libc::NLM_F_DUMP_INTR,
            Flag::DumpFiltered => libc::NLM_F_DUMP_FILTERED,
            Flag::Root => libc::NLM_F_ROOT,
            Flag::Match => libc::NLM_F_MATCH,
            Flag::Dump => libc::NLM_F_DUMP,
            Flag::Atomic => libc::NLM_F_ATOMIC,
            Flag::Replace => libc::NLM_F_REPLACE,
            Flag::Exclusive => libc::NLM_F_EXCL,
            Flag::Create => libc::NLM_F_CREATE,
            Flag::Append => libc::NLM_F_APPEND,
            Flag::NonRecursiveDelete => 0x100, // No libc corresponding flag
            Flag::Capped => 0x100,             // No libc corresponding flag
            Flag::AckTlvs => 0x200,            // No libc corresponding flag
        };

        flag as u16
    }
}
