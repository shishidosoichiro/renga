---
status: done
priority: medium
area: cli
labels: []
---

# --status フィルタの不正値を filter_map(ok()) で黙殺しエラーも警告も出さない

- **ファイル**: `src/commands/list.rs:17`
- **再現シナリオ**: `fbim list --status opne,done`（typo）を実行すると `"opne"` が `.ok()` で黙って捨てられ、done のみが返る。ユーザーはフィルタが部分無効化されていることに気づかない
