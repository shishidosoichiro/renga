---
type: Issue
schema_version: 1
status: done
priority: medium
area: cli
labels: []
---

# list --label で前方一致・否定・複数指定をできるようにする

## 問題

`renga list --label` は完全一致のラベルを1つしか取れない（`src/cli.rs` の `ListArgs.label` が `Option<String>`）。次の使い方では、`--json` と jq で補うしかない。

- **否定**: inbox ラベルの付いた issue を除いて一覧したい
  ```sh
  renga list --json | jq '[.[] | select((.labels | index("inbox")) == null)]'
  ```
- **前方一致**: `rollback:` のように、接頭辞で種類を表すラベルを持つ issue を探したい
  ```sh
  renga list --json | jq '[.[] | select(.labels[]? | startswith("rollback:"))]'
  ```
- **複数指定**: 2つ以上のラベルを同時に持つ issue に絞りたい

## 期待する動作（案）

- `--label` を繰り返し指定できるようにする（AND）
- 前方一致: `--label 'rollback:*'`
- 否定: `--no-label inbox` か `--label '!inbox'`

記法は既存の CLI の慣習を調べてから決める（gh・glab の `--label`、jira 等）。

