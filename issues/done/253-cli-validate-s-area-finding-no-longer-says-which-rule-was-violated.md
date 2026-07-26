---
schema_version: 1
status: done
priority: low
area: cli
labels: [found_in_impl]
---

# cli: validate's area finding no longer says which rule was violated

## 背景

#243 の修正で `validate` の Finding メッセージを両ケース共通の文言に変えた。

`src/commands/validate.rs:126`:

```rust
message: "area is not usable as a directory name",
```

`Finding.message` が `&'static str` なので `validate_area_for_group_by` の実エラーを載せられず、`.is_err()` で捨てている。同じ場面で `migrate` は実エラーを出すようになった（`warning: skipping <path> — area '2024 Q1' is not allowed: its slug '2024-q1' starts with an issue ID prefix`）ので、2 コマンドで情報量が食い違う。

```
$ renga validate
error: 2024-q1/open/1-q1-task.md: area is not usable as a directory name
```

ユーザーには「予約ステータス名と衝突している」のか「ID プレフィックス形」なのか、どう直せばよいのか（`renga update <ID> --area <別名>`）が分からない。`--auto-correct` でも直らないので、メッセージだけが復旧の手がかりになる。

## 提案

`Finding.message` を `Cow<'static, str>` にして `validate_area_for_group_by` のエラー文をそのまま載せる。判定ロジックを validate 側に複製するのは避ける。

## 出典

レビュー（review エージェント）で発見。

## 解決（2026-07-27）— 問題自体が消滅したため対応不要

この issue は「area の規則が 2 つになったのに `Finding.message` が `&'static str` なのでどちらの違反か言えない」というものだった。

#243 の ID プレフィックス規則を撤回した結果、area の規則は「予約ステータス名との衝突」1 つだけに戻った。静的メッセージ `"area collides with a reserved status directory name"` で曖昧さがないため、`Finding.message` は `&'static str` のまま維持する（`String` 化はライブラリの破壊的変更にあたるので、理由が消えた以上は入れない）。

`migrate` 側は実エラーメッセージ（area 名を含む）を出す形を残している。
