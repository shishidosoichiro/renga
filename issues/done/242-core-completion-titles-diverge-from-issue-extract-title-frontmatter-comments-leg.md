---
schema_version: 1
status: done
priority: low
area: core
labels: [found_at:0.1.0]
---

# core: completion titles diverge from issue::extract_title (frontmatter comments, legacy NNN: prefix)

`src/commands/completions.rs::read_title` は補完候補の説明文（`ID\tTITLE` の TITLE 部分）を独自に抽出しており、`src/issue.rs::extract_title` と挙動が食い違う。

```rust
fn read_title(path: &Path) -> Option<String> {
    let content = fs::read_to_string(path).ok()?;
    content
        .lines()
        .find(|l| l.starts_with("# "))          // frontmatter を読み飛ばさない
        .map(|l| l.trim_start_matches("# ").to_string())  // 旧 "NNN: " prefix を剥がさない
}
```

対して `issue::extract_title` は frontmatter を除いた body に対して走り、`strip_linenum_prefix` でレガシーな `NNN: ` プレフィックスを剥がす。

## 再現（v0.16.0 / 現 HEAD で確認）

**1. frontmatter 内の YAML コメントがタイトルとして拾われる**

```sh
renga create "Real Title"
# issues/open/1-real-title.md の frontmatter 先頭に `# yaml comment here` を挿入

renga __complete renga done ""
# 1	yaml comment here      ← コメントがタイトル扱い
renga list
# [1] open medium   Real Title
```

**2. レガシー `NNN: ` プレフィックスが剥がれない**

```sh
# issues/open/7-legacy.md の本文が `# 007: Legacy Titled` の場合
renga __complete renga done ""
# 7	007: Legacy Titled
renga list
# [7] open medium core   Legacy Titled
```

また `trim_start_matches("# ")` は先頭の `# ` を**繰り返し**剥がすため、`# # Title` のような見出しでも `Title` になる（`strip_prefix` なら `# Title` が残る）。軽微だが `extract_title` と挙動が違う。

## 影響

補完候補の説明文だけの問題で、ID 自体は正しいので操作を誤ることはない。ただし renga が書き出す frontmatter に YAML コメントは含まれないので、症状が出るのは手で編集したファイルとレガシーファイルに限られる。優先度 low。

## 修正案

`read_title` を消し、`issue::parse_issue`（または `extract_title`）でタイトルを取る。#240 で `emit_issues_recursive` を `collect_issue_files` ベースに書き換える際、dir 形式のタイトルフォールバック（`README` になってしまう問題）も同時に解決する必要があるので、#240 とまとめて対処するのが自然。

## 関連

- `src/commands/completions.rs::read_title`（~L296）、`src/issue.rs::extract_title` / `strip_linenum_prefix`
- #239 のレビュー中に発見。#239 の変更による退行ではなく、動的補完の導入（v0.1.0, commit c4d57ed）以来の既存挙動
- #240（completions が dir 形式 issue を候補に出さない）

