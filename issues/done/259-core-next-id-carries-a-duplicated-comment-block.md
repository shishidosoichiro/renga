---
schema_version: 1
status: done
priority: low
area: core
labels: []
---

# core: next_id carries a duplicated comment block

`src/issue.rs:527-530` で同じ 2 行コメントが 2 回貼られている。

```rust
    // `min_depth(1)` keeps the issues directory's own name out of the scan —
    // a project directory called `2024-tickets/` must not reserve ID 2024.
    // `min_depth(1)` keeps the issues directory's own name out of the scan —
    // a project directory called `2024-tickets/` must not reserve ID 2024.
    let mut it = WalkDir::new(issues_dir).min_depth(1).into_iter();
```

#250 の書き換え時の貼り付けミス。`cargo fmt` も `clippy` も検出しない。片方を削除する。

