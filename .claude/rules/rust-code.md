---
paths:
  - "src/**/*.rs"
  - "tests/**/*.rs"
---

# Rust コード規約

## エラーハンドリング

- `unwrap()` / `expect()` はテストコード以外で使わない
- ユーザー向けエラーメッセージは `error:` プレフィックスで stderr に出力し、exit code 1 で終了する
- ドメインエラーは `thiserror`、アプリエラーは `anyhow` で扱う

## ドキュメント

- `src/lib.rs` に `#![deny(missing_docs)]` を置く。公開アイテムへの doc コメント漏れをコンパイルエラーにする
- 公開アイテムには `///` で doc コメントを付け、`# Examples` セクションを書いて doctest にする
- 非公開アイテムは WHY が自明でない場合のみ `//` で書く

## テスト方針

- ファイルシステムを伴うテストは `tempfile::TempDir` を使う。モックは使わない
- 統合テストは `tests/` に置く
