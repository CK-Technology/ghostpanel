use std::fmt;

/// GhostPanel error types
#[derive(Debug)]
pub enum Error {
    /// Configuration errors
    Config(String),

    /// Network/HTTP errors
    Network(String),

    /// Bolt integration errors
    Bolt(String),

    /// QUIC/HTTP3 errors
    Quic(String),

    /// Serialization errors
    Serialization(serde_json::Error),

    /// I/O errors
    Io(std::io::Error),

    /// Authentication errors
    Auth(String),

    /// Container operation errors
    Container(String),

    /// GPU/Gaming errors
    Gaming(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Config(msg) => write!(f, "Configuration error: {}", msg),
            Error::Network(msg) => write!(f, "Network error: {}", msg),
            Error::Bolt(msg) => write!(f, "Bolt error: {}", msg),
            Error::Quic(msg) => write!(f, "QUIC error: {}", msg),
            Error::Serialization(err) => write!(f, "Serialization error: {}", err),
            Error::Io(err) => write!(f, "I/O error: {}", err),
            Error::Auth(msg) => write!(f, "Authentication error: {}", msg),
            Error::Container(msg) => write!(f, "Container error: {}", msg),
            Error::Gaming(msg) => write!(f, "Gaming error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Serialization(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

// QUIC error conversions will be added when GQUIC library is ready

/// GhostPanel result type
pub type Result<T> = std::result::Result<T, Error>;