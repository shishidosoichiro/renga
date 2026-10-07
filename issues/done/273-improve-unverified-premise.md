---
type: Issue
schema_version: 1
status: done
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


## 結論: 指示ファイルは変えない（2026-10-07、self-improve）

### 根拠

- #198（反証を出さずに同意した）には、既存のルールが効かなかった。「迎合しない」は CLAUDE.md に 2026-05-26（c916c1a）から、AGENTS.md に 2026-06-06（276a843）からあり、#198（2026-06-18）より前である。同じ趣旨の文を足しても効かない
- #175 に直接当たる既存のルールはない。ただし誤りの元になった資料は直っている。spec.md・spec.ja.md の frontmatter 節に、通常の読み取りと validate の違いが明記された（#170 の方針、10ccbe3）
- 機械では判定できない。「前提を確かめたか」を hook・テスト・CLI で検出する手段はない
- 知識を要る工程が特定できない。#175 は issue 起票時の仕様の解釈、#198 は方針への同意で、共通の工程がない。#198 は判断材料の記録が残っておらず、置き場所を決める観測が足りない。起票の工程に一文を置く案は #175 の1件だけが根拠になるので見送る
- #198 で混ざった2つの論点は #193・#213 に分かれ、どちらも done
- #37 の該当項目（スキル名の誤記）は、c916c1a の時点で CLAUDE.md が正しい名前（`Agent(subagent_type="review")`）になっている。#268 は `.claude/rules/issue-management.md` の「判断記録の出典」と `/commit` スキルで個別に対処済み（6fba6ab）
- この型の retro は #198 から #268 まで約3.6か月起きておらず、#268 は性質が別（公開物の出典）である

### 再検討の条件

次の unverified-premise の retro が起きたとき、「そのとき何が見えていたか」に、読んでいた資料と工程（起票・レビュー・方針への同意など）が記録されていれば、その工程のスキルか `paths` 付きの rules に置き場所を決めて再検討する。同じ工程で2件そろった場合を優先する。
