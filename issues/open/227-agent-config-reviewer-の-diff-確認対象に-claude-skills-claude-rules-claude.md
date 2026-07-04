---
schema_version: 1
status: open
priority: medium
area: agent
labels: []
---

# agent-config-reviewer の diff 確認対象に .claude/skills・.claude/rules・.claude/hooks を含める

retro #226 の実装レビュー時に agent-config-reviewer 自身が指摘した積み残し。

現状の `.claude/agents/agent-config-reviewer.md` の git diff 確認対象に `.claude/skills/`・`.claude/rules/`・`.claude/hooks/` が含まれていない。#226 でこれらのディレクトリが新設されたため、self-improve の変更をレビューする際の対象パスに追加する必要がある。

対応時は retro issue を起票し self-improve 経由で変更すること（/retro スキル参照）。

