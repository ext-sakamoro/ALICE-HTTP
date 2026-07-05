//! `Headers` — case-insensitive header map.

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
