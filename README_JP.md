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

`AGPL-3.0-or-later OR LicenseRef-Commercial` — デュアルライセンス どちらかを選べる

| 選択肢 | 条文 | こういう時 |
|--------|------|-----------|
| **AGPL-3.0-or-later** | [LICENSE-AGPL](LICENSE-AGPL) — 無償、報告義務なし | 自分の project も AGPL 互換の OSS、または社内利用のみ |
| **商用ライセンス** | [LICENSE-COMMERCIAL.md](LICENSE-COMMERCIAL.md) — 有償、コピーレフト義務を解除 | クローズドソース製品 / 商用 SaaS / エッジ・ファームウェア配布 / plugin 再配布 / ソース開示を禁じるプラットフォーム NDA |

AGPL は強いコピーレフト: `alice-http` を link して配布 / 提供する製品・ファームウェア・
サービスは AGPL で公開する義務がある これはオープンなエコシステムのための意図的な
選択で、それが実行できない場合のために商用ライセンスを用意している

商用ライセンスの問い合わせ: <contact@extoria.co.jp>
