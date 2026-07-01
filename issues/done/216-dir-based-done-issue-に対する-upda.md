---
schema_version: 1
status: done
priority: medium
area: test
labels: []
---

# dir-based done issue に対する update/edit がテストされていない

## 問題

#213 の実装（`find_editable_issue`）により `update`/`edit` は done issue の
フィールド編集ができるようになったが、`tests/integration.rs` に追加された
テスト（`update_can_edit_normal_done_issue`,
`update_can_edit_done_issue_missing_status_field`,
`edit_can_edit_normal_done_issue` 等）はすべてフラットファイル形式
（`done/N-title.md`）のみを対象にしており、ディレクトリ形式
（`done/N-title/README.md`）の done issue に対する `update`/`edit` を
検証するテストが無い。

手動確認では以下の通り動作自体は正しく機能している。

```sh
renga create "Task" --dir=true
renga done 1
renga update 1 --assignee alice   # 成功、issues/done/1-task/README.md が更新される
EDITOR=true renga edit 1          # 成功
```

`find_issue`（`src/issue.rs`）はディレクトリ形式・フラットファイル形式の両方を
扱う共通ロジックなので今回のバグではないが、回帰を検出できるよう
テストで固定化しておくべき。

## 対応案

`tests/integration.rs` に以下を追加する。

- `update_can_edit_dir_based_done_issue`
- `edit_can_edit_dir_based_done_issue`

## 関連

- #213

