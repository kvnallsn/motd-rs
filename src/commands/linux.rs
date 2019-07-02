//! Commands that are specific to Linux (and it's variants) but not necessarily Unix

mod net;
pub use net::*;

mod uptime;
pub use uptime::*;
