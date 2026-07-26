---
schema_version: 1
status: done
priority: high
area: core
labels: [found_in_impl]
---

# core: next_id still counts group_by area directories, so an ID-prefixed area skews IDs

## 背景

#243 は `validate_area_for_group_by` に `id_prefix(&slug).is_some()` の拒否を追加して修正した。しかしこれは入力層のガードで、`next_id` 側の根本原因（ディレクトリ名を無条件に ID とみなす）は残っている。

`next_id` はディレクトリを見つけると `is_dir_based_issue` かどうかに関係なく `id_prefix(&name)` で採番済み ID として数える:

```rust
let id_str = if entry.file_type().is_dir() {
    id_prefix(&name)          // area ディレクトリでもここに入る
} else {
    issue_file_id(&name)
};
```

`is_dir_based_issue` は `holds_status_dirs` で area ディレクトリと dir-based issue を既に区別できているのに、採番はその判定を使っていない。

## 再現（今回の差分を適用した状態のバイナリ）

手書き・旧バージョン作成・他人の clone など、既に `issues/2024-q1/` があるプロジェクト:

```
.renga.yml: group_by: [area]
issues/2024-q1/open/1-q1-task.md   (area: 2024 Q1)

$ renga create "New"
issues/open/2025-new.md      # 期待は 2
```

`renga validate` はエラーを報告するが `--auto-correct` は直さないので、ユーザーは手でディレクトリと frontmatter を書き換えるしかない。

## 提案

`next_id` のディレクトリ分岐を `is_dir_based_issue(&entry)` のときだけ数えるようにする。そうすれば status ディレクトリを持つ area ディレクトリは採番対象から外れ、既存データでも ID が飛ばなくなる。入力層のガード（#243 の修正）はそのまま残してよいが、それだけでは既存データを守れない。

## 出典

レビュー（review エージェント）で発見。手動再現済み。

