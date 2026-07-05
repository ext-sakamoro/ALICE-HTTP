//! `Cookie` + `SameSite` (RFC 6265).

use crate::error::HttpError;
use std::fmt;

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
