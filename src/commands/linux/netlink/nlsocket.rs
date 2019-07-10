//! Rust wrapper around libc socket, send/recv

use std::{io::Error, ops::Drop, os::unix::io::RawFd};

/// Don't send any flags
const FLAGS: i32 = 0;

/// Represents the various different kernel modules that we can
/// interact with.
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

    /// Sends a message through the opened socket, returning the number of bytes read
    pub fn send(&self, buffer: &[u8]) -> Result<usize, Error> {
        let len = buffer.len();
        let sent = unsafe { libc::send(self.0, buffer.as_ptr() as *const _, len as usize, FLAGS) };

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

/// Supported IP protocols
#[derive(Clone, Copy, Debug)]
pub enum AddressFamily {
    /// Unknown address space
    Unknown = 0x00,

    /// IPv4 address space
    Inet = libc::AF_INET as isize,

    /// IPv6 address space
    Inet6 = libc::AF_INET6 as isize,

    Unix = libc::AF_UNIX as isize,
}

impl From<u8> for AddressFamily {
    fn from(u: u8) -> AddressFamily {
        let i = u as i32;
        match i {
            libc::AF_INET => AddressFamily::Inet,
            libc::AF_INET6 => AddressFamily::Inet6,
            libc::AF_UNIX => AddressFamily::Unix,
            _ => AddressFamily::Unknown,
        }
    }
}

impl From<i32> for AddressFamily {
    fn from(i: i32) -> AddressFamily {
        match i {
            libc::AF_INET => AddressFamily::Inet,
            libc::AF_INET6 => AddressFamily::Inet6,
            libc::AF_UNIX => AddressFamily::Unix,
            _ => AddressFamily::Unknown,
        }
    }
}

/// Supported L4 protocols
#[derive(Clone, Copy, Debug)]
pub enum L4Protocol {
    /// Transmission Control Protocol
    Tcp = 0x06,

    /// User Datagaram Protocol
    Udp = 0x11,

    /// User Datagaram Protocol Lite
    UdpLite = 136,
}
