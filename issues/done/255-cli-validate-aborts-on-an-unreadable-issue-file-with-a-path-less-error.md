---
schema_version: 1
status: done
priority: medium
area: cli
labels: [bug]
---

# cli: validate aborts on an unreadable issue file with a path-less error

## 症状

#252 で `all_issues` に追加したコメント（`src/issue.rs:454-458`）は

> `renga validate` reports the I/O error in full.

と書いているが、実際の `validate` は I/O エラーを報告しない。`src/commands/validate.rs:79` が

```rust
let content = std::fs::read_to_string(path)?;
```

と `.with_context()` なしで `?` しているため、1 ファイルでも読めないと **検証全体が中断** し、しかもパスを含まないメッセージだけが出る。

## 実測

```
$ renga list
warning: cannot read /.../issues/open/2-broken/README.md: Is a directory (os error 21)
[1] open medium                  Ok

$ renga validate
error: Is a directory (os error 21)
exit=1
```

`list` は「どのファイルが」を教えてくれるのに、原因究明のためのコマンドである `validate` が最も情報の少ない出力になっている。

## 期待

- `read_to_string` に `.with_context(|| format!("failed to read {}", path.display()))` を付ける
- さらに望ましくは、読めないファイルを `Finding { message: "cannot read: {e}", is_error: true }` として報告し、他のファイルの検証は続行する（`all_issues` が読めないファイルで検証を止めないのと同じ方針に揃える）
- `src/commands/migrate.rs:59` と `:122` も同じく context なしの `?` なので併せて見直す
- 上記のいずれかを行うまで、`src/issue.rs` のコメントの記述は事実と異なる

