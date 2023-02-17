use std::error::Error;
use std::{fmt, io};

pub type DriverResult<T> = Result<T, DriverError>;

#[derive(Debug)]
pub enum DriverError {
    InvalidRoot,
    RootNotFound(io::Error),
    InvalidSourceName,
    SourceNotFound(io::Error),
    IoError(io::Error),
}

impl Error for DriverError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        let source = match self {
            Self::RootNotFound(e) => e,
            Self::SourceNotFound(e) => e,
            Self::IoError(e) => e,
            _ => return None,
        };

        Some(source)
    }
}

impl fmt::Display for DriverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRoot => write!(f, "invalid root"),
            Self::RootNotFound(e) => fmt::Display::fmt(e, f),
            Self::InvalidSourceName => write!(f, "invalid source name"),
            Self::SourceNotFound(e) => fmt::Display::fmt(e, f),
            Self::IoError(e) => fmt::Display::fmt(e, f),
        }
    }
}
