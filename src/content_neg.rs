//! Content negotiation (`ContentNegotiation` / `AcceptEntry`).

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
