//! ALICE-HTTP: Pure Rust HTTP/1.1 and HTTP/2 parser and framework.
//!
//! Provides request/response parsing, headers, methods, status codes,
//! chunked transfer encoding, content negotiation, cookie handling, and MIME types.

#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::module_name_repetitions,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::must_use_candidate,
    clippy::wildcard_imports,
    clippy::doc_markdown,
    clippy::too_many_lines,
    clippy::cast_possible_truncation,
    clippy::cast_lossless,
    clippy::similar_names
)]

pub mod chunked;
pub mod content_neg;
pub mod cookie;
pub mod error;
pub mod form;
pub mod h2_frame;
pub mod headers;
pub mod hpack;
pub mod method;
pub mod mime;
pub mod prelude;
pub mod request;
pub mod response;
pub mod status;
pub mod uri;
pub mod url_encoding;
pub mod version;

#[cfg(test)]
mod integration_tests;

// Backward-compat re-exports.
pub use crate::chunked::*;
pub use crate::content_neg::*;
pub use crate::cookie::*;
pub use crate::error::*;
pub use crate::form::*;
pub use crate::h2_frame::*;
pub use crate::headers::*;
pub use crate::hpack::*;
pub use crate::method::*;
pub use crate::mime::*;
pub use crate::request::*;
pub use crate::response::*;
pub use crate::status::*;
pub use crate::uri::*;
pub use crate::url_encoding::*;
pub use crate::version::*;
