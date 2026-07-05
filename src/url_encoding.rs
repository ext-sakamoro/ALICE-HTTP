//! URL encoding / decoding.

use crate::error::HttpError;

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
