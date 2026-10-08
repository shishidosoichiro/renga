---
type: Issue
schema_version: 1
status: open
priority: medium
area: agent
labels: []
---

# Codex 用の custom agent 定義を .codex/agents/ に置き、.claude/agents/ の定義を参照させる


## 背景

#282 でエージェント向けの指示を AGENTS.md に一本化した。AGENTS.md はサブエージェントの役割と定義ファイル（`.claude/agents/<name>.md`）を表で示している。

Codex は 2026年3月にサブエージェントを正式に提供し、custom agent を TOML で定義できる。このリポジトリの `.codex/` には custom agent の定義がまだ無い（#201 も参照）。

## やること

- `.codex/agents/*.toml` に review などの custom agent を定義する。手順の本文は `.claude/agents/*.md` を参照させ、二重に書かない
- 定義したら AGENTS.md の「## Codex」節を更新する

#282 の範囲外として切り出した。
