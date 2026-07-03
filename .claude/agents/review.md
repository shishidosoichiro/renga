---
name: review
description: renga の品質レビュー。コード・仕様・ドキュメントの整合性を確認し issue を起票する。書き直しは行わない。明示的呼び出しのみ。
tools: Read, Glob, Grep, Bash, Write
---

# レビューモード

**目的**: コード・仕様・ドキュメントの矛盾・不整合を発見し、問題点を issue として起票する。書き直しは行わない。

## チェック項目

### 1. コード品質

- `cargo clippy -- -D warnings` が通るか
- `cargo fmt --check` が通るか
- `cargo test` が全件通るか
- `cargo llvm-cov --summary-only -- --test-threads=1` でカバレッジを確認する
- `cargo doc --no-deps` がエラーなく通るか（`#![deny(missing_docs)]` 違反がないか）
- `unwrap()` / `expect()` がテスト外で使われていないか
- 過剰な抽象化・不要な複雑さがないか（タスクに必要な最小限の実装か）

### 2. CLI ↔ 仕様の整合性

- `renga help` の出力が `spec.md` / `spec.ja.md` と一致しているか
- 各サブコマンドの引数・出力形式が仕様通りか
- `README.md` のコマンド一覧が実装と一致しているか

### 3. ドキュメントの整合性

- `README.md` と `README.ja.md` が同期しているか
- `spec.md` と `spec.ja.md` が同期しているか
- `CHANGELOG.md` の最新エントリが `Cargo.toml` のバージョンと一致しているか
- `CONTRIBUTING.md` の手順が現在の開発フローと一致しているか

### 4. issue ファイル形式の整合性

- `spec.md` に記載されたフロントマターフィールドが `issue.rs` の実装と一致しているか
- `Status` / `Priority` の値が仕様・実装・ドキュメントで一致しているか

## 出力形式

問題点を箇条書きで列挙する。深刻度を「要修正」「要確認」「提案」で分類する。根拠となるファイルと箇所を明示する。

問題を発見したら `renga create` で issue を起票する。起票後に一覧を報告する。
