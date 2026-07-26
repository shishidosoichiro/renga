---
schema_version: 1
status: done
priority: low
area: core
labels: []
---

# test: the id-prefixed area tests start from the flat layout, so they never exercise auto-correct

## 症状

`tests/integration.rs` の

- `validate_flags_id_prefixed_area_without_correcting`
- `migrate_skips_id_prefixed_area_with_warning`

はどちらも `issues/open/1-task.md`（= 既にフラット配置）にファイルを置いてから実行し、最後に `assert!(dir.path().join("issues/open/1-task.md").exists())` を確認している。ファイルは最初からそこにあるので、この assert は「動かさなかったこと」を実証していない。

守りたい不変条件は「`validate --auto-correct` が既存の `issues/2024-q1/open/1-task.md` を勝手にフラットへ動かさない」ことなので、テストは **area ディレクトリ配下** から始める必要がある。同じファイル内に `id_prefixed_area_layout(&dir)` ヘルパーが既にあり、まさにその配置を作る。

手動では正しい挙動を確認済み（`group_by: [area]` 有効時、`validate --auto-correct` は area エラーを報告して `issues/2024-q1/open/1-task.md` をそのまま残す）。テストが無いだけで、`canonical_status_dir` のフォールバックが `validate` の Err 分岐より先に効くようなリファクタが入ると無言で退行する。

## その他のテストギャップ

- `update`（`--area` なし）が使えない area の issue をフラットへ再配置する経路のテストが無い。`done` のみカバーされている
- `reopen`・`pending`・`in-progress` も同様（共有ヘルパー経由なので優先度は低い）
- ID プレフィックス付き area ディレクトリの配下に dir-based issue がある場合の `next_id` のテストが無い

