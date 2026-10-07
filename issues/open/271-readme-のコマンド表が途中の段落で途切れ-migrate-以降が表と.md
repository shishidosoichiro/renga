---
type: Issue
schema_version: 1
status: open
priority: low
area: docs
labels: []
---

# README のコマンド表が途中の段落で途切れ、migrate 以降が表として表示されない

## 問題

`README.md` と `README.ja.md` のコマンド表には、途中に段落が挟まっている（「Pass an empty string to `--milestone`…」の段落）。Markdown の表は空行で終わるので、`renga migrate` 以降の行は表として表示されないはずである。

#263 のレビュー中に見つかった（2026-10-07）。

## 対応案

段落を表の後ろへ移す。英語版と日本語版の両方を直す。

