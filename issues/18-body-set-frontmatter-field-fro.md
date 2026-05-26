---
status: open
priority: medium
area: core
labels: []
---

# body 中の '---' で set_frontmatter_field が frontmatter 書き換えロジックを再発動しデータ破損する

- **ファイル**: `src/issue.rs:374`
- **再現シナリオ**: issue body に `---`（Markdown HR）があり、その直後に `status: see notes` という行がある状態で `fbim done`/`pending`/`reopen` を実行すると、`in_fm` が再 ON されて body の行が `status: done` 等で上書きされ、ドキュメント内容が破損する
