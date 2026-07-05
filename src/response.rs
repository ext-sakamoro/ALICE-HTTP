//! `Response` + `ResponseBuilder`.

use crate::error::HttpError;
use crate::headers::Headers;
use crate::status::StatusCode;
use crate::version::Version;

// HTTP Response
// ---------------------------------------------------------------------------

/// An HTTP response.
#[derive(Debug, Clone)]
pub struct Response {
    pub version: Version,
    pub status: StatusCode,
    pub headers: Headers,
    pub body: Vec<u8>,
}

impl Response {
    /// Creates a new response builder.
    #[must_use]
    pub const fn builder() -> ResponseBuilder {
        ResponseBuilder::new()
    }

    /// Parses an HTTP/1.1 response from raw bytes.
    ///
    /// # Errors
    ///
    /// Returns an `HttpError` if the response is malformed.
    pub fn parse(data: &[u8]) -> Result<Self, HttpError> {
        let text = std::str::from_utf8(data).map_err(|_| HttpError::InvalidResponse)?;
        let (head, body_part) = text.split_once("\r\n\r\n").ok_or(HttpError::Incomplete)?;

        let mut lines = head.split("\r\n");
        let status_line = lines.next().ok_or(HttpError::InvalidResponse)?;
        let mut parts = status_line.splitn(3, ' ');

        let version: Version = parts.next().ok_or(HttpError::InvalidResponse)?.parse()?;
        let code_str = parts.next().ok_or(HttpError::InvalidResponse)?;
        let code: u16 = code_str.parse().map_err(|_| HttpError::InvalidStatusCode)?;
        let status = StatusCode::from_u16(code)?;
        // reason phrase is ignored (we use canonical)

        let mut headers = Headers::new();
        for line in lines {
            let (name, value) = line.split_once(':').ok_or(HttpError::InvalidHeader)?;
            headers.append(name.trim(), value.trim());
        }

        let body = body_part.as_bytes().to_vec();

        Ok(Self {
            version,
            status,
            headers,
            body,
        })
    }

    /// Serializes this response to HTTP/1.1 wire format.
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = format!("{} {}\r\n", self.version, self.status);
        out.push_str(&self.headers.to_http1());
        out.push_str("\r\n");
        let mut bytes = out.into_bytes();
        bytes.extend_from_slice(&self.body);
        bytes
    }

    /// Returns the `Content-Type` header value, if present.
    #[must_use]
    pub fn content_type(&self) -> Option<&str> {
        self.headers.get("content-type")
    }

    /// Returns the `Content-Length` parsed as `usize`, if present and valid.
    #[must_use]
    pub fn content_length(&self) -> Option<usize> {
        self.headers
            .get("content-length")
            .and_then(|v| v.parse().ok())
    }
}

// ---------------------------------------------------------------------------
// Response Builder
// ---------------------------------------------------------------------------

/// Builder for constructing HTTP responses.
#[derive(Debug)]
pub struct ResponseBuilder {
    version: Version,
    status: StatusCode,
    headers: Headers,
    body: Vec<u8>,
}

impl ResponseBuilder {
    #[must_use]
    const fn new() -> Self {
        Self {
            version: Version::Http11,
            status: StatusCode::OK,
            headers: Headers::new(),
            body: Vec::new(),
        }
    }

    #[must_use]
    pub const fn version(mut self, version: Version) -> Self {
        self.version = version;
        self
    }

    #[must_use]
    pub const fn status(mut self, status: StatusCode) -> Self {
        self.status = status;
        self
    }

    #[must_use]
    pub fn header(mut self, name: &str, value: &str) -> Self {
        self.headers.set(name, value);
        self
    }

    #[must_use]
    pub fn body(mut self, body: &[u8]) -> Self {
        self.body = body.to_vec();
        self
    }

    /// Builds the response.
    #[must_use]
    pub fn build(self) -> Response {
        Response {
            version: self.version,
            status: self.status,
            headers: self.headers,
            body: self.body,
        }
    }
}
