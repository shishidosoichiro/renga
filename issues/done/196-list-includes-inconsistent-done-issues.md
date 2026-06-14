---
schema_version: 1
status: done
priority: medium
area: core
labels: []
---

# renga list のデフォルト出力に frontmatter 不整合の done issue が混入する

## 問題

`renga list` はデフォルトで open・pending・in-progress のみ返すはずだが、
done/ ディレクトリに存在するファイルの frontmatter が status: open になっている場合、
そのファイルが open issue として出力に混入する。

今回は done/ に誤配置された37件が `renga list --json` に現れ、
assignee 付与スクリプトが全件対象のつもりで処理したのに
`renga update` では not found になる、という二重の混乱を引き起こした。

## 期待する動作

status の正とする情報源は frontmatter とする。
`renga validate` が frontmatter status と実在ディレクトリの不一致を error として検出し、
`renga validate --auto-correct` が frontmatter status に合わせてファイルを移動できる。

`renga list` は frontmatter status を表示・フィルタする現行方針を維持する。
ただし不整合があると list と update 系コマンドの見え方がずれるため、
validate で確実に検出・修復できる必要がある。
