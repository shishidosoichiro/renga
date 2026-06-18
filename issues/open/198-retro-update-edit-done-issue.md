---
schema_version: 1
status: open
priority: medium
area: agent
labels: [retro]
---

# retro: update/edit の done issue 操作方針で反証を出さずに同意した

## 改善メモ

- closed issue そのものを `update` / `edit` 可能にする話と、frontmatter status が active なのに `done/` に置かれた不整合 issue を操作可能にする話を分けて扱う。
- #193 では後者だけを対象にする。frontmatter status を authoritative source とするため、`done/` 配下でも `open` / `pending` / `in-progress` なら操作を継続し、warning で `renga validate <ID> --auto-correct` を案内する。
- 通常の `status: done` issue を close 後に追記・更新できるようにするかは、別 issue で UX と監査性を検討する。
