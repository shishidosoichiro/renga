---
schema_version: 1
status: open
priority: medium
area: agent
labels: [retro]
---

# retro: status authoritative source の既存判断を確認せず推奨した

`status` の authoritative source を推薦する前に、`spec.md`、README の authoritative spec 記述、
per-status directory 導入履歴、`update --status` の既存挙動を確認していなかった。

今後、既存設計に関わる推薦では、先に spec/ADR 相当/過去 issue/git history/実装を確認し、
明文化が弱い場合は「未決定」として扱う。今回の status は frontmatter を正、
ディレクトリは同期対象として見るのが現行設計に合う。

self-improve 相当のサブエージェントレビューでも同じ結論だった。
