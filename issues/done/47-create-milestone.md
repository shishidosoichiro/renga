---
status: done
priority: medium
area: test
labels: []
---

# create: --milestone 指定パスとファイルオープンエラーパスがテストされていない

`src/commands/create.rs` に 2 か所の未カバーパスがある。

## 1. L26: `--milestone` 指定時のパス（uncovered）

```rust
let milestone_line = match &args.milestone {
    Some(m) => format!("milestone: {m}\n"),  // ← uncovered (L26)
    None => String::new(),
};
```

`tests/integration.rs` に `--milestone` を渡すテストがない。

## 2. L41: `write_all` の `?` エラーパス（uncovered）

```rust
std::io::Write::write_all(
    &mut std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)?,  // ← ここが失敗したときのパス
    content.as_bytes(),
)?;                     // ← write_all 失敗時も未カバー
```

書き込みエラー（ディスク満杯など）時のパスがテストされていない。

## 追加: エラーメッセージにコンテキストがない

`OpenOptions` が失敗した際のエラーメッセージが OS raw error のみ：

```
error: Permission denied (os error 13)
```

`done.rs` や `reopen.rs` では `with_context(|| format!("invalid path: {}", path.display()))` を使っているが、`create.rs` では未対応。どのパスへの操作が失敗したか分からない。

## 影響

`commands/create.rs` の region coverage が 88.24%（line 92.00%）に留まっている。
