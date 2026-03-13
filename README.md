**English** | [日本語](README_JP.md)

# ALICE-HTTP

Pure Rust HTTP/1.1 and HTTP/2 parser and framework for the ALICE ecosystem. Provides request/response parsing, headers, status codes, chunked transfer encoding, content negotiation, cookie handling, and MIME types with zero external dependencies.

## Overview

| Item | Value |
|------|-------|
| **Crate** | `alice-http` |
| **Version** | 1.0.0 |
| **License** | AGPL-3.0 |
| **Edition** | 2021 |

## Features

- **HTTP Methods** — GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS, TRACE, CONNECT
- **Protocol Versions** — HTTP/1.0, HTTP/1.1, HTTP/2
- **Request/Response Parsing** — Parse raw bytes into structured request and response types
- **Header Management** — Case-insensitive header map with standard header constants
- **Status Codes** — Full status code enum with reason phrases (1xx-5xx)
- **Chunked Transfer Encoding** — Encode and decode chunked message bodies
- **Content Negotiation** — Accept, Accept-Encoding, Accept-Language parsing
- **Cookie Handling** — Parse and serialize Set-Cookie / Cookie headers
- **MIME Types** — Common MIME type constants and extension-based lookup

## Architecture

```
alice-http (lib.rs — single-file crate)
├── Method                       # HTTP methods
├── Version                      # HTTP/1.0, 1.1, 2
├── StatusCode                   # Response status codes
├── Headers                      # Case-insensitive header map
├── Request / Response           # Parsed HTTP messages
├── ChunkedEncoder / Decoder     # Transfer encoding
├── ContentNegotiation           # Accept header parsing
├── Cookie                       # Cookie handling
└── MimeType                     # MIME type registry
```

## Quick Start

```rust
use alice_http::{Request, Method, Version};

let req = Request::new(Method::Get, "/api/health", Version::Http11);
assert_eq!(req.method.as_str(), "GET");
```

## Build

```bash
cargo build
cargo test
cargo clippy -- -W clippy::all
```

## License

AGPL-3.0 -- see [LICENSE](LICENSE) for details.
