//! HTTP/2 frame types (`H2FrameType` / `H2Frame`, simplified binary parser).

use crate::error::HttpError;

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
