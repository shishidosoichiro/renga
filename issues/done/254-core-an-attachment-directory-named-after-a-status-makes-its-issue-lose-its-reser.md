---
schema_version: 1
status: done
priority: medium
area: core
labels: [bug]
---

# core: an attachment directory named after a status makes its issue lose its reserved ID

## 症状

`is_dir_based_issue` は「ステータスディレクトリを持つか」で area ディレクトリと dir-based issue を区別する（`src/issue.rs:394-410` の `holds_status_dirs`）。そのため dir-based issue が `open`/`pending`/`in-progress`/`done`/`unknown` のいずれかの名前の添付ディレクトリを持つと、その issue ディレクトリ自体が area ディレクトリと誤判定される。

#250 の修正で `next_id` がディレクトリの採番を `is_dir_based_issue` に委ねるようになったため、この誤判定が **ID の再利用** に直結するようになった。修正前の `next_id` は ID プレフィックスを持つディレクトリを無条件に数えていたので、誤判定されても ID は予約されていた。

## 再現

```sh
mkdir -p w/issues/open/12-refactor/done
printf -- "---\nstatus: open\npriority: medium\n---\n\n# Refactor\n" > w/issues/open/12-refactor/README.md
printf "issues_dir: issues\n" > w/.renga.yml
cd w && renga create "task 1" && renga create "task 2"
```

実測:

```
issues/open/1-task-1.md
issues/open/12-refactor/README.md   <- 不可視・ID 未予約
issues/open/2-task-2.md
renga validate  -> ok
```

修正前は `next_id` が 12 を数えて次の ID は 13 になっていた。現在は 1 から振り直され、issue を 12 件作った時点で ID 12 が重複する。`list`・`validate` からは issue 12 が完全に見えないため、重複が起きるまで気付けない。

## 補足

`list` から見えなくなること自体は #243 の `holds_status_dirs` 判定に由来する既存挙動。本 issue は #250 がそこに採番を結び付けたことで生じた新しい影響（ID 再利用）を扱う。

## 検討する方向

- `is_dir_based_issue` の判定に「`README.md` を持つか」を加える（area ディレクトリは通常 README を持たないが、`list_includes_issues_under_area_holding_its_own_readme` のテストが示すとおり持ちうるので単独では不十分）
- `next_id` だけは保守的に、ID プレフィックスを持つディレクトリを（area 判定にかかわらず）予約する側に倒す。area が `2024-q1` の場合に ID 2024 が飛ぶ問題は #250 で解決したい対象なので、この方向は #250 の趣旨と衝突する点に注意
- 関連: #195（重複 ID の防止）

