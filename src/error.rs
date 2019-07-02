//! Error representation for motd

/// Represents different errors that can occur during execution of motd
pub enum Error {
    /// The command that was executed failed
    CommandFailed,

    /// Regex failed to compile/parsing failed
    ParsingFailed(&'static str),

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

impl From<regex::Error> for Error {
    fn from(_: regex::Error) -> Error {
        Error::ParsingFailed("Regex failed to compile")
    }
}
