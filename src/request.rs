//! `Request` + `RequestBuilder`.

use crate::error::HttpError;
use crate::headers::Headers;
use crate::method::Method;
use crate::uri::Uri;
use crate::version::Version;

// HTTP Request
// ---------------------------------------------------------------------------

/// An HTTP request.
#[derive(Debug, Clone)]
pub struct Request {
    pub method: Method,
    pub uri: Uri,
    pub version: Version,
    pub headers: Headers,
    pub body: Vec<u8>,
}

impl Request {
    /// Creates a new request builder.
    #[must_use]
    pub fn builder() -> RequestBuilder {
        RequestBuilder::new()
    }

    /// Parses an HTTP/1.1 request from raw bytes.
    ///
    /// # Errors
    ///
    /// Returns an `HttpError` if the request is malformed.
    pub fn parse(data: &[u8]) -> Result<Self, HttpError> {
        let text = std::str::from_utf8(data).map_err(|_| HttpError::InvalidRequest)?;
        let (head, body_part) = text.split_once("\r\n\r\n").ok_or(HttpError::Incomplete)?;

        let mut lines = head.split("\r\n");
        let request_line = lines.next().ok_or(HttpError::InvalidRequest)?;
        let mut parts = request_line.splitn(3, ' ');

        let method: Method = parts.next().ok_or(HttpError::InvalidRequest)?.parse()?;
        let uri_str = parts.next().ok_or(HttpError::InvalidRequest)?;
        let version: Version = parts.next().ok_or(HttpError::InvalidRequest)?.parse()?;

        let uri = Uri::parse(uri_str)?;
        let mut headers = Headers::new();

        for line in lines {
            let (name, value) = line.split_once(':').ok_or(HttpError::InvalidHeader)?;
            headers.append(name.trim(), value.trim());
        }

        let body = body_part.as_bytes().to_vec();

        Ok(Self {
            method,
            uri,
            version,
            headers,
            body,
        })
    }

    /// Serializes this request to HTTP/1.1 wire format.
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = format!("{} {} {}\r\n", self.method, self.uri, self.version);
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
// Request Builder
// ---------------------------------------------------------------------------

/// Builder for constructing HTTP requests.
#[derive(Debug)]
pub struct RequestBuilder {
    method: Method,
    uri: String,
    version: Version,
    headers: Headers,
    body: Vec<u8>,
}

impl RequestBuilder {
    #[must_use]
    fn new() -> Self {
        Self {
            method: Method::Get,
            uri: "/".to_string(),
            version: Version::Http11,
            headers: Headers::new(),
            body: Vec::new(),
        }
    }

    #[must_use]
    pub const fn method(mut self, method: Method) -> Self {
        self.method = method;
        self
    }

    #[must_use]
    pub fn uri(mut self, uri: &str) -> Self {
        self.uri = uri.to_string();
        self
    }

    #[must_use]
    pub const fn version(mut self, version: Version) -> Self {
        self.version = version;
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

    /// Builds the request.
    ///
    /// # Errors
    ///
    /// Returns `HttpError::InvalidUri` if the URI is invalid.
    pub fn build(self) -> Result<Request, HttpError> {
        let uri = Uri::parse(&self.uri)?;
        Ok(Request {
            method: self.method,
            uri,
            version: self.version,
            headers: self.headers,
            body: self.body,
        })
    }
}
