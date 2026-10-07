---
type: Issue
schema_version: 1
status: open
priority: medium
area: ci
labels: []
---

# CI の clippy を --all-targets で実行する

## 問題

CONTRIBUTING.md と review エージェントは `cargo clippy --all-targets -- -D warnings` を実行する（#277）。CI（`.github/workflows/ci.yml`）は `cargo clippy -- -D warnings` のままで、テストのターゲットを lint しない。

## やること

README.md・README.ja.md の開発コマンドは #277 で揃えた。CI の clippy を `--all-targets` に揃える。#277 の時点で `--all-targets` は既存コードで通ることを確かめてある。
