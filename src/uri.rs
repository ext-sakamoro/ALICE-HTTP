//! `Uri` — request URI parser.

use crate::error::HttpError;
use std::collections::HashMap;
use std::fmt;

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
