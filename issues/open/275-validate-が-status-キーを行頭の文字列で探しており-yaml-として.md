---
type: Issue
schema_version: 1
status: open
priority: low
area: core
labels: [bug, found_at:0.17.0]
---

# validate が status キーを行頭の文字列で探しており、YAML として判定していない

## 現象

`src/commands/validate.rs` は、frontmatter に `status` があるかを行頭の `status:` で判定している。そのため、次のような YAML として正しい書き方を「missing status」と誤って報告する。

- `status : done`
- `"status": done`

#269 の対応で、frontmatter を YAML として編集する方針に変えた。読み手側の判定もそれに揃える。#269 のレビューで見つかった（2026-10-07）。

## 対応案

serde_yaml の Mapping で `status` キーの有無を判定する。

