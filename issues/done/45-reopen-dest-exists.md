---
status: done
priority: medium
area: test
labels: []
---

# reopen: dest.exists() ガードのエラーパスがテストされていない

`src/commands/reopen.rs` L23-29 の `dest.exists()` による `bail!` はカバレッジ未カバー（line 24-25 が uncovered）。

## 再現シナリオ

1. issue #1 を done する（`done/1-foo.md` が生成される）
2. `issues/1-foo.md` を手動で作成する
3. `fbim reopen 1` を実行 → bail! でエラーになるはず

このパスのテストが `tests/integration.rs` に存在しない。

## 影響

`cargo llvm-cov` で `commands/reopen.rs` の region coverage が 73.58% に留まっている。
