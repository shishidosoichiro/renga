---
schema_version: 1
status: done
priority: high
area: cli
labels: []
---

# cargo fmt --check が tests/integration.rs の複数 ID テストで失敗

## 問題

`cargo fmt --check` が以下の 3 箇所で失敗する:

- `tests/integration.rs:190`: `done_multiple_ids` テスト内のチェーン呼び出し
- `tests/integration.rs:244`: `pending_multiple_ids` テスト内のチェーン呼び出し
- `tests/integration.rs:340`: `reopen_multiple_ids` テスト内のチェーン呼び出し

複数行にまたがっていた `.args([...]).assert().success()` を rustfmt が 1 行にまとめようとしている。

## 修正方針

`cargo fmt` を実行してフォーマットを統一するか、80 文字制限を超える場合は明示的に折り返す。

CI の `cargo fmt --check` が通らないため高優先度。
