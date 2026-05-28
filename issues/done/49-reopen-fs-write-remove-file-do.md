---
status: done
priority: low
area: core
labels: []
---

# reopen: fs::write + remove_file が非アトミックで done.rs の tmp/rename パターンと非対称

`src/commands/done.rs`（#20 修正）では tmp ファイル経由の rename パターンを採用した。

```rust
// done.rs: アトミック
let tmp = dest.with_extension("tmp");
std::fs::write(&tmp, &updated)?;
std::fs::rename(&tmp, &dest)?;
std::fs::remove_file(&path)?;
```

一方、`src/commands/reopen.rs`（#19 修正）は `dest.exists()` ガードを追加したが、書き込みは依然として非アトミック。

```rust
// reopen.rs: 非アトミックのまま
std::fs::write(&dest, &updated)?;   // write 成功
if path != dest {
    std::fs::remove_file(&path)?;   // crash するとこの前後で両方に中途状態が残る
}
```

`fs::write` が完了して `remove_file` の前にクラッシュした場合、`issues/1-old.md`（status: open）と `done/1-old.md`（status: open に書き換え済み）が両方に存在する状態になる。

## 影響

`done.rs` との設計一貫性の欠如。クラッシュ耐性を `done.rs` に寄せたなら `reopen.rs` も揃えるか、意図的な差分として理由をコメントで説明する必要がある。
