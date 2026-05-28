---
status: done
priority: low
area: core
labels: []
---

# done: dest.exists() ガードがなく done/ の既存ファイルを黙って上書きする

`src/commands/done.rs` には `reopen.rs` にある `dest.exists()` ガードが存在しない。

## reopen.rs（L23-29）にはガードがある

```rust
if path != dest && dest.exists() {
    anyhow::bail!(
        "cannot reopen {}: {} already exists as an open issue",
        ...
    );
}
```

## done.rs にはガードがない

`done/N-slug.md` が既に存在する場合（手動配置や前回の不完全操作など）、`std::fs::rename(&tmp, &dest)` が POSIX のアトミック上書きでそのまま置き換える。エラーも警告も出力されない。

## 影響

意図しないファイルの上書き損失が起きてもユーザーが気づかない。
`reopen.rs` との設計上の非対称性として、意図的なら仕様コメントが必要。
