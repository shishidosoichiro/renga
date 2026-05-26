---
argument-hint: ""
---

# /install skill

`fbim` をリリースビルドして `/usr/local/bin` と `~/.cargo/bin` にインストールする。

## Steps

1. `cargo build --release` を実行する
2. `target/release/fbim` を `/usr/local/bin/fbim` にコピーする
3. `target/release/fbim` を `~/.cargo/bin/fbim` にコピーする
4. `fbim --version` で確認する

## Rules

- ビルドが失敗したらインストールせずエラーを報告する
- インストール後に `fbim --version` の出力をユーザーに示す
