---
schema_version: 1
status: open
priority: medium
area: core
labels: []
---

# area や assignee を条件にした一括更新コマンドがない

## 問題

`renga update` は1件ずつしか操作できない。
たとえば「area が docs の全 issue に assignee: docs-designer を設定する」
といった一括操作を行うにはシェルスクリプトで `renga update` を繰り返すしかない。
134件に assignee を付与する際に不便だった。

## 期待する動作

`renga update --area docs --assignee docs-designer` のように
フィルタ条件を指定して一括更新できるコマンド、またはオプション。
