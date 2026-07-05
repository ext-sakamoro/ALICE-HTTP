//! `Version` — HTTP protocol version.

use crate::error::HttpError;
use std::fmt;
use std::str::FromStr;

// HTTP Version
// ---------------------------------------------------------------------------

/// HTTP protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Version {
    Http10,
    Http11,
    Http2,
}

impl Version {
    /// Returns the version string.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Http10 => "HTTP/1.0",
            Self::Http11 => "HTTP/1.1",
            Self::Http2 => "HTTP/2",
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for Version {
    type Err = HttpError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HTTP/1.0" => Ok(Self::Http10),
            "HTTP/1.1" => Ok(Self::Http11),
            "HTTP/2" | "HTTP/2.0" => Ok(Self::Http2),
            _ => Err(HttpError::InvalidVersion),
        }
    }
}
