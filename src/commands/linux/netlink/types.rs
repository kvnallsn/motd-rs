//! Varios different requests/responses we can send

mod nlinetdiag;
mod nlsockdiag;

pub use nlinetdiag::{InternetSocketRequest, InternetSocketResponse};
pub use nlsockdiag::{UnixDiagState, UnixSocketRequest};
