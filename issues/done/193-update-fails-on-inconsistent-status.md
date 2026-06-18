---
schema_version: 1
status: done
priority: high
area: core
labels: []
---

# status 不整合ファイルに対して renga update / renga pending 等が not found になる

## 問題

done/ ディレクトリに存在するが frontmatter が status: open のファイルに対して
`renga update`・`renga pending`・`renga done` 等を実行すると
「issue not found」エラーになり操作できない。
ディレクトリを直接操作するしかなくなる。

## 期待する動作

status の正とする情報源は frontmatter とする。
`renga validate [ID]...` が frontmatter status と実在ディレクトリの不一致を error として検出し、
`renga validate [ID]... --auto-correct` が frontmatter status に合わせてファイルを移動できる。

`renga update`・`renga pending`・`renga done` 等で不整合ファイルに当たった場合は、
少なくとも `renga validate <ID> --auto-correct` に誘導できるわかりやすいエラーメッセージを出す。
