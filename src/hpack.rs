//! HPACK integer encoding (HTTP/2 header compression primitive).

use crate::error::HttpError;

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
