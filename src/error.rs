//! `HttpError` — parse / protocol error type.

use std::fmt;

// HTTP Error
// ---------------------------------------------------------------------------

/// Errors produced by the HTTP parser and builder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HttpError {
    InvalidMethod,
    InvalidVersion,
    InvalidStatusCode,
    InvalidRequest,
    InvalidResponse,
    InvalidHeader,
    InvalidChunk,
    InvalidUri,
    InvalidCookie,
    InvalidMediaType,
    Incomplete,
    TooLarge,
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMethod => f.write_str("invalid HTTP method"),
            Self::InvalidVersion => f.write_str("invalid HTTP version"),
            Self::InvalidStatusCode => f.write_str("invalid status code"),
            Self::InvalidRequest => f.write_str("invalid HTTP request"),
            Self::InvalidResponse => f.write_str("invalid HTTP response"),
            Self::InvalidHeader => f.write_str("invalid HTTP header"),
            Self::InvalidChunk => f.write_str("invalid chunked encoding"),
            Self::InvalidUri => f.write_str("invalid URI"),
            Self::InvalidCookie => f.write_str("invalid cookie"),
            Self::InvalidMediaType => f.write_str("invalid media type"),
            Self::Incomplete => f.write_str("incomplete data"),
            Self::TooLarge => f.write_str("payload too large"),
        }
    }
}

impl std::error::Error for HttpError {}
