---
type: Issue
schema_version: 1
status: open
priority: medium
area: core
labels: []
---

# serde_yaml（deprecated）から読み込みを移行する

## 問題

frontmatter の読み込みに使っている `serde_yaml` は、0.9.34 の時点（2024-03）で deprecated になり、保守が終わっている。書き込み側は yaml-edit に移した（#269 の対応）。読み込みと、編集前後の検証は、まだ serde_yaml に頼っている。

## 比較する候補

- 保守されている後継の crate（serde_yaml_ng、serde_norway など）。API がほぼ同じなら、置き換えが小さく済む
- yaml-edit での読み込み。読み手と書き手が同じ構文木を使うので、判定のずれが無くなる。ただし serde のような derive による型付けは無い

## 確かめること

- 重複キーを拒否するか（今の読み手の挙動）
- YAML 1.1 と 1.2 の扱いの差。たとえば `yes`、`1.0` を文字列として読むか
- 読み込みの性能（issue が数百件ある場合）

