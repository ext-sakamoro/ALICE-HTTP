//! Integration tests spanning multiple modules.

#![allow(
    clippy::float_cmp,
    clippy::unreadable_literal,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::cast_possible_wrap,
    clippy::too_many_lines,
    clippy::needless_range_loop,
    clippy::explicit_iter_loop,
    clippy::bool_to_int_with_if,
    clippy::approx_constant,
    clippy::cast_lossless,
    clippy::redundant_clone,
    clippy::format_collect,
    clippy::needless_collect,
    clippy::similar_names
)]

use crate::chunked::*;
use crate::content_neg::*;
use crate::cookie::*;
use crate::error::*;
use crate::form::*;
use crate::h2_frame::*;
use crate::headers::*;
use crate::hpack::*;
use crate::method::*;
use crate::mime::*;
use crate::request::*;
use crate::response::*;
use crate::status::*;
use crate::uri::*;
use crate::url_encoding::*;
use crate::version::*;

// --- Method ---

#[test]
fn method_from_str_valid() {
    assert_eq!("GET".parse::<Method>().unwrap(), Method::Get);
    assert_eq!("POST".parse::<Method>().unwrap(), Method::Post);
    assert_eq!("PUT".parse::<Method>().unwrap(), Method::Put);
    assert_eq!("DELETE".parse::<Method>().unwrap(), Method::Delete);
    assert_eq!("PATCH".parse::<Method>().unwrap(), Method::Patch);
    assert_eq!("HEAD".parse::<Method>().unwrap(), Method::Head);
    assert_eq!("OPTIONS".parse::<Method>().unwrap(), Method::Options);
    assert_eq!("TRACE".parse::<Method>().unwrap(), Method::Trace);
    assert_eq!("CONNECT".parse::<Method>().unwrap(), Method::Connect);
}

#[test]
fn method_from_str_invalid() {
    assert_eq!("INVALID".parse::<Method>(), Err(HttpError::InvalidMethod));
}

#[test]
fn method_display() {
    assert_eq!(Method::Get.to_string(), "GET");
    assert_eq!(Method::Post.to_string(), "POST");
}

#[test]
fn method_as_str() {
    assert_eq!(Method::Delete.as_str(), "DELETE");
    assert_eq!(Method::Options.as_str(), "OPTIONS");
}

// --- Version ---

#[test]
fn version_parse() {
    assert_eq!("HTTP/1.0".parse::<Version>().unwrap(), Version::Http10);
    assert_eq!("HTTP/1.1".parse::<Version>().unwrap(), Version::Http11);
    assert_eq!("HTTP/2".parse::<Version>().unwrap(), Version::Http2);
    assert_eq!("HTTP/2.0".parse::<Version>().unwrap(), Version::Http2);
}

#[test]
fn version_invalid() {
    assert_eq!("HTTP/3".parse::<Version>(), Err(HttpError::InvalidVersion));
}

#[test]
fn version_display() {
    assert_eq!(Version::Http11.to_string(), "HTTP/1.1");
    assert_eq!(Version::Http2.to_string(), "HTTP/2");
}

// --- Status Code ---

#[test]
fn status_code_constants() {
    assert_eq!(StatusCode::OK.code(), 200);
    assert_eq!(StatusCode::NOT_FOUND.code(), 404);
    assert_eq!(StatusCode::INTERNAL_SERVER_ERROR.code(), 500);
}

#[test]
fn status_code_reason() {
    assert_eq!(StatusCode::OK.reason(), "OK");
    assert_eq!(StatusCode::NOT_FOUND.reason(), "Not Found");
    assert_eq!(StatusCode::CONTINUE.reason(), "Continue");
}

#[test]
fn status_code_display() {
    assert_eq!(StatusCode::OK.to_string(), "200 OK");
    assert_eq!(StatusCode::NOT_FOUND.to_string(), "404 Not Found");
}

#[test]
fn status_code_categories() {
    assert!(StatusCode::CONTINUE.is_informational());
    assert!(!StatusCode::CONTINUE.is_success());
    assert!(StatusCode::OK.is_success());
    assert!(StatusCode::MOVED_PERMANENTLY.is_redirection());
    assert!(StatusCode::BAD_REQUEST.is_client_error());
    assert!(StatusCode::INTERNAL_SERVER_ERROR.is_server_error());
}

#[test]
fn status_code_from_u16_valid() {
    assert_eq!(StatusCode::from_u16(200).unwrap().code(), 200);
    assert_eq!(StatusCode::from_u16(999).unwrap().code(), 999);
}

#[test]
fn status_code_from_u16_invalid() {
    assert!(StatusCode::from_u16(0).is_err());
    assert!(StatusCode::from_u16(99).is_err());
    assert!(StatusCode::from_u16(1000).is_err());
}

#[test]
fn status_code_unknown_reason() {
    assert_eq!(StatusCode::from_u16(999).unwrap().reason(), "Unknown");
}

// --- Headers ---

#[test]
fn headers_set_and_get() {
    let mut h = Headers::new();
    h.set("Content-Type", "text/html");
    assert_eq!(h.get("content-type"), Some("text/html"));
    assert_eq!(h.get("CONTENT-TYPE"), Some("text/html"));
}

#[test]
fn headers_overwrite() {
    let mut h = Headers::new();
    h.set("Host", "a.com");
    h.set("Host", "b.com");
    assert_eq!(h.get("host"), Some("b.com"));
    assert_eq!(h.len(), 1);
}

#[test]
fn headers_append() {
    let mut h = Headers::new();
    h.append("Set-Cookie", "a=1");
    h.append("Set-Cookie", "b=2");
    let all = h.get_all("set-cookie");
    assert_eq!(all.len(), 2);
}

#[test]
fn headers_remove() {
    let mut h = Headers::new();
    h.set("Host", "example.com");
    h.remove("HOST");
    assert!(h.get("host").is_none());
    assert!(h.is_empty());
}

#[test]
fn headers_contains() {
    let mut h = Headers::new();
    h.set("Accept", "*/*");
    assert!(h.contains("accept"));
    assert!(!h.contains("host"));
}

#[test]
fn headers_iter() {
    let mut h = Headers::new();
    h.set("A", "1");
    h.set("B", "2");
    let pairs: Vec<_> = h.iter().collect();
    assert_eq!(pairs.len(), 2);
}

#[test]
fn headers_to_http1() {
    let mut h = Headers::new();
    h.set("Host", "example.com");
    let s = h.to_http1();
    assert!(s.contains("host: example.com\r\n"));
}

// --- URI ---

#[test]
fn uri_parse_simple() {
    let uri = Uri::parse("/path").unwrap();
    assert_eq!(uri.path(), "/path");
    assert!(uri.query().is_none());
    assert!(uri.fragment().is_none());
}

#[test]
fn uri_parse_with_query() {
    let uri = Uri::parse("/search?q=hello&lang=en").unwrap();
    assert_eq!(uri.path(), "/search");
    assert_eq!(uri.query(), Some("q=hello&lang=en"));
}

#[test]
fn uri_parse_with_fragment() {
    let uri = Uri::parse("/page#section").unwrap();
    assert_eq!(uri.path(), "/page");
    assert_eq!(uri.fragment(), Some("section"));
}

#[test]
fn uri_parse_full() {
    let uri = Uri::parse("/a?b=c#d").unwrap();
    assert_eq!(uri.path(), "/a");
    assert_eq!(uri.query(), Some("b=c"));
    assert_eq!(uri.fragment(), Some("d"));
}

#[test]
fn uri_parse_empty() {
    assert!(Uri::parse("").is_err());
}

#[test]
fn uri_query_params() {
    let uri = Uri::parse("/s?a=1&b=2&c").unwrap();
    let params = uri.query_params();
    assert_eq!(params.get("a"), Some(&"1".to_string()));
    assert_eq!(params.get("b"), Some(&"2".to_string()));
    assert_eq!(params.get("c"), Some(&String::new()));
}

#[test]
fn uri_display() {
    let uri = Uri::parse("/test?x=1").unwrap();
    assert_eq!(uri.to_string(), "/test?x=1");
}

#[test]
fn uri_raw() {
    let uri = Uri::parse("/hello?world#foo").unwrap();
    assert_eq!(uri.raw(), "/hello?world#foo");
}

// --- Request ---

#[test]
fn request_parse_get() {
    let raw = b"GET /index.html HTTP/1.1\r\nHost: example.com\r\n\r\n";
    let req = Request::parse(raw).unwrap();
    assert_eq!(req.method, Method::Get);
    assert_eq!(req.uri.path(), "/index.html");
    assert_eq!(req.version, Version::Http11);
    assert_eq!(req.headers.get("host"), Some("example.com"));
}

#[test]
fn request_parse_post_with_body() {
    let raw = b"POST /api HTTP/1.1\r\nContent-Length: 5\r\n\r\nhello";
    let req = Request::parse(raw).unwrap();
    assert_eq!(req.method, Method::Post);
    assert_eq!(req.body, b"hello");
    assert_eq!(req.content_length(), Some(5));
}

#[test]
fn request_builder() {
    let req = Request::builder()
        .method(Method::Put)
        .uri("/resource")
        .header("Content-Type", "application/json")
        .body(b"{}")
        .build()
        .unwrap();
    assert_eq!(req.method, Method::Put);
    assert_eq!(req.uri.path(), "/resource");
    assert_eq!(req.content_type(), Some("application/json"));
}

#[test]
fn request_roundtrip() {
    let req = Request::builder()
        .method(Method::Get)
        .uri("/test")
        .header("Host", "localhost")
        .build()
        .unwrap();
    let bytes = req.to_bytes();
    let parsed = Request::parse(&bytes).unwrap();
    assert_eq!(parsed.method, Method::Get);
    assert_eq!(parsed.uri.path(), "/test");
}

#[test]
fn request_parse_incomplete() {
    let raw = b"GET /test HTTP/1.1\r\nHost: x";
    assert!(Request::parse(raw).is_err());
}

#[test]
fn request_parse_invalid_method() {
    let raw = b"FOOBAR /test HTTP/1.1\r\n\r\n";
    assert!(Request::parse(raw).is_err());
}

// --- Response ---

#[test]
fn response_parse_ok() {
    let raw = b"HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<h1>Hi</h1>";
    let resp = Response::parse(raw).unwrap();
    assert_eq!(resp.status, StatusCode::OK);
    assert_eq!(resp.content_type(), Some("text/html"));
    assert_eq!(resp.body, b"<h1>Hi</h1>");
}

#[test]
fn response_parse_404() {
    let raw = b"HTTP/1.1 404 Not Found\r\n\r\n";
    let resp = Response::parse(raw).unwrap();
    assert_eq!(resp.status, StatusCode::NOT_FOUND);
}

#[test]
fn response_builder() {
    let resp = Response::builder()
        .status(StatusCode::CREATED)
        .header("Location", "/new")
        .body(b"created")
        .build();
    assert_eq!(resp.status, StatusCode::CREATED);
    assert_eq!(resp.headers.get("location"), Some("/new"));
}

#[test]
fn response_roundtrip() {
    let resp = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/plain")
        .body(b"hello")
        .build();
    let bytes = resp.to_bytes();
    let parsed = Response::parse(&bytes).unwrap();
    assert_eq!(parsed.status, StatusCode::OK);
    assert_eq!(parsed.body, b"hello");
}

#[test]
fn response_content_length() {
    let resp = Response::builder().header("Content-Length", "42").build();
    assert_eq!(resp.content_length(), Some(42));
}

#[test]
fn response_version() {
    let resp = Response::builder().version(Version::Http10).build();
    assert_eq!(resp.version, Version::Http10);
}

// --- Chunked Encoding ---

#[test]
fn chunked_encode_decode() {
    let data = b"Hello, World!";
    let encoded = ChunkedEncoding::encode(data, 5);
    let decoded = ChunkedEncoding::decode(&encoded).unwrap();
    assert_eq!(decoded, data);
}

#[test]
fn chunked_single_chunk() {
    let data = b"abc";
    let encoded = ChunkedEncoding::encode(data, 100);
    let decoded = ChunkedEncoding::decode(&encoded).unwrap();
    assert_eq!(decoded, data);
}

#[test]
fn chunked_decode_with_extension() {
    let encoded = b"5;ext=val\r\nhello\r\n0\r\n\r\n";
    let decoded = ChunkedEncoding::decode(encoded).unwrap();
    assert_eq!(decoded, b"hello");
}

#[test]
fn chunked_decode_invalid() {
    assert!(ChunkedEncoding::decode(b"xyz\r\n").is_err());
}

#[test]
fn chunked_empty() {
    let encoded = b"0\r\n\r\n";
    let decoded = ChunkedEncoding::decode(encoded).unwrap();
    assert!(decoded.is_empty());
}

// --- Media Type ---

#[test]
fn media_type_parse_simple() {
    let mt = MediaType::parse("text/html").unwrap();
    assert_eq!(mt.main_type, "text");
    assert_eq!(mt.sub_type, "html");
    assert_eq!(mt.essence(), "text/html");
}

#[test]
fn media_type_parse_with_charset() {
    let mt = MediaType::parse("text/html; charset=utf-8").unwrap();
    assert_eq!(mt.charset(), Some("utf-8"));
}

#[test]
fn media_type_parse_with_quoted_param() {
    let mt = MediaType::parse("text/html; charset=\"utf-8\"").unwrap();
    assert_eq!(mt.charset(), Some("utf-8"));
}

#[test]
fn media_type_display() {
    let mt = MediaType::parse("application/json").unwrap();
    assert_eq!(mt.to_string(), "application/json");
}

#[test]
fn media_type_invalid() {
    assert!(MediaType::parse("invalid").is_err());
    assert!(MediaType::parse("/html").is_err());
    assert!(MediaType::parse("text/").is_err());
}

#[test]
fn media_type_from_extension() {
    assert_eq!(MediaType::from_extension("html"), "text/html");
    assert_eq!(MediaType::from_extension("json"), "application/json");
    assert_eq!(MediaType::from_extension("png"), "image/png");
    assert_eq!(MediaType::from_extension("jpg"), "image/jpeg");
    assert_eq!(MediaType::from_extension("css"), "text/css");
    assert_eq!(MediaType::from_extension("js"), "text/javascript");
    assert_eq!(
        MediaType::from_extension("unknown"),
        "application/octet-stream"
    );
}

#[test]
fn media_type_from_extension_case_insensitive() {
    assert_eq!(MediaType::from_extension("HTML"), "text/html");
    assert_eq!(MediaType::from_extension("JSON"), "application/json");
}

#[test]
fn media_type_constants() {
    assert_eq!(MediaType::TEXT_PLAIN, "text/plain");
    assert_eq!(MediaType::APPLICATION_JSON, "application/json");
    assert_eq!(MediaType::IMAGE_WEBP, "image/webp");
}

// --- Cookie ---

#[test]
fn cookie_new() {
    let c = Cookie::new("session", "abc123");
    assert_eq!(c.name, "session");
    assert_eq!(c.value, "abc123");
    assert!(!c.secure);
    assert!(!c.http_only);
}

#[test]
fn cookie_parse_set_cookie_simple() {
    let c = Cookie::parse_set_cookie("session=abc; Path=/; Secure; HttpOnly").unwrap();
    assert_eq!(c.name, "session");
    assert_eq!(c.value, "abc");
    assert_eq!(c.path, Some("/".to_string()));
    assert!(c.secure);
    assert!(c.http_only);
}

#[test]
fn cookie_parse_set_cookie_full() {
    let c = Cookie::parse_set_cookie(
        "id=42; Path=/api; Domain=example.com; Max-Age=3600; SameSite=Strict; Secure",
    )
    .unwrap();
    assert_eq!(c.name, "id");
    assert_eq!(c.value, "42");
    assert_eq!(c.domain, Some("example.com".to_string()));
    assert_eq!(c.max_age, Some(3600));
    assert_eq!(c.same_site, Some(SameSite::Strict));
    assert!(c.secure);
}

#[test]
fn cookie_parse_set_cookie_samesite_lax() {
    let c = Cookie::parse_set_cookie("x=1; SameSite=Lax").unwrap();
    assert_eq!(c.same_site, Some(SameSite::Lax));
}

#[test]
fn cookie_parse_set_cookie_samesite_none() {
    let c = Cookie::parse_set_cookie("x=1; SameSite=None").unwrap();
    assert_eq!(c.same_site, Some(SameSite::None));
}

#[test]
fn cookie_parse_invalid() {
    assert!(Cookie::parse_set_cookie("").is_err());
    assert!(Cookie::parse_set_cookie("=value").is_err());
}

#[test]
fn cookie_to_set_cookie() {
    let mut c = Cookie::new("tok", "xyz");
    c.path = Some("/".to_string());
    c.secure = true;
    c.http_only = true;
    c.same_site = Some(SameSite::Lax);
    let s = c.to_set_cookie();
    assert!(s.contains("tok=xyz"));
    assert!(s.contains("Path=/"));
    assert!(s.contains("Secure"));
    assert!(s.contains("HttpOnly"));
    assert!(s.contains("SameSite=Lax"));
}

#[test]
fn cookie_parse_header() {
    let cookies = Cookie::parse_cookie_header("a=1; b=2; c=3");
    assert_eq!(cookies.len(), 3);
    assert_eq!(cookies[0], ("a".to_string(), "1".to_string()));
    assert_eq!(cookies[2], ("c".to_string(), "3".to_string()));
}

#[test]
fn cookie_max_age_display() {
    let mut c = Cookie::new("x", "1");
    c.max_age = Some(7200);
    let s = c.to_set_cookie();
    assert!(s.contains("Max-Age=7200"));
}

#[test]
fn cookie_domain_display() {
    let mut c = Cookie::new("x", "1");
    c.domain = Some("example.com".to_string());
    let s = c.to_set_cookie();
    assert!(s.contains("Domain=example.com"));
}

// --- Content Negotiation ---

#[test]
fn accept_parse_simple() {
    let entries = ContentNegotiation::parse_accept("text/html, application/json");
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].media_type, "text/html");
}

#[test]
fn accept_parse_with_quality() {
    let entries = ContentNegotiation::parse_accept("text/html;q=0.9, application/json;q=1.0");
    assert_eq!(entries[0].media_type, "application/json");
    assert!((entries[0].quality - 1.0).abs() < f32::EPSILON);
}

#[test]
fn negotiate_exact_match() {
    let accept = ContentNegotiation::parse_accept("application/json, text/html");
    let result = ContentNegotiation::negotiate(&accept, &["text/html", "application/json"]);
    assert_eq!(result, Some("application/json"));
}

#[test]
fn negotiate_wildcard() {
    let accept = ContentNegotiation::parse_accept("*/*");
    let result = ContentNegotiation::negotiate(&accept, &["text/plain"]);
    assert_eq!(result, Some("text/plain"));
}

#[test]
fn negotiate_subtype_wildcard() {
    let accept = ContentNegotiation::parse_accept("text/*");
    let result = ContentNegotiation::negotiate(&accept, &["text/html", "application/json"]);
    assert_eq!(result, Some("text/html"));
}

#[test]
fn negotiate_no_match() {
    let accept = ContentNegotiation::parse_accept("image/png");
    let result = ContentNegotiation::negotiate(&accept, &["text/html"]);
    assert!(result.is_none());
}

#[test]
fn accept_encoding_parse() {
    let entries = ContentNegotiation::parse_accept_encoding("gzip, deflate;q=0.5, br;q=0.8");
    assert_eq!(entries[0].media_type, "gzip");
    assert_eq!(entries[1].media_type, "br");
    assert_eq!(entries[2].media_type, "deflate");
}

#[test]
fn accept_language_parse() {
    let entries = ContentNegotiation::parse_accept_language("en-US, ja;q=0.9, fr;q=0.5");
    assert_eq!(entries[0].media_type, "en-US");
    assert_eq!(entries[1].media_type, "ja");
}

// --- HTTP/2 Frames ---

#[test]
fn h2_frame_type_roundtrip() {
    for i in 0..=9 {
        let ft = H2FrameType::from_u8(i);
        assert_eq!(ft.to_u8(), i);
    }
}

#[test]
fn h2_frame_type_unknown() {
    let ft = H2FrameType::from_u8(255);
    assert_eq!(ft, H2FrameType::Unknown(255));
    assert_eq!(ft.to_u8(), 255);
}

#[test]
fn h2_frame_parse_settings() {
    let frame = H2Frame::settings(0, &[]);
    let bytes = frame.to_bytes();
    let parsed = H2Frame::parse(&bytes).unwrap();
    assert_eq!(parsed.frame_type, H2FrameType::Settings);
    assert_eq!(parsed.stream_id, 0);
    assert!(parsed.payload.is_empty());
}

#[test]
fn h2_frame_parse_with_payload() {
    let payload = b"hello";
    let mut frame = H2Frame::settings(1, payload);
    frame.frame_type = H2FrameType::Data;
    let bytes = frame.to_bytes();
    let parsed = H2Frame::parse(&bytes).unwrap();
    assert_eq!(parsed.frame_type, H2FrameType::Data);
    assert_eq!(parsed.payload, payload);
}

#[test]
fn h2_frame_parse_incomplete() {
    assert!(H2Frame::parse(&[0; 5]).is_err());
}

#[test]
fn h2_frame_ping() {
    let frame = H2Frame::ping(&[1, 2, 3, 4, 5, 6, 7, 8]);
    let bytes = frame.to_bytes();
    let parsed = H2Frame::parse(&bytes).unwrap();
    assert_eq!(parsed.frame_type, H2FrameType::Ping);
    assert_eq!(parsed.payload, [1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn h2_frame_window_update() {
    let frame = H2Frame::window_update(1, 65535);
    let bytes = frame.to_bytes();
    let parsed = H2Frame::parse(&bytes).unwrap();
    assert_eq!(parsed.frame_type, H2FrameType::WindowUpdate);
    assert_eq!(parsed.stream_id, 1);
}

#[test]
fn h2_frame_goaway() {
    let frame = H2Frame::goaway(0, 0);
    let bytes = frame.to_bytes();
    let parsed = H2Frame::parse(&bytes).unwrap();
    assert_eq!(parsed.frame_type, H2FrameType::GoAway);
    assert_eq!(parsed.length, 8);
}

#[test]
fn h2_frame_flags() {
    let mut frame = H2Frame::settings(0, &[]);
    frame.flags = 0x01; // END_STREAM / ACK
    assert!(frame.is_end_stream());
    assert!(frame.is_ack());

    frame.flags = 0x04; // END_HEADERS
    assert!(frame.is_end_headers());
    assert!(!frame.is_end_stream());
}

#[test]
fn h2_connection_preface() {
    assert!(H2Frame::CONNECTION_PREFACE.starts_with(b"PRI"));
}

// --- HPACK Integer ---

#[test]
fn hpack_int_small() {
    let encoded = HpackInt::encode(10, 5);
    let (val, consumed) = HpackInt::decode(&encoded, 5).unwrap();
    assert_eq!(val, 10);
    assert_eq!(consumed, 1);
}

#[test]
fn hpack_int_large() {
    let encoded = HpackInt::encode(1337, 5);
    let (val, consumed) = HpackInt::decode(&encoded, 5).unwrap();
    assert_eq!(val, 1337);
    assert!(consumed > 1);
}

#[test]
fn hpack_int_boundary() {
    let encoded = HpackInt::encode(31, 5);
    let (val, _) = HpackInt::decode(&encoded, 5).unwrap();
    assert_eq!(val, 31);
}

#[test]
fn hpack_int_zero() {
    let encoded = HpackInt::encode(0, 5);
    let (val, consumed) = HpackInt::decode(&encoded, 5).unwrap();
    assert_eq!(val, 0);
    assert_eq!(consumed, 1);
}

#[test]
fn hpack_int_decode_empty() {
    assert!(HpackInt::decode(&[], 5).is_err());
}

// --- URL Encoding ---

#[test]
fn url_encode_simple() {
    assert_eq!(UrlEncoding::encode("hello world"), "hello%20world");
}

#[test]
fn url_encode_special() {
    assert_eq!(UrlEncoding::encode("a&b=c"), "a%26b%3Dc");
}

#[test]
fn url_encode_unreserved() {
    assert_eq!(UrlEncoding::encode("abc-._~123"), "abc-._~123");
}

#[test]
fn url_decode_simple() {
    assert_eq!(UrlEncoding::decode("hello%20world").unwrap(), "hello world");
}

#[test]
fn url_decode_plus() {
    assert_eq!(UrlEncoding::decode("hello+world").unwrap(), "hello world");
}

#[test]
fn url_decode_invalid() {
    assert!(UrlEncoding::decode("hello%2").is_err());
    assert!(UrlEncoding::decode("hello%GG").is_err());
}

#[test]
fn url_roundtrip() {
    let input = "Hello, World! /path?q=1&a=2";
    let encoded = UrlEncoding::encode(input);
    let decoded = UrlEncoding::decode(&encoded).unwrap();
    assert_eq!(decoded, input);
}

// --- Form Data ---

#[test]
fn form_data_parse() {
    let data = FormData::parse("name=Alice&age=30").unwrap();
    assert_eq!(data.get("name"), Some(&"Alice".to_string()));
    assert_eq!(data.get("age"), Some(&"30".to_string()));
}

#[test]
fn form_data_parse_encoded() {
    let data = FormData::parse("q=hello+world&lang=en").unwrap();
    assert_eq!(data.get("q"), Some(&"hello world".to_string()));
}

#[test]
fn form_data_encode() {
    let encoded = FormData::encode(&[("name", "Alice Bob"), ("age", "30")]);
    assert!(encoded.contains("name=Alice%20Bob"));
    assert!(encoded.contains("age=30"));
}

#[test]
fn form_data_empty() {
    let data = FormData::parse("").unwrap();
    assert!(data.is_empty());
}

// --- Error Display ---

#[test]
fn error_display() {
    assert_eq!(HttpError::InvalidMethod.to_string(), "invalid HTTP method");
    assert_eq!(
        HttpError::InvalidChunk.to_string(),
        "invalid chunked encoding"
    );
    assert_eq!(HttpError::TooLarge.to_string(), "payload too large");
}

#[test]
fn error_is_std_error() {
    let err: &dyn std::error::Error = &HttpError::InvalidRequest;
    assert!(!err.to_string().is_empty());
}

// --- SameSite Display ---

#[test]
fn samesite_display() {
    assert_eq!(SameSite::Strict.to_string(), "Strict");
    assert_eq!(SameSite::Lax.to_string(), "Lax");
    assert_eq!(SameSite::None.to_string(), "None");
}

// --- Additional edge cases ---

#[test]
fn request_with_query_params() {
    let raw = b"GET /search?q=test&page=2 HTTP/1.1\r\nHost: example.com\r\n\r\n";
    let req = Request::parse(raw).unwrap();
    let params = req.uri.query_params();
    assert_eq!(params.get("q"), Some(&"test".to_string()));
    assert_eq!(params.get("page"), Some(&"2".to_string()));
}

#[test]
fn response_parse_with_content_length() {
    let raw = b"HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nhello";
    let resp = Response::parse(raw).unwrap();
    assert_eq!(resp.content_length(), Some(5));
    assert_eq!(resp.body, b"hello");
}

#[test]
fn chunked_multi_chunk() {
    let data = b"abcdefghij";
    let encoded = ChunkedEncoding::encode(data, 3);
    let decoded = ChunkedEncoding::decode(&encoded).unwrap();
    assert_eq!(decoded, data);
}

#[test]
fn media_type_with_multiple_params() {
    let mt = MediaType::parse("text/html; charset=utf-8; boundary=something").unwrap();
    assert_eq!(mt.charset(), Some("utf-8"));
    assert_eq!(mt.params.get("boundary"), Some(&"something".to_string()));
}

#[test]
fn headers_default() {
    let h = Headers::default();
    assert!(h.is_empty());
    assert_eq!(h.len(), 0);
}

#[test]
fn version_as_str() {
    assert_eq!(Version::Http10.as_str(), "HTTP/1.0");
    assert_eq!(Version::Http11.as_str(), "HTTP/1.1");
    assert_eq!(Version::Http2.as_str(), "HTTP/2");
}

#[test]
fn status_switching_protocols() {
    assert!(StatusCode::SWITCHING_PROTOCOLS.is_informational());
    assert_eq!(
        StatusCode::SWITCHING_PROTOCOLS.reason(),
        "Switching Protocols"
    );
}

#[test]
fn status_created() {
    assert!(StatusCode::CREATED.is_success());
    assert_eq!(StatusCode::CREATED.code(), 201);
}

#[test]
fn status_redirection_codes() {
    assert!(StatusCode::FOUND.is_redirection());
    assert!(StatusCode::SEE_OTHER.is_redirection());
    assert!(StatusCode::TEMPORARY_REDIRECT.is_redirection());
    assert!(StatusCode::PERMANENT_REDIRECT.is_redirection());
}

#[test]
fn status_client_error_codes() {
    assert!(StatusCode::UNAUTHORIZED.is_client_error());
    assert!(StatusCode::FORBIDDEN.is_client_error());
    assert!(StatusCode::METHOD_NOT_ALLOWED.is_client_error());
    assert!(StatusCode::CONFLICT.is_client_error());
    assert!(StatusCode::GONE.is_client_error());
    assert!(StatusCode::TOO_MANY_REQUESTS.is_client_error());
}

#[test]
fn status_server_error_codes() {
    assert!(StatusCode::NOT_IMPLEMENTED.is_server_error());
    assert!(StatusCode::BAD_GATEWAY.is_server_error());
    assert!(StatusCode::SERVICE_UNAVAILABLE.is_server_error());
    assert!(StatusCode::GATEWAY_TIMEOUT.is_server_error());
}
