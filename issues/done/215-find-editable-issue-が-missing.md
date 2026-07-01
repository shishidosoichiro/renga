---
schema_version: 1
status: done
priority: medium
area: core
labels: [found_in_impl]
---

# find_editable_issue が missing-status の done issue を無条件に編集可能にしてしまう

## 問題

`find_editable_issue`（`src/issue.rs`）のフォールバックは `find_active_issue` が
`None` を返した場合に `find_issue(issues_dir, id, true)` を呼び、frontmatter の
status を一切確認せずに `done/` 配下のファイルを ID だけでマッチさせている。

doc コメントは以下のように「通常の done issue（frontmatter status: done）」に
限定すると書いているが、実装はそれを検証していない。

```rust
/// Extends [`find_active_issue`] by also matching a normal `done/` issue
/// (frontmatter `status: done`, correctly stored under `done/`). ...
pub fn find_editable_issue(issues_dir: &Path, id: &str) -> Result<Option<ActiveIssuePath>> {
    if let Some(active) = find_active_issue(issues_dir, id)? {
        return Ok(Some(active));
    }
    let Some(path) = find_issue(issues_dir, id, true)? else {
        return Ok(None);
    };
    Ok(Some(ActiveIssuePath { path, warning: None }))
}
```

`find_active_issue` は、frontmatter に明示的な `status` フィールドが無い
`done/` 配下のファイルを「misplaced active issue」として認識できず（
`explicit_frontmatter_status` が `None` を返すため）`None` を返す。結果として
`find_editable_issue` のフォールバックが素通しでそのファイルを「編集可能」と
判定してしまう。

これは既存の設計判断と矛盾する。`tests/integration.rs` の
`pending_does_not_operate_on_done_issue_with_missing_status` は、
status フィールドが無い `done/` 配下のファイルに対して `pending` が
意図的に "not found" として扱うことを検証している（曖昧なファイルには
手を出さない、という設計）。

## 再現手順

```sh
mkdir -p issues/done
cat > issues/done/1-missing-status.md <<'MD'
---
schema_version: 1
priority: medium
area: core
labels: []
---

# Missing status
MD

renga pending 1
# => error: issue 1 not found  （意図通り）

renga update 1 --assignee alice
# => 成功し、assignee: alice が書き込まれる（矛盾）
```

## 期待する挙動

`update`/`edit` も `pending`/`done`/`in-progress` と同様、status フィールドが
存在しない曖昧な `done/` ファイルに対しては "not found" として扱うべき。
`find_editable_issue` のフォールバックは `find_issue(.., true)` に丸投げする
のではなく、`explicit_frontmatter_status` などで frontmatter status が
明示的に `done` であることを確認してから受理する必要がある。

`update_can_edit_done_issue_missing_status_field`
（`tests/integration.rs`）はこの矛盾した挙動をそのままテストとして
固定化してしまっているため、修正時に併せて見直す必要がある。

## 関連

- #213（この issue の元になった実装変更）
- `src/issue.rs` の `find_active_issue` / `find_editable_issue`
- `tests/integration.rs` の `pending_does_not_operate_on_done_issue_with_missing_status`

