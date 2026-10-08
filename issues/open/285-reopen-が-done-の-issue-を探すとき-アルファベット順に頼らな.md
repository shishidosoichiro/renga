---
type: Issue
schema_version: 1
status: open
priority: low
area: core
labels: []
---

# reopen が done の issue を探すとき、アルファベット順に頼らない

## 問題

#284 の対応で、`find_issue` はファイル名の順に辿るようになった。`reopen` が ID の衝突を検出できるのは、同じディレクトリの中で `done/` が `open/` より先に並ぶからである。

ただし、area のディレクトリをまたぐと、この順序は成り立たない。たとえば `api/open/1`（frontmatter の area が誤っている）と `core/done/1` があるとき、`reopen 1` は open 側を見つけて移動しうる。status の名前を変えた場合も、順序が崩れる。

#284 のレビューで指摘された（2026-10-08）。

## 対応案

- 案A: `find_issue` に「done の下だけを探す」選択肢を足し、`reopen` で使う
- 案B: ID が重複しているときは、`reopen` をエラーにする

同じ ID のファイルを作らせない #195 が入れば、問題自体が起きにくくなる。

