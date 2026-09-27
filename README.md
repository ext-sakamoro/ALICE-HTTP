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

`AGPL-3.0-or-later OR LicenseRef-Commercial` — dual-licensed. Pick either.

| Option | Terms | Use it when |
|--------|-------|-------------|
| **AGPL-3.0-or-later** | [LICENSE-AGPL](LICENSE-AGPL) — free, no reporting obligation | Your project is itself AGPL-compatible open source, or you are only using it internally |
| **Commercial License** | [LICENSE-COMMERCIAL.md](LICENSE-COMMERCIAL.md) — paid, removes the copyleft | Closed-source product, proprietary SaaS, edge / firmware distribution, plugin redistribution, or a platform NDA that forbids source disclosure |

AGPL is a strong copyleft: a product, firmware image, or service that links
`alice-http` and is distributed or served to users must be released under the AGPL
as well. That is intentional for the open ecosystem, and the Commercial
License exists for the cases where it is not something you are able to do.

Commercial licence enquiries: <contact@extoria.co.jp>
