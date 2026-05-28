---
argument-hint: ""
---

# /install skill

`renga` をインストールする。インストール方法を選択できる。

## インストール方法の選択

以下の2つの方法があります。どちらを使うかをユーザーに確認してから実行する。

**A. crates.io からインストール（通常推奨）**
- 公開済みの安定版をインストールする場合に使う
- コマンド: `cargo install renga`

**B. ローカルビルドからインストール（開発中の動作確認用）**
- 現在のソースコードをビルドしてインストールする場合に使う
- `/usr/local/bin` と `~/.cargo/bin` の両方にコピーする

## Steps（A: crates.io からインストール）

1. ユーザーに「crates.io からインストール」を確認する
2. `cargo install renga` を実行する
3. `renga --version` で確認する

## Steps（B: ローカルビルドからインストール）

1. ユーザーに「ローカルビルドからインストール」を確認する
2. `cargo build --release` を実行する
3. `target/release/renga` を `/usr/local/bin/renga` にコピーする
4. `target/release/renga` を `~/.cargo/bin/renga` にコピーする
5. `renga --version` で確認する

## Rules

- どちらの方法を使うかをユーザーに確認してから実行する
- ビルド・インストールが失敗したらエラーを報告する
- インストール後に `renga --version` の出力をユーザーに示す
