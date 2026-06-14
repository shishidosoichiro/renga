---
schema_version: 1
status: open
priority: medium
area: core
labels: []
---

# issue 作成時に重複 ID を防止できない

## 問題

同じ ID のファイルが複数存在しても `renga create` 時に弾かれない。
今回 #188 が2ファイル存在していることが `renga validate` で判明した。

## 期待する動作

`renga create` 時に既存 ID と衝突しないことを確認する。
または `renga validate` を常時走らせて重複を即座に検出できる仕組みを提供する。
