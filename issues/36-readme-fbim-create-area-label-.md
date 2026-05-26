---
status: open
priority: medium
area: docs
labels: []
---

# README の fbim create にオプション（--area, --label, --slug 等）が一切記載されていない

- **ファイル**: `README.md:63`, `README.ja.md:63`
- **内容**: コマンド一覧表の `fbim create` 行はタイトルのみで、`--area`・`--label`・`--slug`・`--priority` 等のオプションが一切書かれていない。`fbim list` は多数のオプションが記載されているのに非対称
- **対応**: `fbim create --help` の出力を元に README 両版を更新する
