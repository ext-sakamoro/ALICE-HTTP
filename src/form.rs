//! Form data (`application/x-www-form-urlencoded`) parsing.

use crate::error::HttpError;
use crate::url_encoding::UrlEncoding;
use std::collections::HashMap;

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
