---
schema_version: 1
status: open
priority: medium
area: cli
labels: [found_at:0.9.1]
---

# cargo fmt --check が通らない: issue.rs の空行

src/issue.rs の128行目と131行目の間に余分な空行が1行ある（}のあとにimplが続く部分）。cargo fmt --check が exit code 1 で失敗する。今回の変更（cli.rs/completions.rs）とは無関係な既存の問題。
