---
schema_version: 1
status: done
priority: medium
area: agent
labels: [retro]
---


# retro: AGENTS.md への運用ルール移植漏れを修正する

CLAUDE.md には review サブエージェント、self-improve、Plan 条件、レビュー時 coverage、バグラベル規約などの運用ルールがあるが、AGENTS.md への移植が不完全だった。Codex 向けに移植可能な内容を AGENTS.md に反映し、Claude Code 固有構文は Codex の subagent/custom agent 表現に置き換える。
