---
schema_version: 1
status: open
priority: low
area: core
labels: [found_at:0.1.0]
---

# core: next_id counts the issues root directory's own name when assigning IDs

`next_id` は `WalkDir::new(issues_dir)` を `min_depth` 無しで回すため、issues ルート自身（depth 0）のディレクトリ名からも ID を拾う。

## 再現

```sh
printf 'issues_dir: 2024-tickets\n' > .renga.yml
renga init
renga create "First"
# 2024-tickets/open/2025-first.md   ← 最初の issue が ID 2025
```

## 原因

```rust
// src/issue.rs::next_id
let mut it = WalkDir::new(issues_dir).into_iter();
while let Some(entry) = it.next() {
    ...
    let id_str = if entry.file_type().is_dir() { id_prefix(&name) } else { issue_file_id(&name) };
```

depth 0 のルート `2024-tickets` に対して `id_prefix` が `Some("2024")` を返し、`max` に反映される。

## 修正案

`WalkDir::new(issues_dir).min_depth(1)` にする。`find_issue` / `collect_issue_files` はすでに `min_depth(1)` で揃っている。副次的に、#241 の修正で入れた

```rust
if entry.depth() > 0 && dir_based_issue_readme(&entry).is_some() {
    it.skip_current_dir();
}
```

の `entry.depth() > 0` ガード（ルートで `skip_current_dir()` すると走査全体が止まるため入れたもの）が不要になり、3つの走査の書き方が完全に揃う。

## 関連

- `src/issue.rs::next_id`

