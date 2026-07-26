---
schema_version: 1
status: done
priority: medium
area: core
labels: [found_at:0.15.0]
---

# core: files named N-slug.md inside a directory-based issue are treated as separate issues

ディレクトリ形式の issue (`N-title/`) の中に `N-slug.md` という名前のファイルを置くと、renga がそれを独立した issue として扱ってしまう。`issue::collect_issue_files` も `completions::emit_issues_recursive` も**ファイル名だけ**で issue 判定しており、そのファイルが status ディレクトリ直下（または `group_by` の area/status 直下）にあるかどうかを検証していないため。

spec.md:35 / spec.ja.md:35 は dir 形式を「添付ファイルやメモを issue と同じ場所に置ける」形式として案内しているので、`1-dir-task/9-design.md` のような命名は正当な運用の範囲内。

## 再現（v0.16.0 / 現 HEAD で確認）

```sh
renga init
renga create "Dir Task" --dir true          # issues/open/1-dir-task/README.md
cat > issues/open/1-dir-task/9-design.md <<'MD'
---
schema_version: 1
status: open
priority: high
area: core
labels: []
---

# Attached design note
MD

renga list
# [1] open medium                  Dir Task
# [9] open high   core             Attached design note   ← 添付が issue として出る

renga done 9
# .../issues/done/9-design.md        ← 親ディレクトリの外へ移動してしまう
```

移動後のツリー:

```
issues/done/9-design.md          ← 添付ファイルが親 issue から剥がされた
issues/open/1-dir-task/README.md
```

## 影響

| コマンド | 症状 |
|---|---|
| `done` / `pending` / `in-progress` / `reopen` | 添付ファイルを親ディレクトリの外へ移動し、dir 形式 issue の中身を壊す（データ破壊） |
| `list` / `show` / `update` / `edit` | 添付ファイルを独立 issue として扱う |
| `validate` | frontmatter を持たない添付（例: `3-notes.md`）に対して `unparseable frontmatter` エラーを出し exit 1 |
| `next_id` | 添付ファイルの数字を ID として数えるので採番が飛ぶ（`3-notes.md` があると次が 4 になる） |
| `__complete` | 添付ファイルが補完候補に出る（`3\tSome attachment notes`） |

## 原因

`src/issue.rs::collect_issue_files` は `WalkDir` の全深さを走査し、`is_issue_file_name(name)`（= ファイル名が `N-slug.md`）だけで判定する。ディレクトリ形式 issue の中は「issue の内部」であって、そこにある `.md` を再び issue として拾ってはいけない。`emit_issues_recursive`（#239 で `issue_file_id` に統一済み）も同じ判定なので同じ結果になる。

## 修正案

`collect_issue_files` で「ディレクトリ名が `id_prefix` を満たすディレクトリ」を検出したら、その配下を `WalkDir` の `skip_current_dir()` で丸ごと降りないようにする（`README.md` だけを issue として採用する）。`emit_issues_recursive` 側は #240 で `collect_issue_files` に寄せる方針なので、そこで自動的に直る。

## 関連

- `src/issue.rs::collect_issue_files`（~L384）、`src/commands/completions.rs::emit_issues_recursive`
- #239（completions のファイル名フィルタ統一）のレビュー中に発見。#239 が入れた変更による退行ではなく、dir 形式導入（v0.15.0, commit 3450c3e）以来の既存バグ
- #240（completions が dir 形式 issue を候補に出さない）と同じ「ファイル名だけで issue を判定している」問題の裏返し

