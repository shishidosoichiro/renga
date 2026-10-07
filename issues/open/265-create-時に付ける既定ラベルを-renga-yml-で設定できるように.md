---
type: Issue
schema_version: 1
status: open
priority: low
area: config
labels: []
---

# create 時に付ける既定ラベルを .renga.yml で設定できるようにする

## 問題

`renga create` で既定のラベルを付ける設定が無い。`.renga.yml` の `defaults` には `dir` しか無い。

たとえば「新しく起票した issue には必ず `inbox` を付け、トリアージ待ちと分かるようにしたい」という運用を考える。今は、毎回 `--label inbox` を付けるか、エージェントの hook でコマンド文字列を書き換えるしかない。hook での書き換えは、`&&` で連結したコマンドを正しく扱えない。

## 期待する動作（案）

`.renga.yml` に `defaults.labels: [inbox]` を置き、create 時に付ける。

## 吟味すべき論点

実運用では「ほかのラベル（例: リリース予定の版を示すラベル）が無いときだけ inbox を付ける」という条件付きの要望がありうる。条件付きの既定値まで Renga が持つべきかを決める。最小限なら、無条件の既定値と、それを打ち消すオプション（例: `--no-default-labels`）で足りるかを検討する。

