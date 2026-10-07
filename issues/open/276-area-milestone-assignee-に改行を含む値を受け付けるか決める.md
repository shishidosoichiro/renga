---
type: Issue
schema_version: 1
status: open
priority: low
area: core
labels: []
---

# area・milestone・assignee に改行を含む値を受け付けるか決める

## 背景

#269 の対応で、frontmatter を yaml-edit で書くようにした。その結果、JSON 入力などで area・milestone・assignee に改行を含む値を渡すと、`|-` 形式の正しい YAML として書かれるようになった。以前は frontmatter を壊していた。

ただし、こうした値は次の箇所に影響しうる。

- `renga list` の1行表示
- `group_by` の area ディレクトリ名
- `issues/README.md` の表

labels と同じく改行を拒否するか、受け付けて表示側で扱うかを決める。#269 のレビューで指摘された（2026-10-07）。

