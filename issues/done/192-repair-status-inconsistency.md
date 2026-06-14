---
schema_version: 1
status: done
priority: medium
area: core
labels: []
---

# ディレクトリと frontmatter の status 不整合を自動修正するコマンドがない

## 問題

issues/done/ にあるのに frontmatter の status が open のままのファイルが生じることがある。
status の正とする情報源は frontmatter で、ステータス別ディレクトリは同期対象のレイアウト。
`renga validate` はこの不整合を error として検出するべきだが、修正するオプションがない。
ユーザーはファイルを手動で移動・編集するしかない。

## 期待する動作

`renga validate [ID]... --auto-correct` で、frontmatter の status に合わせて
ファイルを正しいステータス別ディレクトリへ移動できる。
ID 未指定時は全 issue、ID 指定時は対象 issue だけを検査・修正する。
