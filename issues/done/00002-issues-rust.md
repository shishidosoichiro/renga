---
status: done
priority: high
area: bug
labels: []
---

# 上位ディレクトリへの issues/ 探索が Rust 移植で失われた

Rust 移植前は issues ディレクトリをカレントディレクトリから上位へ辿って探す機能があったが、現在の実装ではカレントディレクトリの issues/ しか見ない。このためサブディレクトリ（例: kiwi/docs/）から実行した場合に上位（kiwi/issues/）を見つけられない。kiwi/issues/ をプロジェクト横断の issues リポジトリとして使うユースケースで必要。
