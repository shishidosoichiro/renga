---
schema_version: 1
status: open
priority: low
area: docs
labels: []
---

# CONTRIBUTING.md の --test-threads=1 が今も必要か確かめる

## 問題

CONTRIBUTING.md には、「テストが CWD を変える場合は `cargo test -- --test-threads=1` が必要」と書かれている。カバレッジ計測のコマンドにも `--test-threads=1` が付いている。

ところが 2026-10-07 時点で、`src/` と `tests/` に `set_current_dir` は無い。`tests/integration.rs` は、コマンドごとに `current_dir` を指定している。`cargo test` を並列で1回実行したところ、全件が通った（unit 201件、integration 73件、doctest 10件）。

## 確認すること

- 並列実行を複数回くり返して、不安定なテストが無いか
- `--test-threads=1` が必要だった経緯（git log）

不要と分かったら、CONTRIBUTING.md と `/commit` スキルのコマンド例から外す。`/commit` スキルは `.claude/` 配下なので、self-improve を通す。

