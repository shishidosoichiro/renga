---
schema_version: 1
status: done
priority: medium
area: cli
labels: []
---

# pending/in-progress/reopen: 部分失敗テストが欠けている

## 問題

`done_partial_failure` テストは存在するが、同様のシナリオが `pending`・`in_progress`・`reopen` にはない。

## 欠けているテスト

- `pending_partial_failure`: 存在する ID と存在しない ID を混在させ、成功分が moved + exit 1
- `in_progress_partial_failure`: 同上
- `reopen_partial_failure`: done 済み ID と存在しない ID を混在させ、成功分が reopened + exit 1

各テストは:
1. exit code 1 を確認
2. stderr に 'not found' を含む
3. 成功した ID が正しいディレクトリに移動済みであることをアサート

の 3 点すべてを確認すること（`done_partial_failure` の実装を参考にする）。
