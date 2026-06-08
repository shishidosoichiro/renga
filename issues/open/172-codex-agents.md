---
schema_version: 1
status: open
priority: medium
area: agent
labels: []
---

# Codex/.agents 向けの公開ドキュメントと開発規約を整合させる

AGENTS.md:51 は skills/ を ~/.claude/skills/ または ~/.agents/skills/ にシンボリックリンクして使うと説明しているが、README.md:47-60 と README.ja.md:47-60 は Claude Code integration と ~/.claude/skills の手順だけを案内している。CONTRIBUTING.md:52 と CLAUDE.md も Claude/.claude 前提の記述が残っている。Codex 視点では、配布スキルと agent 指示が Codex/.agents 対応済みなのに公開導線が Claude 専用に見えるため、README/README.ja/CONTRIBUTING/必要なら CLAUDE.md を同期して、Claude Code と Codex の両方で使える導線を明確にする。AGENTS.md や CLAUDE.md を変更する場合は AGENTS.md の self-improve 経由ルールに従う。
