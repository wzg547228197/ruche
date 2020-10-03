use failure::Fail;
use std::io;

/// Error type for ruche.
#[derive(Fail, Debug)]
#[fail(display)]
pub enum RucheError {
    /// Non-exist key error.
    #[fail(display="Key not found")]
    KeyNotFound,

    /// I/O error.
    #[fail(display="{}", _0)]
    IO(#[cause] io::Error),

    /// Serialization or deserialization error.
    #[fail(display = "{}", _0)]
    Serde(#[cause] serde_json::Error),

    /// Error with a string message
    #[fail(display = "{}", _0)]
    StringError(String),
}

impl From<io::Error> for RucheError {
    fn from(err: io::Error) -> Self {
        RucheError::IO(err)
    }
}

impl From<serde_json::Error> for RucheError {
    fn from(err: serde_json::Error) -> Self {
        RucheError::Serde(err)
    }
}

/// Result type for ruche.
pub type RucheResult<T> = std::result::Result<T, RucheError>;