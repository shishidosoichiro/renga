---
schema_version: 1
status: done
priority: medium
area: docs
labels: []
---

# README.md の Commands テーブルに literal \n が混入している

README.md の 125 行目と README.ja.md の 125 行目で、テーブルの行区切り（改行）として \n が文字列として埋め込まれており、Markdown として正しく表示されない。show/edit/update の各行が 1 セルに結合されて見える。
