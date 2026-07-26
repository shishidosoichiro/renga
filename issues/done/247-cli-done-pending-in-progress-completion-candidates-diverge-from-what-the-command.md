---
schema_version: 1
status: done
priority: low
area: cli
labels: [found_in_impl]
---

# cli: done/pending/in-progress completion candidates diverge from what the commands accept

completions を `all_issues`（frontmatter の status で絞る）に一本化したことで、`done` / `pending` / `in-progress` の候補と、コマンドが実際に受け付ける issue がずれた。frontmatter とディレクトリが食い違っている issue で顕在化する。

## 再現

```sh
mkdir -p issues/open issues/done && touch .renga.yml
printf -- '---\nstatus: done\n---\n\n# Misplaced Done\n' > issues/open/5-misplaced.md
printf -- 'no frontmatter\n'                             > issues/done/6-nofm.md

renga __complete renga done ""
# 6	6-nofm            ← コマンドは受け付けない
# （5 が出ない）      ← コマンドは受け付ける

renga done 6
# error: no frontmatter in .../issues/done/6-nofm.md   (exit 1)

renga done 5
# .../issues/done/5-misplaced.md                       (exit 0)
```

## 原因

`ACTIVE_STATUSES` に `Status::Unknown` が含まれるため、`done/` にある frontmatter 無しファイル（= `Unknown`）が active 候補として出る。しかし `done` は `find_active_issue` → `explicit_frontmatter_status` を通り、`done/` にあって frontmatter が無いファイルは `no frontmatter in ...` でエラーになる。

逆に `open/` にあって frontmatter が `done` の issue（`Status::Done`）は候補から外れるが、`find_active_issue` は `open/` にあるので普通に見つけてしまい、コマンドは成功する。

`open/` にある frontmatter 無しファイルを候補に出すのは正しい（コマンドも成功する）ので、`Unknown` を一律に外すのは誤り。「`Unknown` かつ `done/` 配下」だけが問題。

## 修正案の論点

- 候補側を寄せる: `Unknown` は `status_dir_name != done` のときだけ active 候補にする
- コマンド側を寄せる: `find_active_issue` が `done/` の frontmatter 無しファイルをエラーではなく「active ではない」として扱う（`explicit_frontmatter_status` の `with_context` を `Ok(None)` に）
- `open/` にある frontmatter `done` を `done` の候補に戻すかどうかは別途判断（今の挙動は `tests/integration.rs::complete_done_filters_on_frontmatter_not_directory` が固定している）

## 関連

- `src/commands/completions.rs::ACTIVE_STATUSES`、`src/issue.rs::find_active_issue`
- #240（completions の書き換え）、#244（`reopen` 側の同種のずれ）

