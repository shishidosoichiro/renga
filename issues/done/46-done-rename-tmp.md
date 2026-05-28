---
status: done
priority: low
area: test
labels: []
---

# done: rename 失敗時の tmp 削除パスがテストされていない

`src/commands/done.rs` L32-35 の `if let Err(e) = std::fs::rename(...)` ブロック（tmp を削除して Err を返す）が未カバー。

```
33: let _ = std::fs::remove_file(&tmp);   // uncovered
34: return Err(e.into());                  // uncovered
```

## 影響

`cargo llvm-cov` で `commands/done.rs` の line coverage が 87.50% 止まり。
rename 失敗時に tmp ファイルが残る挙動が検証されていない。

## 補足

rename エラーを tempfile 上で再現させることは難しい（クロスデバイス移動など）。
テスト困難な場合は `#[cfg(test)]` でモック的な境界を設けるか、ドキュメントで明示する方針を検討する。
