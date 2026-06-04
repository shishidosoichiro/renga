---
schema_version: 1
status: open
priority: medium
area: docs
labels: []
---

# CHANGELOG.md が v0.8.0 以降のコミットを反映していない

v0.8.0 タグ以降の以下の変更が CHANGELOG.md に記載されていない。次のリリース前に git-cliff で再生成が必要。
- feat(core)!: reorganize issues into per-status directories (4a1db66)
- feat(cli): improve renga update with title arg, label ops, and body safety (8fd8294)
- fix(cli): update --status now moves file to matching status directory (bf9a89d)
- fix(core): fix init idempotency check to cover all five status dirs (71d125d)
- fix(core): replace expect() with anyhow context in migrate.rs (9934144)

なお issue #133 (migrate コマンドのみ) と部分的に重複する。
