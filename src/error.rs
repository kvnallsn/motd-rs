//! Error representation for motd

/// Represents different errors that can occur during execution of motd
pub enum Error {
    /// The command that was executed failed
    CommandFailed,

    /// This command is not supported on the request OS
    UnsupportedOS,
}

/// Wrapper for a result struct
pub type MotdResult<T> = Result<T, Error>;

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Error {
        Error::CommandFailed
    }
}
