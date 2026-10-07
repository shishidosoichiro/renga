---
type: Issue
schema_version: 1
status: done
priority: medium
area: core
labels: [bug, found_at:0.17.0]
---

# update でラベルを変えると、ブロック形式の labels を持つ issue が invalid frontmatter で失敗する

## 現象

`labels:` を YAML のブロック形式で書いた issue に `renga update <N> --add-label c` を実行すると、`invalid frontmatter ... did not find expected key` で失敗する。ファイルは変更されない。

```markdown
---
status: open
labels:
  - a
  - b
---
```

この issue は `renga validate` を通る（エラー0件）。renga が書く形式（`labels: [a, b]`）ではないが、YAML としては正しいので、手で書かれることはありうる。

## 原因

`set_frontmatter_field` は、`labels:` で始まる最初の1行だけを置き換える。そのため、継続行の `  - a` / `  - b` が残る。update は書き込む前に結果を parse し直すので、ここで失敗する。

## 期待する動作（要検討）

- 案A: 継続行（インデントされた行）を、置き換えの範囲に含める
- 案B: 1行の値でないキーは編集を拒否し、どう直せばよいかをメッセージで示す

ARCHITECTURE.md の不変条件「frontmatter は行単位で書き換える。編集するキーの値は1行である前提」との関係を、どちらの案でも保つこと。

ARCHITECTURE.md のレビュー中に見つかった（2026-10-07）。

