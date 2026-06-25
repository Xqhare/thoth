use nemesis::NemesisError;
use std::fmt;

/// Crate-level Result type using `NemesisError`
pub type ThothResult<T> = Result<T, NemesisError>;

#[derive(Debug)]
pub enum ThothError {
    TableParsing(String),
    InvalidUtf8(String),
    Io(std::io::Error),
}

impl fmt::Display for ThothError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ThothError::TableParsing(msg) | ThothError::InvalidUtf8(msg) => write!(f, "{}", msg),
            ThothError::Io(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for ThothError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ThothError::TableParsing(_) | ThothError::InvalidUtf8(_) => None,
            ThothError::Io(err) => Some(err),
        }
    }
}

impl From<std::io::Error> for ThothError {
    fn from(err: std::io::Error) -> Self {
        ThothError::Io(err)
    }
}
