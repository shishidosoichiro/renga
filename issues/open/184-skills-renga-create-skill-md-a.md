---
schema_version: 1
status: open
priority: low
area: cli
labels: []
---

# skills/renga-create/SKILL.md の argument-hint に --milestone と --assignee が欠落している

renga-create/SKILL.md の argument-hint は '<title> [--slug <slug>] [--priority high|medium|low] [--area <area>] [--body <text>] | --json' のまま。今回の assignee 実装で --assignee を追加したが argument-hint には反映されていない。--milestone も同様に欠落している。spec.md のコマンド一覧では両フィールドとも列挙されている。
