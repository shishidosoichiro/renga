---
type: Issue
schema_version: 1
status: open
priority: high
area: agent
labels: [improve, unverified-premise]
---

# improve: unverified-premise

## 対象の retro

open（改善のきっかけ）:
- #175 retro: validate の frontmatter なし判定を実装バグとして誤分類した
- #198 retro: update/edit が done issue を扱えるかの前提

同じ型で close 済み（改善のときに読む）:
- #37
- #268（非公開の参考プロジェクト名を公開物に書いた件。self-improve で個別に対処済み）

## 型の定義

`unverified-premise`: 自分か宍戸さんの前提を、確かめずに結論に使った。資料を読んだか、資料が存在しなかったうえで、解釈や仮定を確かめなかったもの。読めば分かる資料を読まなかった場合は `unread-source` に分類する。

## 進め方

self-improve に渡すのは、宍戸さんが承認してからにする（`.claude/skills/retro/SKILL.md` の Step 4）。

