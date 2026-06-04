---
schema_version: 1
status: done
priority: high
area: agent
labels: [retro]
---

# retro: issue管理ルールを .claude/rules/issue-management.md に切り出してCLAUDE.mdを軽量化

## 問題

CLAUDE.md の「Issue 管理」セクションに area テーブル・バグラベル規約など詳細ルールが蓄積し、
すべてのセッションで不要なコンテキストを消費している。

## 解決策

Claude Code の `.claude/rules/` 機能を使い、`issues/**/*.md` を触るときだけ読み込まれる
条件付きルールファイルを作成する。

## やること

1. `.claude/rules/issue-management.md` を新規作成：
   ```
   ---
   paths:
     - "issues/**/*.md"
   ---
   ```
   以下の内容を移動する：
   - area テーブル（cli/core/config/test/docs/ci/agent/misc）
   - バグラベル規約（found_in_impl / found_at:X.Y.Z / fixed_at:X.Y.Z）
   - retro issue の起票ルール

2. CLAUDE.md の「Issue 管理」セクションを簡潔なポインタに縮小する

