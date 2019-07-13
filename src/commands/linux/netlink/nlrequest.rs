//! A `NetlinkRequest` represents an individual request to a specific NETLINK subsystem

use super::{NetlinkFamily, NetlinkResponse, NetlinkSocket};
use std::io::Error;

/// A request that will be sent to a NETLINK subsystem or family.  This trait will automatically
/// implement sending a request to a NETLINK socket and parsing the received response.  The
/// subsystem (or family) the `send` will utilizing when creating the socket is defined by the
/// `family()` function.  Similarly, build will build a complete message request to send
/// down the NETLINK socket.
pub trait NetlinkRequest: Sized {
    /// Returns an instance of the NETLINK family this message is intended
    /// to communicate with.  For example, to get internet socket statistics,
    /// we need to use the `NetlinkFamily::SockDiag` family.  For routing, we
    /// would use `NetlinkFamily::Route`
    fn family(&self) -> NetlinkFamily;

    /// Sends a given message over a new NETLINK socket and parses the response
    /// into a NetlinkResponse struct.  Then returns vector of all responses received,
    /// not including the done response if successful, or an io::Error if an error
    /// occured
    fn send(&self) -> Result<Vec<NetlinkResponse>, Error> {
        let mut responses: Vec<NetlinkResponse> = Vec::new();

        // Create a netlink socket
        let socket = NetlinkSocket::new(self.family())?;

        // Send our message through the socket
        if socket.send(self)? == 0 {
            return Ok(vec![]);
        }

        let mut is_done = false;
        while !is_done {
            // Create a large enough buffer
            let mut buffer = vec![0u8; 16384];

            // Wait for a response
            let received = socket.recv(&mut buffer)?;

            // If we didn't recieve anything, break out of the loop
            if received == 0 {
                break;
            }

            // Parse respone(s) into NetlinkResponse(s)
            loop {
                let resp = NetlinkResponse::new(&mut buffer);
                if let Some(resp) = resp {
                    if resp.is_last() {
                        is_done = true;
                        break;
                    }

                    responses.push(resp);
                } else {
                    break;
                }
            }
        }

        // Return vector of responses
        Ok(responses)
    }
}
