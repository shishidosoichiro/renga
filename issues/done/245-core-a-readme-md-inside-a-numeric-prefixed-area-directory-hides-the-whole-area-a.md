---
schema_version: 1
status: done
priority: medium
area: core
labels: [found_in_impl]
---

# core: a README.md inside a numeric-prefixed area directory hides the whole area and duplicates IDs

#241 の修正で入れた「`N-slug` ディレクトリでも `README.md` が無ければ dir 形式 issue とみなさない」というガード（`src/issue.rs::dir_based_issue_readme`）には抜け道がある。area ディレクトリに `README.md` が置かれると、そのガードが逆に働いて area 配下の issue が全部消える。

## 再現

```sh
renga init
printf 'group_by: [area]\n' > .renga.yml
renga create "Q1 task"     --area "2024 Q1"
renga create "Q1 task two" --area "2024 Q1"
renga list
# [1]    open medium 2024 Q1  Q1 task
# [2025] open medium 2024 Q1  Q1 task two      ← ID の飛びは #243

printf '# 2024 Q1 area\n' > issues/2024-q1/README.md   # area の説明を置く（人間の自然な操作）

renga list
# （何も出ない）                                        ← area 配下の issue が全消失

renga create "another" --area "2024 Q1"
# issues/2024-q1/open/2025-another.md                   ← ID 2025 が重複
```

## 原因

`dir_based_issue_readme` は「ディレクトリ名に `id_prefix` があり、`README.md` が存在する」だけで dir 形式 issue と判定し、呼び出し側が `skip_current_dir()` で配下を走査対象から外す。area ディレクトリ `2024-q1/` に `README.md` があると、この条件を満たしてしまう。

- `collect_issue_files` → area 配下を走査しない → `list` / `validate` / `readme` から issue が消える
- `next_id` → area 配下を走査しない → 既存 ID を見落として重複採番する（データ破損）
- `find_issue` → `renga done 2024` が area ディレクトリごと `issues/done/2024-q1/` へ移動しうる（#241 と同じ破壊）

## #243 との関係

#243 の本文にある「#241 の修正で `README.md` が無ければ issue とみなさないガードを入れたので、area ディレクトリが誤って skip される事故は起きない」という記述は誤り。`README.md` があれば起きる。#243 を修正するときにこの記述も直す。

`validate_area_for_group_by` で数字プレフィックスの area slug を拒否すれば新規 area については塞がるが、既存リポジトリ・手で作ったディレクトリは残る。

## 修正案

dir 形式 issue の判定を `README.md` の有無より強い条件にする。dir 形式 issue は必ず status ディレクトリ（`open`/`pending`/`in-progress`/`done`/`unknown`）の直下にあり、area ディレクトリは issues ルート直下にあるので、`status_dir_name(entry.path())` が `Status` として解釈できることを条件に加えれば区別できる。

## 関連

- `src/issue.rs::dir_based_issue_readme`、`collect_issue_files`、`next_id`、`find_issue`
- #241（元の修正）、#243（数字プレフィックス area slug）

