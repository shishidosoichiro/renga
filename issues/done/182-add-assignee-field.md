---
schema_version: 1
status: done
priority: medium
area: core
labels: []
---

# feat: add assignee field to issue front matter

issue の front matter に assignee フィールドを追加する。

## 背景

現在、kiwi プロジェクトでは area フィールドでエージェントの担当を代用しているが、area はコード領域を示す概念であり、担当者とは別物。概念の分離が必要。

## 仕様

- front matter に `assignee: string` を追加（単数・省略可能）
- `renga create --assignee <name>` オプションを追加
- `renga list` / `renga update` でも扱えるようにする
- spec.md / spec.ja.md のドキュメント更新も含める
