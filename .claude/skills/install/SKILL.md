---
argument-hint: ""
---

# /install skill

`renga` をリリースビルドして `/usr/local/bin` と `~/.cargo/bin` にインストールする。

## Steps

1. `cargo build --release` を実行する
2. `target/release/renga` を `/usr/local/bin/renga` にコピーする
3. `target/release/renga` を `~/.cargo/bin/renga` にコピーする
4. `renga --version` で確認する

## Rules

- ビルドが失敗したらインストールせずエラーを報告する
- インストール後に `renga --version` の出力をユーザーに示す
