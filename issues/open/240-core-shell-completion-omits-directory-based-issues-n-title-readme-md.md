---
schema_version: 1
status: open
priority: medium
area: core
labels: []
---

# core: shell completion omits directory-based issues (N-title/README.md)

`__complete` の issue 候補生成（`src/commands/completions.rs::emit_issues_recursive`）はファイル名から ID を導くため、ディレクトリ形式の issue (`N-title/README.md`) が候補に一切出ない。`README.md` は ID プレフィックスを持たないので除外される。

## 再現

```sh
renga create "Dir Task" --dir true   # issues/open/1-dir-task/README.md
renga create "Flat Task"             # issues/open/2-flat-task.md
renga __complete renga done ""
# => 2\tFlat Task   （1 が出ない）
```

`defaults.dir: true` を設定しているプロジェクトでは全 issue が補完から消えることになる。

## 原因

`emit_issues_recursive` は `WalkDir` の各 **ファイル** のファイル名を `issue_file_id` に通す。`issue::collect_issue_files` は「ファイル名が issue 形式」または「**ディレクトリ名**が `id_prefix` を満たし `README.md` が存在する」の 2 分岐で処理しており、completions 側だけ後者が欠けている。

## 修正案

`emit_issues_recursive` を `collect_issue_files` と同じ 2 分岐にする。ただし現状の実装は以下に依存しているため、単純な差し替えではなく調整が必要:

- `status_dir_name(e.path())` による done / not-done フィルタ（`README.md` のパスでも status ディレクトリは同じ階層にあるので流用可）
- `entries.sort_by(file_name)` — ディレクトリ形式では全て `README.md` になるためソートキーが壊れる。ID かディレクトリ名でソートするよう変える
- `read_title` のフォールバック（`stem`）— `README` になってしまうのでディレクトリ名を使う

## 関連

- #239 の修正（`is_issue_file_name` への統一）中に発見した別問題。#239 の範囲外なので分離して起票
- `src/issue.rs::collect_issue_files`

