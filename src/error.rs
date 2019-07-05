//! Error representation for motd

/// Represents different errors that can occur during execution of motd
pub enum Error {
    /// The command that was executed failed
    CommandFailed,

    /// Regex failed to compile/parsing failed
    ParsingFailed(ParsingError),

    /// This command is not supported on the request OS
    UnsupportedOS,
}

/// Represents errors that may occur while parsing text
pub enum ParsingError {
    /// Regex failed to compile or in someother way panic'd
    RegexFailed,

    /// String failed to convert to a number
    NumberConversionFailed,
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
        Error::ParsingFailed(ParsingError::RegexFailed)
    }
}

impl From<std::num::ParseIntError> for ParsingError {
    fn from(_: std::num::ParseIntError) -> ParsingError {
        ParsingError::NumberConversionFailed
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(_: std::num::ParseIntError) -> Error {
        Error::from(ParsingError::NumberConversionFailed)
    }
}

impl From<ParsingError> for Error {
    fn from(e: ParsingError) -> Error {
        Error::ParsingFailed(e)
    }
}
