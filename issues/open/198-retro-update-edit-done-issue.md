---
type: Issue
schema_version: 1
status: open
priority: medium
area: agent
labels: [retro, unverified-premise]
---

# retro: update/edit の done issue 操作方針で反証を出さずに同意した

## 観測

`update` / `edit` で done の issue を操作する方針について、反証を出さずに同意した。

## そのとき何が見えていたか

- 2つの論点が1つとして扱われていた。closed issue そのものを編集可能にする話と、frontmatter の status が active なのに `done/` に置かれた不整合 issue を操作可能にする話である
- 当時の判断材料の詳細は記録に残っていない
- 論点はその後、#193（不整合 issue の操作）と #213（done issue の編集）に分かれた

## 推測

2つの論点が混ざったまま、前提を確かめずに同意した可能性がある。
