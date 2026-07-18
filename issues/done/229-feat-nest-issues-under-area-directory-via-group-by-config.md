---
schema_version: 1
status: done
priority: medium
area: core
labels: []
---

# feat: nest issues under area directory via group_by config

`.renga.yml` に opt-in の `group_by: [area]` を追加し、`issues/<area>/<status>/...` という物理レイアウトを選択可能にする。デフォルト（`group_by` 未設定）は現状と完全に同一で、既存ユーザーへの影響はない。

## 背景

複数プロジェクトの issue を1つの renga リポジトリで area 分けして管理する場合、集中したいときに他プロジェクトの issue がファイルシステム上・CLI 上のノイズになる。宍戸さんとの設計会話（2026-07-14〜18）の結論。

## スコープ

- `group_by: [area]` のみ実装。設定は list 型（将来の複数軸拡張に備える）だが v1 では要素数1・値は `"area"` 固定
- `label:<prefix>` 方式は実装しない（ラベル仕様には触れない、という明示的な決定）
- area は既存の `make_slug` でスラグ化。値なし issue は今まで通り `issues/<status>/` 直下

## 設計上の発見

`issues/<status>/` への書き込み・移動ロジックが update.rs/pending.rs/done.rs/in_progress.rs/validate.rs に5箇所独立して重複していた。group_by 導入にあたり共有ヘルパー（`relocate_issue`/`canonical_status_dir`）に統合する。同時に "最初の1階層 = status" と決め打ちしている既存バグ（find_issue の done 除外 ×2、completions.rs の同パターン、status_dir_name の重複実装）も同じ修正でまとめて直す。

## 実装順序

1. 純粋リファクタ（relocate_issue/status_dir_name 統合、done 除外バグ修正）— 挙動変化なし
2. group_by 本体（config・canonical_dir・各コマンドの area 対応・migrate/validate）

詳細設計は plan ファイル参照: ~/.claude/plans/hazy-cooking-quail.md
