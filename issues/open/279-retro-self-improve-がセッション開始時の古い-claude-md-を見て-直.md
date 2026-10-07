---
type: Issue
schema_version: 1
status: open
priority: medium
area: agent
labels: [retro, unverified-premise]
---

# retro: self-improve がセッション開始時の古い CLAUDE.md を見て、直っている指示を古いと判断した

## 観測

2026-10-07、#273 を処理していた self-improve が、#279 を起票した。内容は「CLAUDE.md の retro の手順が古いまま、記録専用の /retro スキルと食い違っている」というもの。

ところが、ディスク上の CLAUDE.md は 701c31f ですでに直っていた。agent-config-reviewer がこれを指摘し、#279 は取り消された。

## そのとき何が見えていたか

- **読んでいたもの**: セッション開始時にコンテキストへ読み込まれた CLAUDE.md。701c31f より前の内容だった。701c31f は、同じセッションの中で別のサブエージェントがコミットしたもの
- **読んでいなかったもの**: ディスク上の CLAUDE.md の現在の内容と、`git log -- CLAUDE.md`

工程は「改善工程の中で、指示ファイルの現状を根拠にして issue を起票する」場面である。

## 推測

長いセッションでは、別のエージェントが指示ファイルを書き換えることがある。そのため、開始時に読み込まれた CLAUDE.md・AGENTS.md は、途中で古くなりうる。指示ファイルの現状を根拠にするときに、ディスクを読み直す手順が無かった。

