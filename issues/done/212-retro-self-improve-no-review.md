---
schema_version: 1
status: done
priority: medium
area: agent
labels: [retro]
---

# retro: self-improve の変更にレビューが入っていない

self-improve エージェントが .claude/agents/ や CLAUDE.md を変更した後、その変更を新鮮なコンテキストでレビューするステップが存在しない。

## 方針

「レビューは新鮮なコンテキストで行う」という原則に従い、self-improve の変更を専用の reviewer subagent に委ねる構造にする。

- 既存の `review.md` は Rust コード専用（clippy・test・仕様整合性）で `.claude/` 変更には不適
- `.claude/agents/` 変更向けの専用 reviewer subagent を新規作成する
- self-improve.md の Step 8 でこの reviewer subagent を呼ぶ

## 必要な変更

1. `.claude/agents/agent-config-reviewer.md` を新規作成
   - レビュー観点: 根拠の有無、推測的変更がないか、他ファイルとの整合性、削除の妥当性、記述の明確さ
2. `self-improve.md` に Step 8 を追加して agent-config-reviewer を呼ぶ
