[English](README.md) | **日本語**

# ALICE-HTTP

ALICEエコシステムの純Rust HTTP/1.1 & HTTP/2パーサー＆フレームワーク。リクエスト/レスポンス解析、ヘッダー、ステータスコード、チャンク転送エンコーディング、コンテントネゴシエーション、Cookie処理、MIMEタイプを外部依存なしで提供。

## 概要

| 項目 | 値 |
|------|-----|
| **クレート名** | `alice-http` |
| **バージョン** | 1.0.0 |
| **ライセンス** | AGPL-3.0 |
| **エディション** | 2021 |

## 機能

- **HTTPメソッド** — GET / POST / PUT / DELETE / PATCH / HEAD / OPTIONS / TRACE / CONNECT
- **プロトコルバージョン** — HTTP/1.0、HTTP/1.1、HTTP/2
- **リクエスト/レスポンス解析** — 生バイトから構造化されたリクエスト・レスポンス型への変換
- **ヘッダー管理** — 大文字小文字非依存のヘッダーマップと標準ヘッダー定数
- **ステータスコード** — 理由フレーズ付き完全ステータスコード列挙型 (1xx-5xx)
- **チャンク転送エンコーディング** — チャンクメッセージボディのエンコード/デコード
- **コンテントネゴシエーション** — Accept / Accept-Encoding / Accept-Language 解析
- **Cookie処理** — Set-Cookie / Cookie ヘッダーの解析とシリアライズ
- **MIMEタイプ** — 一般的なMIMEタイプ定数と拡張子ベースのルックアップ

## アーキテクチャ

```
alice-http (lib.rs — 単一ファイルクレート)
├── Method                       # HTTPメソッド
├── Version                      # HTTP/1.0, 1.1, 2
├── StatusCode                   # レスポンスステータスコード
├── Headers                      # 大文字小文字非依存ヘッダーマップ
├── Request / Response           # 解析済みHTTPメッセージ
├── ChunkedEncoder / Decoder     # 転送エンコーディング
├── ContentNegotiation           # Acceptヘッダー解析
├── Cookie                       # Cookie処理
└── MimeType                     # MIMEタイプレジストリ
```

## クイックスタート

```rust
use alice_http::{Request, Method, Version};

let req = Request::new(Method::Get, "/api/health", Version::Http11);
assert_eq!(req.method.as_str(), "GET");
```

## ビルド

```bash
cargo build
cargo test
cargo clippy -- -W clippy::all
```

## ライセンス

AGPL-3.0 — 詳細は [LICENSE](LICENSE) を参照。
