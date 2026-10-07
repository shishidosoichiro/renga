---
type: Issue
status: done
priority: high
area: bug
labels: []
---

# 上位ディレクトリへの issues/ 探索が Rust 移植で失われた

Rust 移植前は issues ディレクトリをカレントディレクトリから上位へ辿って探す機能があったが、現在の実装ではカレントディレクトリの issues/ しか見ない。このためサブディレクトリ（例: project/docs/）から実行した場合に上位（project/issues/）を見つけられない。上位の issues/ を複数リポジトリ横断の issues リポジトリとして使うユースケースで必要。
