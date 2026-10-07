---
type: Issue
schema_version: 1
status: done
priority: medium
area: core
labels: [bug, found_at:0.17.0]
---

# ラベルに : や # を含めると、create が読めない issue ファイルを作る

## 現象

`validate_label` は `, [ ] { }` と末尾の `*` しか拒否しない。そのため、`renga create --label "a: b"` は成功する。できたファイルは `labels: [a: b]` となる。これはインライン YAML として不正なので、次のようになる。
- `renga list` に出てこない
- `renga validate` は `unparseable frontmatter` と報告する

`#x` や空文字列でも同じことが起きる。

#265 で `defaults.labels` を足したので、影響が広がった。`.renga.yml` の既定ラベルにこうした値が入ると、以後の `create` がすべて壊れたファイルを作る。

#265 のレビューで見つかった（2026-10-07）。

## 対応案

- 案A: ラベルを書き出すときに、インライン YAML として必要に応じて引用符で囲む
- 案B: `validate_label` で、YAML の構文に関わる文字（`:` の後ろの空白、`#`、先頭の特殊文字、空文字列）を拒否する

ARCHITECTURE.md の不変条件（frontmatter は行単位で書き換え、値は1行）と両立する方を選ぶ。

