//! Rust wrapper around libc socket, send/recv

use crate::commands::linux::netlink::NetlinkRequest;
use std::{io::Error, ops::Drop, os::unix::io::RawFd};

/// Don't send any flags
const FLAGS: i32 = 0;

/// Represents the various different kernel modules that we can
/// interact with.
#[allow(dead_code)]
pub enum NetlinkFamily {
    /// Routing/Device hook
    Route = libc::NETLINK_ROUTE as isize,

    /// Reserved for user mode socket protocols
    UserSock = libc::NETLINK_USERSOCK as isize,

    /// Unused number, formerly ip_queue
    Firewall = libc::NETLINK_FIREWALL as isize,

    /// Socket monitoring
    SockDiag = libc::NETLINK_SOCK_DIAG as isize,

    /// Netfilter/iptables ULOG
    NFLog = libc::NETLINK_NFLOG as isize,

    /// IPSec
    XFRM = libc::NETLINK_XFRM as isize,

    /// SELinux event notifications
    SELinux = libc::NETLINK_SELINUX as isize,

    /// Open-iSCSI
    ISCSI = libc::NETLINK_ISCSI as isize,

    /// Auditing
    Audit = libc::NETLINK_AUDIT as isize,

    FibLookup = libc::NETLINK_FIB_LOOKUP as isize,
    Connector = libc::NETLINK_CONNECTOR as isize,

    /// Netfilter subsystem
    Netfilter = libc::NETLINK_NETFILTER as isize,

    Ip6Fw = libc::NETLINK_IP6_FW as isize,

    /// DECnet Routing Messages
    DNRTMSG = libc::NETLINK_DNRTMSG as isize,

    /// Kernel messages to userpsace
    KObjectUevent = libc::NETLINK_KOBJECT_UEVENT as isize,

    Generic = libc::NETLINK_GENERIC as isize,

    /// SCSI Transports
    ScsiTransport = libc::NETLINK_SCSITRANSPORT as isize,

    ECryptFs = libc::NETLINK_ECRYPTFS as isize,
    Rdma = libc::NETLINK_RDMA as isize,

    /// Crypto Layer
    Crypto = libc::NETLINK_CRYPTO as isize,
}

/// Represents a NETLINK socket that can send and receive NETLINK messages
pub struct NetlinkSocket(RawFd);

impl Drop for NetlinkSocket {
    fn drop(&mut self) {
        unsafe {
            libc::close(self.0);
        }
    }
}

impl NetlinkSocket {
    /// Creates a new NETLINK socket with the specified netlink family.  The netlink family
    /// is responsible for selecting the correct kernel module or resource to communicate with
    /// after the socket is opened.  If an error occurs, a std::io::Error will be returned.  On
    /// success, the opened NetlinkSocket will be returned.
    ///
    /// Note: There is no need to call close(), the socket will automatically be closed when
    /// the reference to this struct is dropped.
    ///
    /// # Arguments
    ///
    /// * `family` - Kernel module/resource to communicate with (e.g., SockDiag)
    pub fn new(family: NetlinkFamily) -> Result<NetlinkSocket, Error> {
        let fd = unsafe { libc::socket(libc::AF_NETLINK, libc::SOCK_DGRAM, family as i32) };
        if fd == -1 {
            Err(Error::last_os_error())
        } else {
            Ok(NetlinkSocket(fd))
        }
    }

    /// Sends a message through the opened socket, returning the number of bytes read.
    /// The parameter `msg` must implement NetlinkRequest and it *must* have the
    /// #[repr(C)] attribute.  A reference to the struct will be cast as c_void ptr
    /// and then passed to send() in an unsafe call.  If the structure of `msg` does
    /// not exactly match the structure in the appropriate manpage then the call will
    /// most likely fail
    ///
    /// # Arguments
    ///
    /// * `msg` - A struct that implements a NetlinkRequest
    pub fn send<M: NetlinkRequest>(&self, msg: &M) -> Result<usize, Error> {
        let len = std::mem::size_of::<M>();
        let buffer: *const M = msg;
        let sent = unsafe { libc::send(self.0, buffer as *const _, len as usize, FLAGS) };

        if sent < 0 {
            Err(Error::last_os_error())
        } else {
            Ok(sent as usize)
        }
    }

    /// Receives a message sent from the kernel module/resource
    pub fn recv(&self, buffer: &mut [u8]) -> Result<usize, Error> {
        let len = buffer.len();
        let received =
            unsafe { libc::recv(self.0, buffer.as_mut_ptr() as *mut _, len as usize, FLAGS) };

        if received < 0 {
            Err(Error::last_os_error())
        } else {
            Ok(received as usize)
        }
    }
}
