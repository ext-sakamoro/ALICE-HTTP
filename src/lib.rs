#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::module_name_repetitions)]

//! ALICE-HTTP: Pure Rust HTTP/1.1 and HTTP/2 parser and framework.
//!
//! Provides request/response parsing, headers, methods, status codes,
//! chunked transfer encoding, content negotiation, cookie handling, and MIME types.

use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

// ---------------------------------------------------------------------------
// HTTP Method
// ---------------------------------------------------------------------------

/// HTTP request methods.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
    Trace,
    Connect,
}

impl Method {
    /// Returns the method as an uppercase string slice.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Get => "GET",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Delete => "DELETE",
            Self::Patch => "PATCH",
            Self::Head => "HEAD",
            Self::Options => "OPTIONS",
            Self::Trace => "TRACE",
            Self::Connect => "CONNECT",
        }
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for Method {
    type Err = HttpError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::Get),
            "POST" => Ok(Self::Post),
            "PUT" => Ok(Self::Put),
            "DELETE" => Ok(Self::Delete),
            "PATCH" => Ok(Self::Patch),
            "HEAD" => Ok(Self::Head),
            "OPTIONS" => Ok(Self::Options),
            "TRACE" => Ok(Self::Trace),
            "CONNECT" => Ok(Self::Connect),
            _ => Err(HttpError::InvalidMethod),
        }
    }
}

// ---------------------------------------------------------------------------
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

// ---------------------------------------------------------------------------
// Status Code
// ---------------------------------------------------------------------------

/// HTTP status code with reason phrase.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StatusCode(u16);

impl StatusCode {
    // 1xx
    pub const CONTINUE: Self = Self(100);
    pub const SWITCHING_PROTOCOLS: Self = Self(101);

    // 2xx
    pub const OK: Self = Self(200);
    pub const CREATED: Self = Self(201);
    pub const ACCEPTED: Self = Self(202);
    pub const NO_CONTENT: Self = Self(204);
    pub const PARTIAL_CONTENT: Self = Self(206);

    // 3xx
    pub const MOVED_PERMANENTLY: Self = Self(301);
    pub const FOUND: Self = Self(302);
    pub const SEE_OTHER: Self = Self(303);
    pub const NOT_MODIFIED: Self = Self(304);
    pub const TEMPORARY_REDIRECT: Self = Self(307);
    pub const PERMANENT_REDIRECT: Self = Self(308);

    // 4xx
    pub const BAD_REQUEST: Self = Self(400);
    pub const UNAUTHORIZED: Self = Self(401);
    pub const FORBIDDEN: Self = Self(403);
    pub const NOT_FOUND: Self = Self(404);
    pub const METHOD_NOT_ALLOWED: Self = Self(405);
    pub const NOT_ACCEPTABLE: Self = Self(406);
    pub const CONFLICT: Self = Self(409);
    pub const GONE: Self = Self(410);
    pub const LENGTH_REQUIRED: Self = Self(411);
    pub const PAYLOAD_TOO_LARGE: Self = Self(413);
    pub const URI_TOO_LONG: Self = Self(414);
    pub const UNSUPPORTED_MEDIA_TYPE: Self = Self(415);
    pub const TOO_MANY_REQUESTS: Self = Self(429);

    // 5xx
    pub const INTERNAL_SERVER_ERROR: Self = Self(500);
    pub const NOT_IMPLEMENTED: Self = Self(501);
    pub const BAD_GATEWAY: Self = Self(502);
    pub const SERVICE_UNAVAILABLE: Self = Self(503);
    pub const GATEWAY_TIMEOUT: Self = Self(504);

    /// Create a status code from a raw `u16`.
    ///
    /// # Errors
    ///
    /// Returns `HttpError::InvalidStatusCode` if the code is outside 100..=999.
    pub const fn from_u16(code: u16) -> Result<Self, HttpError> {
        if code >= 100 && code <= 999 {
            Ok(Self(code))
        } else {
            Err(HttpError::InvalidStatusCode)
        }
    }

    /// Returns the numeric code.
    #[must_use]
    pub const fn code(self) -> u16 {
        self.0
    }

    /// Returns the canonical reason phrase, if known.
    #[must_use]
    pub const fn reason(self) -> &'static str {
        match self.0 {
            100 => "Continue",
            101 => "Switching Protocols",
            200 => "OK",
            201 => "Created",
            202 => "Accepted",
            204 => "No Content",
            206 => "Partial Content",
            301 => "Moved Permanently",
            302 => "Found",
            303 => "See Other",
            304 => "Not Modified",
            307 => "Temporary Redirect",
            308 => "Permanent Redirect",
            400 => "Bad Request",
            401 => "Unauthorized",
            403 => "Forbidden",
            404 => "Not Found",
            405 => "Method Not Allowed",
            406 => "Not Acceptable",
            409 => "Conflict",
            410 => "Gone",
            411 => "Length Required",
            413 => "Payload Too Large",
            414 => "URI Too Long",
            415 => "Unsupported Media Type",
            429 => "Too Many Requests",
            500 => "Internal Server Error",
            501 => "Not Implemented",
            502 => "Bad Gateway",
            503 => "Service Unavailable",
            504 => "Gateway Timeout",
            _ => "Unknown",
        }
    }

    /// Returns `true` if this is an informational (1xx) status.
    #[must_use]
    pub const fn is_informational(self) -> bool {
        self.0 >= 100 && self.0 < 200
    }

    /// Returns `true` if this is a success (2xx) status.
    #[must_use]
    pub const fn is_success(self) -> bool {
        self.0 >= 200 && self.0 < 300
    }

    /// Returns `true` if this is a redirection (3xx) status.
    #[must_use]
    pub const fn is_redirection(self) -> bool {
        self.0 >= 300 && self.0 < 400
    }

    /// Returns `true` if this is a client error (4xx) status.
    #[must_use]
    pub const fn is_client_error(self) -> bool {
        self.0 >= 400 && self.0 < 500
    }

    /// Returns `true` if this is a server error (5xx) status.
    #[must_use]
    pub const fn is_server_error(self) -> bool {
        self.0 >= 500 && self.0 < 600
    }
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.0, self.reason())
    }
}

// ---------------------------------------------------------------------------
// Headers (case-insensitive)
// ---------------------------------------------------------------------------

/// Case-insensitive HTTP header map.
#[derive(Debug, Clone, Default)]
pub struct Headers {
    entries: Vec<(String, String)>,
}

impl Headers {
    /// Creates an empty header map.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Inserts a header. If the header already exists, it is replaced.
    pub fn set(&mut self, name: &str, value: &str) {
        let lower = name.to_ascii_lowercase();
        for entry in &mut self.entries {
            if entry.0 == lower {
                entry.1 = value.to_string();
                return;
            }
        }
        self.entries.push((lower, value.to_string()));
    }

    /// Appends a header value (for headers that allow multiple values).
    pub fn append(&mut self, name: &str, value: &str) {
        let lower = name.to_ascii_lowercase();
        self.entries.push((lower, value.to_string()));
    }

    /// Gets the first value for the given header name (case-insensitive).
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&str> {
        let lower = name.to_ascii_lowercase();
        self.entries
            .iter()
            .find(|(k, _)| *k == lower)
            .map(|(_, v)| v.as_str())
    }

    /// Gets all values for the given header name (case-insensitive).
    #[must_use]
    pub fn get_all(&self, name: &str) -> Vec<&str> {
        let lower = name.to_ascii_lowercase();
        self.entries
            .iter()
            .filter(|(k, _)| *k == lower)
            .map(|(_, v)| v.as_str())
            .collect()
    }

    /// Removes all entries for the given header name.
    pub fn remove(&mut self, name: &str) {
        let lower = name.to_ascii_lowercase();
        self.entries.retain(|(k, _)| *k != lower);
    }

    /// Returns `true` if the header exists.
    #[must_use]
    pub fn contains(&self, name: &str) -> bool {
        let lower = name.to_ascii_lowercase();
        self.entries.iter().any(|(k, _)| *k == lower)
    }

    /// Returns the number of header entries.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns `true` if there are no headers.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns an iterator over (name, value) pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.entries.iter().map(|(k, v)| (k.as_str(), v.as_str()))
    }

    /// Serializes headers to HTTP/1.1 wire format.
    #[must_use]
    pub fn to_http1(&self) -> String {
        let mut out = String::new();
        for (k, v) in &self.entries {
            out.push_str(k);
            out.push_str(": ");
            out.push_str(v);
            out.push_str("\r\n");
        }
        out
    }
}

// ---------------------------------------------------------------------------
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

// ---------------------------------------------------------------------------
// URI
// ---------------------------------------------------------------------------

/// A parsed URI (path + optional query + optional fragment).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Uri {
    raw: String,
    path: String,
    query: Option<String>,
    fragment: Option<String>,
}

impl Uri {
    /// Parses a URI string.
    ///
    /// # Errors
    ///
    /// Returns `HttpError::InvalidUri` if the string is empty.
    pub fn parse(raw: &str) -> Result<Self, HttpError> {
        if raw.is_empty() {
            return Err(HttpError::InvalidUri);
        }

        let (before_frag, fragment) = raw.find('#').map_or((raw, None), |pos| {
            (&raw[..pos], Some(raw[pos + 1..].to_string()))
        });

        let (path, query) = before_frag.find('?').map_or_else(
            || (before_frag.to_string(), None),
            |pos| {
                (
                    before_frag[..pos].to_string(),
                    Some(before_frag[pos + 1..].to_string()),
                )
            },
        );

        Ok(Self {
            raw: raw.to_string(),
            path,
            query,
            fragment,
        })
    }

    /// Returns the full raw URI.
    #[must_use]
    pub fn raw(&self) -> &str {
        &self.raw
    }

    /// Returns the path component.
    #[must_use]
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Returns the query string, if present.
    #[must_use]
    pub fn query(&self) -> Option<&str> {
        self.query.as_deref()
    }

    /// Returns the fragment, if present.
    #[must_use]
    pub fn fragment(&self) -> Option<&str> {
        self.fragment.as_deref()
    }

    /// Parses the query string into key-value pairs.
    #[must_use]
    pub fn query_params(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        if let Some(q) = &self.query {
            for pair in q.split('&') {
                if let Some((k, v)) = pair.split_once('=') {
                    map.insert(k.to_string(), v.to_string());
                } else if !pair.is_empty() {
                    map.insert(pair.to_string(), String::new());
                }
            }
        }
        map
    }
}

impl fmt::Display for Uri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.raw)
    }
}

// ---------------------------------------------------------------------------
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

// ---------------------------------------------------------------------------
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

// ---------------------------------------------------------------------------
// Chunked Transfer Encoding
// ---------------------------------------------------------------------------

/// Encoder/decoder for HTTP chunked transfer encoding.
pub struct ChunkedEncoding;

impl ChunkedEncoding {
    /// Encodes a body into chunked transfer encoding format.
    #[must_use]
    pub fn encode(data: &[u8], chunk_size: usize) -> Vec<u8> {
        let mut out = Vec::new();
        let size = if chunk_size == 0 {
            data.len().max(1)
        } else {
            chunk_size
        };

        for chunk in data.chunks(size) {
            let header = format!("{:x}\r\n", chunk.len());
            out.extend_from_slice(header.as_bytes());
            out.extend_from_slice(chunk);
            out.extend_from_slice(b"\r\n");
        }
        out.extend_from_slice(b"0\r\n\r\n");
        out
    }

    /// Decodes a chunked transfer encoded body.
    ///
    /// # Errors
    ///
    /// Returns `HttpError::InvalidChunk` if the encoding is malformed.
    pub fn decode(data: &[u8]) -> Result<Vec<u8>, HttpError> {
        let text = std::str::from_utf8(data).map_err(|_| HttpError::InvalidChunk)?;
        let mut out = Vec::new();
        let mut rest = text;

        loop {
            let (size_line, after) = rest.split_once("\r\n").ok_or(HttpError::InvalidChunk)?;

            let size_str = size_line.split(';').next().unwrap_or(size_line).trim();
            let size = usize::from_str_radix(size_str, 16).map_err(|_| HttpError::InvalidChunk)?;

            if size == 0 {
                break;
            }

            if after.len() < size {
                return Err(HttpError::InvalidChunk);
            }

            out.extend_from_slice(&after.as_bytes()[..size]);

            let remaining = &after[size..];
            rest = remaining
                .strip_prefix("\r\n")
                .ok_or(HttpError::InvalidChunk)?;
        }

        Ok(out)
    }
}

// ---------------------------------------------------------------------------
// MIME / Media Type
// ---------------------------------------------------------------------------

/// An HTTP media type (MIME type).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MediaType {
    pub main_type: String,
    pub sub_type: String,
    pub params: HashMap<String, String>,
}

impl MediaType {
    // Common types
    pub const TEXT_PLAIN: &'static str = "text/plain";
    pub const TEXT_HTML: &'static str = "text/html";
    pub const TEXT_CSS: &'static str = "text/css";
    pub const TEXT_JAVASCRIPT: &'static str = "text/javascript";
    pub const APPLICATION_JSON: &'static str = "application/json";
    pub const APPLICATION_XML: &'static str = "application/xml";
    pub const APPLICATION_OCTET_STREAM: &'static str = "application/octet-stream";
    pub const APPLICATION_FORM_URLENCODED: &'static str = "application/x-www-form-urlencoded";
    pub const MULTIPART_FORM_DATA: &'static str = "multipart/form-data";
    pub const IMAGE_PNG: &'static str = "image/png";
    pub const IMAGE_JPEG: &'static str = "image/jpeg";
    pub const IMAGE_GIF: &'static str = "image/gif";
    pub const IMAGE_WEBP: &'static str = "image/webp";
    pub const IMAGE_SVG: &'static str = "image/svg+xml";
    pub const AUDIO_MPEG: &'static str = "audio/mpeg";
    pub const VIDEO_MP4: &'static str = "video/mp4";

    /// Parses a media type string (e.g. `text/html; charset=utf-8`).
    ///
    /// # Errors
    ///
    /// Returns `HttpError::InvalidMediaType` if the string is malformed.
    pub fn parse(s: &str) -> Result<Self, HttpError> {
        let mut parts = s.split(';');
        let type_part = parts.next().ok_or(HttpError::InvalidMediaType)?.trim();
        let (main, sub) = type_part
            .split_once('/')
            .ok_or(HttpError::InvalidMediaType)?;

        if main.is_empty() || sub.is_empty() {
            return Err(HttpError::InvalidMediaType);
        }

        let mut params = HashMap::new();
        for param in parts {
            let trimmed = param.trim();
            if let Some((k, v)) = trimmed.split_once('=') {
                params.insert(
                    k.trim().to_ascii_lowercase(),
                    v.trim().trim_matches('"').to_string(),
                );
            }
        }

        Ok(Self {
            main_type: main.to_ascii_lowercase(),
            sub_type: sub.to_ascii_lowercase(),
            params,
        })
    }

    /// Returns the full MIME type string (without parameters).
    #[must_use]
    pub fn essence(&self) -> String {
        format!("{}/{}", self.main_type, self.sub_type)
    }

    /// Returns the charset parameter, if present.
    #[must_use]
    pub fn charset(&self) -> Option<&str> {
        self.params.get("charset").map(String::as_str)
    }

    /// Guesses a MIME type from a file extension.
    #[must_use]
    pub fn from_extension(ext: &str) -> &'static str {
        match ext.to_ascii_lowercase().as_str() {
            "html" | "htm" => Self::TEXT_HTML,
            "css" => Self::TEXT_CSS,
            "js" | "mjs" => Self::TEXT_JAVASCRIPT,
            "json" => Self::APPLICATION_JSON,
            "xml" => Self::APPLICATION_XML,
            "txt" => Self::TEXT_PLAIN,
            "png" => Self::IMAGE_PNG,
            "jpg" | "jpeg" => Self::IMAGE_JPEG,
            "gif" => Self::IMAGE_GIF,
            "webp" => Self::IMAGE_WEBP,
            "svg" => Self::IMAGE_SVG,
            "mp3" => Self::AUDIO_MPEG,
            "mp4" => Self::VIDEO_MP4,
            _ => Self::APPLICATION_OCTET_STREAM,
        }
    }
}

impl fmt::Display for MediaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.main_type, self.sub_type)?;
        for (k, v) in &self.params {
            write!(f, "; {k}={v}")?;
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Cookie
// ---------------------------------------------------------------------------

/// An HTTP cookie.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub path: Option<String>,
    pub domain: Option<String>,
    pub max_age: Option<i64>,
    pub secure: bool,
    pub http_only: bool,
    pub same_site: Option<SameSite>,
}

/// `SameSite` cookie attribute.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SameSite {
    Strict,
    Lax,
    None,
}

impl fmt::Display for SameSite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Strict => f.write_str("Strict"),
            Self::Lax => f.write_str("Lax"),
            Self::None => f.write_str("None"),
        }
    }
}

impl Cookie {
    /// Creates a new cookie with name and value.
    #[must_use]
    pub fn new(name: &str, value: &str) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
            path: None,
            domain: None,
            max_age: None,
            secure: false,
            http_only: false,
            same_site: None,
        }
    }

    /// Parses a `Set-Cookie` header value.
    ///
    /// # Errors
    ///
    /// Returns `HttpError::InvalidCookie` if the string is malformed.
    pub fn parse_set_cookie(s: &str) -> Result<Self, HttpError> {
        let mut parts = s.split(';');
        let first = parts.next().ok_or(HttpError::InvalidCookie)?;
        let (name, value) = first.split_once('=').ok_or(HttpError::InvalidCookie)?;

        let name = name.trim();
        if name.is_empty() {
            return Err(HttpError::InvalidCookie);
        }

        let mut cookie = Self::new(name, value.trim());

        for part in parts {
            let trimmed = part.trim();
            let lower = trimmed.to_ascii_lowercase();

            if lower == "secure" {
                cookie.secure = true;
            } else if lower == "httponly" {
                cookie.http_only = true;
            } else if let Some((k, v)) = trimmed.split_once('=') {
                let key_lower = k.trim().to_ascii_lowercase();
                let val = v.trim();
                match key_lower.as_str() {
                    "path" => cookie.path = Some(val.to_string()),
                    "domain" => cookie.domain = Some(val.to_string()),
                    "max-age" => {
                        cookie.max_age = val.parse().ok();
                    }
                    "samesite" => {
                        cookie.same_site = match val.to_ascii_lowercase().as_str() {
                            "strict" => Some(SameSite::Strict),
                            "lax" => Some(SameSite::Lax),
                            "none" => Some(SameSite::None),
                            _ => None,
                        };
                    }
                    _ => {}
                }
            }
        }

        Ok(cookie)
    }

    /// Parses a `Cookie` request header into a list of name-value pairs.
    #[must_use]
    pub fn parse_cookie_header(s: &str) -> Vec<(String, String)> {
        let mut out = Vec::new();
        for part in s.split(';') {
            let trimmed = part.trim();
            if let Some((k, v)) = trimmed.split_once('=') {
                out.push((k.trim().to_string(), v.trim().to_string()));
            }
        }
        out
    }

    /// Serializes to `Set-Cookie` header value.
    #[must_use]
    pub fn to_set_cookie(&self) -> String {
        let mut out = format!("{}={}", self.name, self.value);
        if let Some(ref path) = self.path {
            out.push_str("; Path=");
            out.push_str(path);
        }
        if let Some(ref domain) = self.domain {
            out.push_str("; Domain=");
            out.push_str(domain);
        }
        if let Some(max_age) = self.max_age {
            use std::fmt::Write;
            let _ = write!(out, "; Max-Age={max_age}");
        }
        if self.secure {
            out.push_str("; Secure");
        }
        if self.http_only {
            out.push_str("; HttpOnly");
        }
        if let Some(ref ss) = self.same_site {
            use std::fmt::Write;
            let _ = write!(out, "; SameSite={ss}");
        }
        out
    }
}

// ---------------------------------------------------------------------------
// Content Negotiation
// ---------------------------------------------------------------------------

/// Content negotiation utilities.
pub struct ContentNegotiation;

/// A parsed `Accept` header entry with quality factor.
#[derive(Debug, Clone)]
pub struct AcceptEntry {
    pub media_type: String,
    pub quality: f32,
}

impl ContentNegotiation {
    /// Parses an `Accept` header into entries sorted by quality (descending).
    #[must_use]
    pub fn parse_accept(header: &str) -> Vec<AcceptEntry> {
        let mut entries: Vec<AcceptEntry> = header
            .split(',')
            .filter_map(|part| {
                let trimmed = part.trim();
                if trimmed.is_empty() {
                    return None;
                }

                let mut segments = trimmed.split(';');
                let media_type = segments.next()?.trim().to_string();
                let mut quality: f32 = 1.0;

                for seg in segments {
                    let seg = seg.trim();
                    if let Some(q_val) = seg.strip_prefix("q=") {
                        quality = q_val.parse().unwrap_or(1.0);
                    }
                }

                Some(AcceptEntry {
                    media_type,
                    quality,
                })
            })
            .collect();

        entries.sort_by(|a, b| {
            b.quality
                .partial_cmp(&a.quality)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        entries
    }

    /// Selects the best matching media type from a list of available types.
    #[must_use]
    pub fn negotiate<'a>(accept: &[AcceptEntry], available: &[&'a str]) -> Option<&'a str> {
        for entry in accept {
            if entry.media_type == "*/*" {
                return available.first().copied();
            }

            for avail in available {
                if *avail == entry.media_type {
                    return Some(avail);
                }
            }

            // Check wildcard sub-type (e.g. text/*)
            if let Some(main) = entry.media_type.strip_suffix("/*") {
                for avail in available {
                    if avail.starts_with(main) && avail.as_bytes().get(main.len()) == Some(&b'/') {
                        return Some(avail);
                    }
                }
            }
        }
        None
    }

    /// Parses an `Accept-Encoding` header and returns encodings sorted by quality.
    #[must_use]
    pub fn parse_accept_encoding(header: &str) -> Vec<AcceptEntry> {
        Self::parse_accept(header)
    }

    /// Parses an `Accept-Language` header and returns languages sorted by quality.
    #[must_use]
    pub fn parse_accept_language(header: &str) -> Vec<AcceptEntry> {
        Self::parse_accept(header)
    }
}

// ---------------------------------------------------------------------------
// HTTP/2 Frame Types (simplified binary frame parser)
// ---------------------------------------------------------------------------

/// HTTP/2 frame types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum H2FrameType {
    Data,
    Headers,
    Priority,
    RstStream,
    Settings,
    PushPromise,
    Ping,
    GoAway,
    WindowUpdate,
    Continuation,
    Unknown(u8),
}

impl H2FrameType {
    /// Converts a raw byte to a frame type.
    #[must_use]
    pub const fn from_u8(v: u8) -> Self {
        match v {
            0 => Self::Data,
            1 => Self::Headers,
            2 => Self::Priority,
            3 => Self::RstStream,
            4 => Self::Settings,
            5 => Self::PushPromise,
            6 => Self::Ping,
            7 => Self::GoAway,
            8 => Self::WindowUpdate,
            9 => Self::Continuation,
            other => Self::Unknown(other),
        }
    }

    /// Converts to the raw byte value.
    #[must_use]
    pub const fn to_u8(self) -> u8 {
        match self {
            Self::Data => 0,
            Self::Headers => 1,
            Self::Priority => 2,
            Self::RstStream => 3,
            Self::Settings => 4,
            Self::PushPromise => 5,
            Self::Ping => 6,
            Self::GoAway => 7,
            Self::WindowUpdate => 8,
            Self::Continuation => 9,
            Self::Unknown(v) => v,
        }
    }
}

/// An HTTP/2 frame header (9 bytes).
#[derive(Debug, Clone)]
pub struct H2Frame {
    pub length: u32,
    pub frame_type: H2FrameType,
    pub flags: u8,
    pub stream_id: u32,
    pub payload: Vec<u8>,
}

impl H2Frame {
    /// The HTTP/2 connection preface.
    pub const CONNECTION_PREFACE: &'static [u8] = b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n";

    /// Frame header size in bytes.
    pub const HEADER_SIZE: usize = 9;

    /// Parses an HTTP/2 frame from raw bytes.
    ///
    /// # Errors
    ///
    /// Returns `HttpError::Incomplete` if there are not enough bytes.
    pub fn parse(data: &[u8]) -> Result<Self, HttpError> {
        if data.len() < Self::HEADER_SIZE {
            return Err(HttpError::Incomplete);
        }

        let length = u32::from(data[0]) << 16 | u32::from(data[1]) << 8 | u32::from(data[2]);
        let frame_type = H2FrameType::from_u8(data[3]);
        let flags = data[4];
        let stream_id = u32::from_be_bytes([data[5] & 0x7F, data[6], data[7], data[8]]);

        let total = Self::HEADER_SIZE + length as usize;
        if data.len() < total {
            return Err(HttpError::Incomplete);
        }

        let payload = data[Self::HEADER_SIZE..total].to_vec();

        Ok(Self {
            length,
            frame_type,
            flags,
            stream_id,
            payload,
        })
    }

    /// Serializes this frame to bytes.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn to_bytes(&self) -> Vec<u8> {
        let len = self.payload.len() as u32;
        let mut out = Vec::with_capacity(Self::HEADER_SIZE + self.payload.len());
        out.push((len >> 16) as u8);
        out.push((len >> 8) as u8);
        out.push(len as u8);
        out.push(self.frame_type.to_u8());
        out.push(self.flags);
        let sid = self.stream_id.to_be_bytes();
        out.push(sid[0] & 0x7F);
        out.push(sid[1]);
        out.push(sid[2]);
        out.push(sid[3]);
        out.extend_from_slice(&self.payload);
        out
    }

    /// Creates a SETTINGS frame.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn settings(stream_id: u32, payload: &[u8]) -> Self {
        Self {
            length: payload.len() as u32,
            frame_type: H2FrameType::Settings,
            flags: 0,
            stream_id,
            payload: payload.to_vec(),
        }
    }

    /// Creates a PING frame.
    #[must_use]
    pub fn ping(data: &[u8; 8]) -> Self {
        Self {
            length: 8,
            frame_type: H2FrameType::Ping,
            flags: 0,
            stream_id: 0,
            payload: data.to_vec(),
        }
    }

    /// Creates a `WINDOW_UPDATE` frame.
    #[must_use]
    pub fn window_update(stream_id: u32, increment: u32) -> Self {
        Self {
            length: 4,
            frame_type: H2FrameType::WindowUpdate,
            flags: 0,
            stream_id,
            payload: increment.to_be_bytes().to_vec(),
        }
    }

    /// Creates a GOAWAY frame.
    #[must_use]
    pub fn goaway(last_stream_id: u32, error_code: u32) -> Self {
        let mut payload = Vec::with_capacity(8);
        payload.extend_from_slice(&last_stream_id.to_be_bytes());
        payload.extend_from_slice(&error_code.to_be_bytes());
        Self {
            length: 8,
            frame_type: H2FrameType::GoAway,
            flags: 0,
            stream_id: 0,
            payload,
        }
    }

    /// Returns `true` if the `END_STREAM` flag is set.
    #[must_use]
    pub const fn is_end_stream(&self) -> bool {
        self.flags & 0x01 != 0
    }

    /// Returns `true` if the `END_HEADERS` flag is set.
    #[must_use]
    pub const fn is_end_headers(&self) -> bool {
        self.flags & 0x04 != 0
    }

    /// Returns `true` if the ACK flag is set.
    #[must_use]
    pub const fn is_ack(&self) -> bool {
        self.flags & 0x01 != 0
    }
}

// ---------------------------------------------------------------------------
// HPACK Integer Encoding (HTTP/2 header compression primitive)
// ---------------------------------------------------------------------------

/// HPACK integer encoding/decoding for HTTP/2.
pub struct HpackInt;

impl HpackInt {
    /// Encodes an integer with the given prefix size (1..=8).
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn encode(mut value: u64, prefix_bits: u8) -> Vec<u8> {
        let max_prefix = (1u64 << prefix_bits) - 1;
        if value < max_prefix {
            return vec![value as u8];
        }

        let mut out = vec![max_prefix as u8];
        value -= max_prefix;
        while value >= 128 {
            out.push((value & 0x7F) as u8 | 0x80);
            value >>= 7;
        }
        out.push(value as u8);
        out
    }

    /// Decodes an integer with the given prefix size from a byte slice.
    /// Returns `(value, bytes_consumed)`.
    ///
    /// # Errors
    ///
    /// Returns `HttpError::Incomplete` if the data is insufficient.
    pub fn decode(data: &[u8], prefix_bits: u8) -> Result<(u64, usize), HttpError> {
        if data.is_empty() {
            return Err(HttpError::Incomplete);
        }

        let max_prefix = (1u64 << prefix_bits) - 1;
        let first = u64::from(data[0]) & max_prefix;

        if first < max_prefix {
            return Ok((first, 1));
        }

        let mut value = max_prefix;
        let mut shift = 0u32;
        let mut i = 1;

        loop {
            if i >= data.len() {
                return Err(HttpError::Incomplete);
            }
            let byte = u64::from(data[i]);
            value += (byte & 0x7F) << shift;
            i += 1;
            if byte & 0x80 == 0 {
                break;
            }
            shift += 7;
        }

        Ok((value, i))
    }
}

// ---------------------------------------------------------------------------
// URL Encoding
// ---------------------------------------------------------------------------

/// Percent-encoding utilities.
pub struct UrlEncoding;

impl UrlEncoding {
    /// Percent-encodes a string (RFC 3986 unreserved characters preserved).
    #[must_use]
    pub fn encode(input: &str) -> String {
        let mut out = String::with_capacity(input.len());
        for byte in input.bytes() {
            if byte.is_ascii_alphanumeric() || b"-._~".contains(&byte) {
                out.push(byte as char);
            } else {
                out.push('%');
                out.push(Self::to_hex_upper(byte >> 4));
                out.push(Self::to_hex_upper(byte & 0x0F));
            }
        }
        out
    }

    /// Decodes a percent-encoded string.
    ///
    /// # Errors
    ///
    /// Returns `HttpError::InvalidUri` on malformed sequences.
    pub fn decode(input: &str) -> Result<String, HttpError> {
        let mut bytes = Vec::with_capacity(input.len());
        let input_bytes = input.as_bytes();
        let mut i = 0;
        while i < input_bytes.len() {
            if input_bytes[i] == b'%' {
                if i + 2 >= input_bytes.len() {
                    return Err(HttpError::InvalidUri);
                }
                let hi = Self::from_hex(input_bytes[i + 1]).ok_or(HttpError::InvalidUri)?;
                let lo = Self::from_hex(input_bytes[i + 2]).ok_or(HttpError::InvalidUri)?;
                bytes.push(hi << 4 | lo);
                i += 3;
            } else if input_bytes[i] == b'+' {
                bytes.push(b' ');
                i += 1;
            } else {
                bytes.push(input_bytes[i]);
                i += 1;
            }
        }
        String::from_utf8(bytes).map_err(|_| HttpError::InvalidUri)
    }

    fn to_hex_upper(nibble: u8) -> char {
        match nibble {
            0..=9 => (b'0' + nibble) as char,
            10..=15 => (b'A' + nibble - 10) as char,
            _ => unreachable!(),
        }
    }

    const fn from_hex(byte: u8) -> Option<u8> {
        match byte {
            b'0'..=b'9' => Some(byte - b'0'),
            b'a'..=b'f' => Some(byte - b'a' + 10),
            b'A'..=b'F' => Some(byte - b'A' + 10),
            _ => None,
        }
    }
}

// ---------------------------------------------------------------------------
// Form Data Parsing
// ---------------------------------------------------------------------------

/// Parses `application/x-www-form-urlencoded` data.
pub struct FormData;

impl FormData {
    /// Parses URL-encoded form data into key-value pairs.
    ///
    /// # Errors
    ///
    /// Returns `HttpError::InvalidUri` on malformed percent-encoding.
    pub fn parse(data: &str) -> Result<HashMap<String, String>, HttpError> {
        let mut map = HashMap::new();
        for pair in data.split('&') {
            if pair.is_empty() {
                continue;
            }
            let (k, v) = if let Some((k, v)) = pair.split_once('=') {
                (UrlEncoding::decode(k)?, UrlEncoding::decode(v)?)
            } else {
                (UrlEncoding::decode(pair)?, String::new())
            };
            map.insert(k, v);
        }
        Ok(map)
    }

    /// Encodes key-value pairs to `application/x-www-form-urlencoded` format.
    #[must_use]
    pub fn encode(pairs: &[(&str, &str)]) -> String {
        pairs
            .iter()
            .map(|(k, v)| format!("{}={}", UrlEncoding::encode(k), UrlEncoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&")
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- Method ---

    #[test]
    fn method_from_str_valid() {
        assert_eq!("GET".parse::<Method>().unwrap(), Method::Get);
        assert_eq!("POST".parse::<Method>().unwrap(), Method::Post);
        assert_eq!("PUT".parse::<Method>().unwrap(), Method::Put);
        assert_eq!("DELETE".parse::<Method>().unwrap(), Method::Delete);
        assert_eq!("PATCH".parse::<Method>().unwrap(), Method::Patch);
        assert_eq!("HEAD".parse::<Method>().unwrap(), Method::Head);
        assert_eq!("OPTIONS".parse::<Method>().unwrap(), Method::Options);
        assert_eq!("TRACE".parse::<Method>().unwrap(), Method::Trace);
        assert_eq!("CONNECT".parse::<Method>().unwrap(), Method::Connect);
    }

    #[test]
    fn method_from_str_invalid() {
        assert_eq!("INVALID".parse::<Method>(), Err(HttpError::InvalidMethod));
    }

    #[test]
    fn method_display() {
        assert_eq!(Method::Get.to_string(), "GET");
        assert_eq!(Method::Post.to_string(), "POST");
    }

    #[test]
    fn method_as_str() {
        assert_eq!(Method::Delete.as_str(), "DELETE");
        assert_eq!(Method::Options.as_str(), "OPTIONS");
    }

    // --- Version ---

    #[test]
    fn version_parse() {
        assert_eq!("HTTP/1.0".parse::<Version>().unwrap(), Version::Http10);
        assert_eq!("HTTP/1.1".parse::<Version>().unwrap(), Version::Http11);
        assert_eq!("HTTP/2".parse::<Version>().unwrap(), Version::Http2);
        assert_eq!("HTTP/2.0".parse::<Version>().unwrap(), Version::Http2);
    }

    #[test]
    fn version_invalid() {
        assert_eq!("HTTP/3".parse::<Version>(), Err(HttpError::InvalidVersion));
    }

    #[test]
    fn version_display() {
        assert_eq!(Version::Http11.to_string(), "HTTP/1.1");
        assert_eq!(Version::Http2.to_string(), "HTTP/2");
    }

    // --- Status Code ---

    #[test]
    fn status_code_constants() {
        assert_eq!(StatusCode::OK.code(), 200);
        assert_eq!(StatusCode::NOT_FOUND.code(), 404);
        assert_eq!(StatusCode::INTERNAL_SERVER_ERROR.code(), 500);
    }

    #[test]
    fn status_code_reason() {
        assert_eq!(StatusCode::OK.reason(), "OK");
        assert_eq!(StatusCode::NOT_FOUND.reason(), "Not Found");
        assert_eq!(StatusCode::CONTINUE.reason(), "Continue");
    }

    #[test]
    fn status_code_display() {
        assert_eq!(StatusCode::OK.to_string(), "200 OK");
        assert_eq!(StatusCode::NOT_FOUND.to_string(), "404 Not Found");
    }

    #[test]
    fn status_code_categories() {
        assert!(StatusCode::CONTINUE.is_informational());
        assert!(!StatusCode::CONTINUE.is_success());
        assert!(StatusCode::OK.is_success());
        assert!(StatusCode::MOVED_PERMANENTLY.is_redirection());
        assert!(StatusCode::BAD_REQUEST.is_client_error());
        assert!(StatusCode::INTERNAL_SERVER_ERROR.is_server_error());
    }

    #[test]
    fn status_code_from_u16_valid() {
        assert_eq!(StatusCode::from_u16(200).unwrap().code(), 200);
        assert_eq!(StatusCode::from_u16(999).unwrap().code(), 999);
    }

    #[test]
    fn status_code_from_u16_invalid() {
        assert!(StatusCode::from_u16(0).is_err());
        assert!(StatusCode::from_u16(99).is_err());
        assert!(StatusCode::from_u16(1000).is_err());
    }

    #[test]
    fn status_code_unknown_reason() {
        assert_eq!(StatusCode::from_u16(999).unwrap().reason(), "Unknown");
    }

    // --- Headers ---

    #[test]
    fn headers_set_and_get() {
        let mut h = Headers::new();
        h.set("Content-Type", "text/html");
        assert_eq!(h.get("content-type"), Some("text/html"));
        assert_eq!(h.get("CONTENT-TYPE"), Some("text/html"));
    }

    #[test]
    fn headers_overwrite() {
        let mut h = Headers::new();
        h.set("Host", "a.com");
        h.set("Host", "b.com");
        assert_eq!(h.get("host"), Some("b.com"));
        assert_eq!(h.len(), 1);
    }

    #[test]
    fn headers_append() {
        let mut h = Headers::new();
        h.append("Set-Cookie", "a=1");
        h.append("Set-Cookie", "b=2");
        let all = h.get_all("set-cookie");
        assert_eq!(all.len(), 2);
    }

    #[test]
    fn headers_remove() {
        let mut h = Headers::new();
        h.set("Host", "example.com");
        h.remove("HOST");
        assert!(h.get("host").is_none());
        assert!(h.is_empty());
    }

    #[test]
    fn headers_contains() {
        let mut h = Headers::new();
        h.set("Accept", "*/*");
        assert!(h.contains("accept"));
        assert!(!h.contains("host"));
    }

    #[test]
    fn headers_iter() {
        let mut h = Headers::new();
        h.set("A", "1");
        h.set("B", "2");
        let pairs: Vec<_> = h.iter().collect();
        assert_eq!(pairs.len(), 2);
    }

    #[test]
    fn headers_to_http1() {
        let mut h = Headers::new();
        h.set("Host", "example.com");
        let s = h.to_http1();
        assert!(s.contains("host: example.com\r\n"));
    }

    // --- URI ---

    #[test]
    fn uri_parse_simple() {
        let uri = Uri::parse("/path").unwrap();
        assert_eq!(uri.path(), "/path");
        assert!(uri.query().is_none());
        assert!(uri.fragment().is_none());
    }

    #[test]
    fn uri_parse_with_query() {
        let uri = Uri::parse("/search?q=hello&lang=en").unwrap();
        assert_eq!(uri.path(), "/search");
        assert_eq!(uri.query(), Some("q=hello&lang=en"));
    }

    #[test]
    fn uri_parse_with_fragment() {
        let uri = Uri::parse("/page#section").unwrap();
        assert_eq!(uri.path(), "/page");
        assert_eq!(uri.fragment(), Some("section"));
    }

    #[test]
    fn uri_parse_full() {
        let uri = Uri::parse("/a?b=c#d").unwrap();
        assert_eq!(uri.path(), "/a");
        assert_eq!(uri.query(), Some("b=c"));
        assert_eq!(uri.fragment(), Some("d"));
    }

    #[test]
    fn uri_parse_empty() {
        assert!(Uri::parse("").is_err());
    }

    #[test]
    fn uri_query_params() {
        let uri = Uri::parse("/s?a=1&b=2&c").unwrap();
        let params = uri.query_params();
        assert_eq!(params.get("a"), Some(&"1".to_string()));
        assert_eq!(params.get("b"), Some(&"2".to_string()));
        assert_eq!(params.get("c"), Some(&String::new()));
    }

    #[test]
    fn uri_display() {
        let uri = Uri::parse("/test?x=1").unwrap();
        assert_eq!(uri.to_string(), "/test?x=1");
    }

    #[test]
    fn uri_raw() {
        let uri = Uri::parse("/hello?world#foo").unwrap();
        assert_eq!(uri.raw(), "/hello?world#foo");
    }

    // --- Request ---

    #[test]
    fn request_parse_get() {
        let raw = b"GET /index.html HTTP/1.1\r\nHost: example.com\r\n\r\n";
        let req = Request::parse(raw).unwrap();
        assert_eq!(req.method, Method::Get);
        assert_eq!(req.uri.path(), "/index.html");
        assert_eq!(req.version, Version::Http11);
        assert_eq!(req.headers.get("host"), Some("example.com"));
    }

    #[test]
    fn request_parse_post_with_body() {
        let raw = b"POST /api HTTP/1.1\r\nContent-Length: 5\r\n\r\nhello";
        let req = Request::parse(raw).unwrap();
        assert_eq!(req.method, Method::Post);
        assert_eq!(req.body, b"hello");
        assert_eq!(req.content_length(), Some(5));
    }

    #[test]
    fn request_builder() {
        let req = Request::builder()
            .method(Method::Put)
            .uri("/resource")
            .header("Content-Type", "application/json")
            .body(b"{}")
            .build()
            .unwrap();
        assert_eq!(req.method, Method::Put);
        assert_eq!(req.uri.path(), "/resource");
        assert_eq!(req.content_type(), Some("application/json"));
    }

    #[test]
    fn request_roundtrip() {
        let req = Request::builder()
            .method(Method::Get)
            .uri("/test")
            .header("Host", "localhost")
            .build()
            .unwrap();
        let bytes = req.to_bytes();
        let parsed = Request::parse(&bytes).unwrap();
        assert_eq!(parsed.method, Method::Get);
        assert_eq!(parsed.uri.path(), "/test");
    }

    #[test]
    fn request_parse_incomplete() {
        let raw = b"GET /test HTTP/1.1\r\nHost: x";
        assert!(Request::parse(raw).is_err());
    }

    #[test]
    fn request_parse_invalid_method() {
        let raw = b"FOOBAR /test HTTP/1.1\r\n\r\n";
        assert!(Request::parse(raw).is_err());
    }

    // --- Response ---

    #[test]
    fn response_parse_ok() {
        let raw = b"HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<h1>Hi</h1>";
        let resp = Response::parse(raw).unwrap();
        assert_eq!(resp.status, StatusCode::OK);
        assert_eq!(resp.content_type(), Some("text/html"));
        assert_eq!(resp.body, b"<h1>Hi</h1>");
    }

    #[test]
    fn response_parse_404() {
        let raw = b"HTTP/1.1 404 Not Found\r\n\r\n";
        let resp = Response::parse(raw).unwrap();
        assert_eq!(resp.status, StatusCode::NOT_FOUND);
    }

    #[test]
    fn response_builder() {
        let resp = Response::builder()
            .status(StatusCode::CREATED)
            .header("Location", "/new")
            .body(b"created")
            .build();
        assert_eq!(resp.status, StatusCode::CREATED);
        assert_eq!(resp.headers.get("location"), Some("/new"));
    }

    #[test]
    fn response_roundtrip() {
        let resp = Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "text/plain")
            .body(b"hello")
            .build();
        let bytes = resp.to_bytes();
        let parsed = Response::parse(&bytes).unwrap();
        assert_eq!(parsed.status, StatusCode::OK);
        assert_eq!(parsed.body, b"hello");
    }

    #[test]
    fn response_content_length() {
        let resp = Response::builder().header("Content-Length", "42").build();
        assert_eq!(resp.content_length(), Some(42));
    }

    #[test]
    fn response_version() {
        let resp = Response::builder().version(Version::Http10).build();
        assert_eq!(resp.version, Version::Http10);
    }

    // --- Chunked Encoding ---

    #[test]
    fn chunked_encode_decode() {
        let data = b"Hello, World!";
        let encoded = ChunkedEncoding::encode(data, 5);
        let decoded = ChunkedEncoding::decode(&encoded).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn chunked_single_chunk() {
        let data = b"abc";
        let encoded = ChunkedEncoding::encode(data, 100);
        let decoded = ChunkedEncoding::decode(&encoded).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn chunked_decode_with_extension() {
        let encoded = b"5;ext=val\r\nhello\r\n0\r\n\r\n";
        let decoded = ChunkedEncoding::decode(encoded).unwrap();
        assert_eq!(decoded, b"hello");
    }

    #[test]
    fn chunked_decode_invalid() {
        assert!(ChunkedEncoding::decode(b"xyz\r\n").is_err());
    }

    #[test]
    fn chunked_empty() {
        let encoded = b"0\r\n\r\n";
        let decoded = ChunkedEncoding::decode(encoded).unwrap();
        assert!(decoded.is_empty());
    }

    // --- Media Type ---

    #[test]
    fn media_type_parse_simple() {
        let mt = MediaType::parse("text/html").unwrap();
        assert_eq!(mt.main_type, "text");
        assert_eq!(mt.sub_type, "html");
        assert_eq!(mt.essence(), "text/html");
    }

    #[test]
    fn media_type_parse_with_charset() {
        let mt = MediaType::parse("text/html; charset=utf-8").unwrap();
        assert_eq!(mt.charset(), Some("utf-8"));
    }

    #[test]
    fn media_type_parse_with_quoted_param() {
        let mt = MediaType::parse("text/html; charset=\"utf-8\"").unwrap();
        assert_eq!(mt.charset(), Some("utf-8"));
    }

    #[test]
    fn media_type_display() {
        let mt = MediaType::parse("application/json").unwrap();
        assert_eq!(mt.to_string(), "application/json");
    }

    #[test]
    fn media_type_invalid() {
        assert!(MediaType::parse("invalid").is_err());
        assert!(MediaType::parse("/html").is_err());
        assert!(MediaType::parse("text/").is_err());
    }

    #[test]
    fn media_type_from_extension() {
        assert_eq!(MediaType::from_extension("html"), "text/html");
        assert_eq!(MediaType::from_extension("json"), "application/json");
        assert_eq!(MediaType::from_extension("png"), "image/png");
        assert_eq!(MediaType::from_extension("jpg"), "image/jpeg");
        assert_eq!(MediaType::from_extension("css"), "text/css");
        assert_eq!(MediaType::from_extension("js"), "text/javascript");
        assert_eq!(
            MediaType::from_extension("unknown"),
            "application/octet-stream"
        );
    }

    #[test]
    fn media_type_from_extension_case_insensitive() {
        assert_eq!(MediaType::from_extension("HTML"), "text/html");
        assert_eq!(MediaType::from_extension("JSON"), "application/json");
    }

    #[test]
    fn media_type_constants() {
        assert_eq!(MediaType::TEXT_PLAIN, "text/plain");
        assert_eq!(MediaType::APPLICATION_JSON, "application/json");
        assert_eq!(MediaType::IMAGE_WEBP, "image/webp");
    }

    // --- Cookie ---

    #[test]
    fn cookie_new() {
        let c = Cookie::new("session", "abc123");
        assert_eq!(c.name, "session");
        assert_eq!(c.value, "abc123");
        assert!(!c.secure);
        assert!(!c.http_only);
    }

    #[test]
    fn cookie_parse_set_cookie_simple() {
        let c = Cookie::parse_set_cookie("session=abc; Path=/; Secure; HttpOnly").unwrap();
        assert_eq!(c.name, "session");
        assert_eq!(c.value, "abc");
        assert_eq!(c.path, Some("/".to_string()));
        assert!(c.secure);
        assert!(c.http_only);
    }

    #[test]
    fn cookie_parse_set_cookie_full() {
        let c = Cookie::parse_set_cookie(
            "id=42; Path=/api; Domain=example.com; Max-Age=3600; SameSite=Strict; Secure",
        )
        .unwrap();
        assert_eq!(c.name, "id");
        assert_eq!(c.value, "42");
        assert_eq!(c.domain, Some("example.com".to_string()));
        assert_eq!(c.max_age, Some(3600));
        assert_eq!(c.same_site, Some(SameSite::Strict));
        assert!(c.secure);
    }

    #[test]
    fn cookie_parse_set_cookie_samesite_lax() {
        let c = Cookie::parse_set_cookie("x=1; SameSite=Lax").unwrap();
        assert_eq!(c.same_site, Some(SameSite::Lax));
    }

    #[test]
    fn cookie_parse_set_cookie_samesite_none() {
        let c = Cookie::parse_set_cookie("x=1; SameSite=None").unwrap();
        assert_eq!(c.same_site, Some(SameSite::None));
    }

    #[test]
    fn cookie_parse_invalid() {
        assert!(Cookie::parse_set_cookie("").is_err());
        assert!(Cookie::parse_set_cookie("=value").is_err());
    }

    #[test]
    fn cookie_to_set_cookie() {
        let mut c = Cookie::new("tok", "xyz");
        c.path = Some("/".to_string());
        c.secure = true;
        c.http_only = true;
        c.same_site = Some(SameSite::Lax);
        let s = c.to_set_cookie();
        assert!(s.contains("tok=xyz"));
        assert!(s.contains("Path=/"));
        assert!(s.contains("Secure"));
        assert!(s.contains("HttpOnly"));
        assert!(s.contains("SameSite=Lax"));
    }

    #[test]
    fn cookie_parse_header() {
        let cookies = Cookie::parse_cookie_header("a=1; b=2; c=3");
        assert_eq!(cookies.len(), 3);
        assert_eq!(cookies[0], ("a".to_string(), "1".to_string()));
        assert_eq!(cookies[2], ("c".to_string(), "3".to_string()));
    }

    #[test]
    fn cookie_max_age_display() {
        let mut c = Cookie::new("x", "1");
        c.max_age = Some(7200);
        let s = c.to_set_cookie();
        assert!(s.contains("Max-Age=7200"));
    }

    #[test]
    fn cookie_domain_display() {
        let mut c = Cookie::new("x", "1");
        c.domain = Some("example.com".to_string());
        let s = c.to_set_cookie();
        assert!(s.contains("Domain=example.com"));
    }

    // --- Content Negotiation ---

    #[test]
    fn accept_parse_simple() {
        let entries = ContentNegotiation::parse_accept("text/html, application/json");
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].media_type, "text/html");
    }

    #[test]
    fn accept_parse_with_quality() {
        let entries = ContentNegotiation::parse_accept("text/html;q=0.9, application/json;q=1.0");
        assert_eq!(entries[0].media_type, "application/json");
        assert!((entries[0].quality - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn negotiate_exact_match() {
        let accept = ContentNegotiation::parse_accept("application/json, text/html");
        let result = ContentNegotiation::negotiate(&accept, &["text/html", "application/json"]);
        assert_eq!(result, Some("application/json"));
    }

    #[test]
    fn negotiate_wildcard() {
        let accept = ContentNegotiation::parse_accept("*/*");
        let result = ContentNegotiation::negotiate(&accept, &["text/plain"]);
        assert_eq!(result, Some("text/plain"));
    }

    #[test]
    fn negotiate_subtype_wildcard() {
        let accept = ContentNegotiation::parse_accept("text/*");
        let result = ContentNegotiation::negotiate(&accept, &["text/html", "application/json"]);
        assert_eq!(result, Some("text/html"));
    }

    #[test]
    fn negotiate_no_match() {
        let accept = ContentNegotiation::parse_accept("image/png");
        let result = ContentNegotiation::negotiate(&accept, &["text/html"]);
        assert!(result.is_none());
    }

    #[test]
    fn accept_encoding_parse() {
        let entries = ContentNegotiation::parse_accept_encoding("gzip, deflate;q=0.5, br;q=0.8");
        assert_eq!(entries[0].media_type, "gzip");
        assert_eq!(entries[1].media_type, "br");
        assert_eq!(entries[2].media_type, "deflate");
    }

    #[test]
    fn accept_language_parse() {
        let entries = ContentNegotiation::parse_accept_language("en-US, ja;q=0.9, fr;q=0.5");
        assert_eq!(entries[0].media_type, "en-US");
        assert_eq!(entries[1].media_type, "ja");
    }

    // --- HTTP/2 Frames ---

    #[test]
    fn h2_frame_type_roundtrip() {
        for i in 0..=9 {
            let ft = H2FrameType::from_u8(i);
            assert_eq!(ft.to_u8(), i);
        }
    }

    #[test]
    fn h2_frame_type_unknown() {
        let ft = H2FrameType::from_u8(255);
        assert_eq!(ft, H2FrameType::Unknown(255));
        assert_eq!(ft.to_u8(), 255);
    }

    #[test]
    fn h2_frame_parse_settings() {
        let frame = H2Frame::settings(0, &[]);
        let bytes = frame.to_bytes();
        let parsed = H2Frame::parse(&bytes).unwrap();
        assert_eq!(parsed.frame_type, H2FrameType::Settings);
        assert_eq!(parsed.stream_id, 0);
        assert!(parsed.payload.is_empty());
    }

    #[test]
    fn h2_frame_parse_with_payload() {
        let payload = b"hello";
        let mut frame = H2Frame::settings(1, payload);
        frame.frame_type = H2FrameType::Data;
        let bytes = frame.to_bytes();
        let parsed = H2Frame::parse(&bytes).unwrap();
        assert_eq!(parsed.frame_type, H2FrameType::Data);
        assert_eq!(parsed.payload, payload);
    }

    #[test]
    fn h2_frame_parse_incomplete() {
        assert!(H2Frame::parse(&[0; 5]).is_err());
    }

    #[test]
    fn h2_frame_ping() {
        let frame = H2Frame::ping(&[1, 2, 3, 4, 5, 6, 7, 8]);
        let bytes = frame.to_bytes();
        let parsed = H2Frame::parse(&bytes).unwrap();
        assert_eq!(parsed.frame_type, H2FrameType::Ping);
        assert_eq!(parsed.payload, [1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn h2_frame_window_update() {
        let frame = H2Frame::window_update(1, 65535);
        let bytes = frame.to_bytes();
        let parsed = H2Frame::parse(&bytes).unwrap();
        assert_eq!(parsed.frame_type, H2FrameType::WindowUpdate);
        assert_eq!(parsed.stream_id, 1);
    }

    #[test]
    fn h2_frame_goaway() {
        let frame = H2Frame::goaway(0, 0);
        let bytes = frame.to_bytes();
        let parsed = H2Frame::parse(&bytes).unwrap();
        assert_eq!(parsed.frame_type, H2FrameType::GoAway);
        assert_eq!(parsed.length, 8);
    }

    #[test]
    fn h2_frame_flags() {
        let mut frame = H2Frame::settings(0, &[]);
        frame.flags = 0x01; // END_STREAM / ACK
        assert!(frame.is_end_stream());
        assert!(frame.is_ack());

        frame.flags = 0x04; // END_HEADERS
        assert!(frame.is_end_headers());
        assert!(!frame.is_end_stream());
    }

    #[test]
    fn h2_connection_preface() {
        assert!(H2Frame::CONNECTION_PREFACE.starts_with(b"PRI"));
    }

    // --- HPACK Integer ---

    #[test]
    fn hpack_int_small() {
        let encoded = HpackInt::encode(10, 5);
        let (val, consumed) = HpackInt::decode(&encoded, 5).unwrap();
        assert_eq!(val, 10);
        assert_eq!(consumed, 1);
    }

    #[test]
    fn hpack_int_large() {
        let encoded = HpackInt::encode(1337, 5);
        let (val, consumed) = HpackInt::decode(&encoded, 5).unwrap();
        assert_eq!(val, 1337);
        assert!(consumed > 1);
    }

    #[test]
    fn hpack_int_boundary() {
        let encoded = HpackInt::encode(31, 5);
        let (val, _) = HpackInt::decode(&encoded, 5).unwrap();
        assert_eq!(val, 31);
    }

    #[test]
    fn hpack_int_zero() {
        let encoded = HpackInt::encode(0, 5);
        let (val, consumed) = HpackInt::decode(&encoded, 5).unwrap();
        assert_eq!(val, 0);
        assert_eq!(consumed, 1);
    }

    #[test]
    fn hpack_int_decode_empty() {
        assert!(HpackInt::decode(&[], 5).is_err());
    }

    // --- URL Encoding ---

    #[test]
    fn url_encode_simple() {
        assert_eq!(UrlEncoding::encode("hello world"), "hello%20world");
    }

    #[test]
    fn url_encode_special() {
        assert_eq!(UrlEncoding::encode("a&b=c"), "a%26b%3Dc");
    }

    #[test]
    fn url_encode_unreserved() {
        assert_eq!(UrlEncoding::encode("abc-._~123"), "abc-._~123");
    }

    #[test]
    fn url_decode_simple() {
        assert_eq!(UrlEncoding::decode("hello%20world").unwrap(), "hello world");
    }

    #[test]
    fn url_decode_plus() {
        assert_eq!(UrlEncoding::decode("hello+world").unwrap(), "hello world");
    }

    #[test]
    fn url_decode_invalid() {
        assert!(UrlEncoding::decode("hello%2").is_err());
        assert!(UrlEncoding::decode("hello%GG").is_err());
    }

    #[test]
    fn url_roundtrip() {
        let input = "Hello, World! /path?q=1&a=2";
        let encoded = UrlEncoding::encode(input);
        let decoded = UrlEncoding::decode(&encoded).unwrap();
        assert_eq!(decoded, input);
    }

    // --- Form Data ---

    #[test]
    fn form_data_parse() {
        let data = FormData::parse("name=Alice&age=30").unwrap();
        assert_eq!(data.get("name"), Some(&"Alice".to_string()));
        assert_eq!(data.get("age"), Some(&"30".to_string()));
    }

    #[test]
    fn form_data_parse_encoded() {
        let data = FormData::parse("q=hello+world&lang=en").unwrap();
        assert_eq!(data.get("q"), Some(&"hello world".to_string()));
    }

    #[test]
    fn form_data_encode() {
        let encoded = FormData::encode(&[("name", "Alice Bob"), ("age", "30")]);
        assert!(encoded.contains("name=Alice%20Bob"));
        assert!(encoded.contains("age=30"));
    }

    #[test]
    fn form_data_empty() {
        let data = FormData::parse("").unwrap();
        assert!(data.is_empty());
    }

    // --- Error Display ---

    #[test]
    fn error_display() {
        assert_eq!(HttpError::InvalidMethod.to_string(), "invalid HTTP method");
        assert_eq!(
            HttpError::InvalidChunk.to_string(),
            "invalid chunked encoding"
        );
        assert_eq!(HttpError::TooLarge.to_string(), "payload too large");
    }

    #[test]
    fn error_is_std_error() {
        let err: &dyn std::error::Error = &HttpError::InvalidRequest;
        assert!(!err.to_string().is_empty());
    }

    // --- SameSite Display ---

    #[test]
    fn samesite_display() {
        assert_eq!(SameSite::Strict.to_string(), "Strict");
        assert_eq!(SameSite::Lax.to_string(), "Lax");
        assert_eq!(SameSite::None.to_string(), "None");
    }

    // --- Additional edge cases ---

    #[test]
    fn request_with_query_params() {
        let raw = b"GET /search?q=test&page=2 HTTP/1.1\r\nHost: example.com\r\n\r\n";
        let req = Request::parse(raw).unwrap();
        let params = req.uri.query_params();
        assert_eq!(params.get("q"), Some(&"test".to_string()));
        assert_eq!(params.get("page"), Some(&"2".to_string()));
    }

    #[test]
    fn response_parse_with_content_length() {
        let raw = b"HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nhello";
        let resp = Response::parse(raw).unwrap();
        assert_eq!(resp.content_length(), Some(5));
        assert_eq!(resp.body, b"hello");
    }

    #[test]
    fn chunked_multi_chunk() {
        let data = b"abcdefghij";
        let encoded = ChunkedEncoding::encode(data, 3);
        let decoded = ChunkedEncoding::decode(&encoded).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn media_type_with_multiple_params() {
        let mt = MediaType::parse("text/html; charset=utf-8; boundary=something").unwrap();
        assert_eq!(mt.charset(), Some("utf-8"));
        assert_eq!(mt.params.get("boundary"), Some(&"something".to_string()));
    }

    #[test]
    fn headers_default() {
        let h = Headers::default();
        assert!(h.is_empty());
        assert_eq!(h.len(), 0);
    }

    #[test]
    fn version_as_str() {
        assert_eq!(Version::Http10.as_str(), "HTTP/1.0");
        assert_eq!(Version::Http11.as_str(), "HTTP/1.1");
        assert_eq!(Version::Http2.as_str(), "HTTP/2");
    }

    #[test]
    fn status_switching_protocols() {
        assert!(StatusCode::SWITCHING_PROTOCOLS.is_informational());
        assert_eq!(
            StatusCode::SWITCHING_PROTOCOLS.reason(),
            "Switching Protocols"
        );
    }

    #[test]
    fn status_created() {
        assert!(StatusCode::CREATED.is_success());
        assert_eq!(StatusCode::CREATED.code(), 201);
    }

    #[test]
    fn status_redirection_codes() {
        assert!(StatusCode::FOUND.is_redirection());
        assert!(StatusCode::SEE_OTHER.is_redirection());
        assert!(StatusCode::TEMPORARY_REDIRECT.is_redirection());
        assert!(StatusCode::PERMANENT_REDIRECT.is_redirection());
    }

    #[test]
    fn status_client_error_codes() {
        assert!(StatusCode::UNAUTHORIZED.is_client_error());
        assert!(StatusCode::FORBIDDEN.is_client_error());
        assert!(StatusCode::METHOD_NOT_ALLOWED.is_client_error());
        assert!(StatusCode::CONFLICT.is_client_error());
        assert!(StatusCode::GONE.is_client_error());
        assert!(StatusCode::TOO_MANY_REQUESTS.is_client_error());
    }

    #[test]
    fn status_server_error_codes() {
        assert!(StatusCode::NOT_IMPLEMENTED.is_server_error());
        assert!(StatusCode::BAD_GATEWAY.is_server_error());
        assert!(StatusCode::SERVICE_UNAVAILABLE.is_server_error());
        assert!(StatusCode::GATEWAY_TIMEOUT.is_server_error());
    }
}
