//! `MediaType` — MIME / media type parser.

use crate::error::HttpError;
use std::collections::HashMap;
use std::fmt;

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
