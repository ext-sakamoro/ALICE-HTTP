//! Chunked transfer encoding.

use crate::error::HttpError;

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
