---
schema_version: 1
status: done
priority: high
area: agent
labels: [retro]
---

# retro: レビュー指摘 issue に found_at/fixed_at/found_in_impl ラベルを付けるフローを追加

## 目的

レビューで見つかったバグが「今の実装で入ったバグ」か「以前から存在していたバグ」かを
issue のラベルで明示し、コミット戦略の判断基準を明確にする。

## ラベル規約

- `found_in_impl` — 今の実装サイクルで入ったバグ（feature コミットに含めて直す、CHANGELOG に出さない）
- `found_at:X.Y.Z` — 指定バージョンから存在していたバグ（別 `fix:` コミット、CHANGELOG に出す）
- `fixed_at:X.Y.Z` — 修正されたバージョン（クローズ時に付ける）

コロン（`:`）は renga の label として使用可能であることを確認済み。

## CLAUDE.md への反映

実装フロー（retro #143 で更新済み）のステップ 4「レビュー指摘を分類する」に以下を追加：

- レビューで見つかった問題を issue 化するとき、必ず上記ラベルを付ける
- `found_in_impl` → コミット前に修正、feature コミットに含める
- `found_at:X.Y.Z` → コミット後に別の `fix:` コミット、修正時に `fixed_at:X.Y.Z` を付ける

