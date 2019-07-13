//! Returns information on established vs listening connections

use crate::{
    commands::linux::netlink::{sockdiag, NetlinkRequest},
    error::{Error, MotdResult, ParsingError},
};

// Returns number of listening and established connections (IPv4 TCP only)
pub fn connections(_args: Option<String>) -> MotdResult<(usize, usize)> {
    let req = sockdiag::inet::Request::new().socket_state(sockdiag::inet::SocketState::Listen);
    let listen = req.send().map_err(|_| Error::CommandFailed)?;

    let req = sockdiag::inet::Request::new().socket_state(sockdiag::inet::SocketState::Established);
    let established = req.send().map_err(|_| Error::CommandFailed)?;

    Ok((listen.len(), established.len()))
}
