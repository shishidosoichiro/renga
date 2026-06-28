---
schema_version: 1
status: done
priority: high
area: agent
labels: [retro]
---

# リリース前 fixup フローを renga の規律に反映する

## 背景

kiwi/agents/CLAUDE.md（retro #381）にある「リリース前 fixup フロー」が renga リポジトリの CLAUDE.md・.claude・CONTRIBUTING.md に反映されていない。

## kiwi/agents 側の方針

- feat / fix / fixup の使い分け表（開発中＝前タグ以降に追加したコードへの手直しは fixup!、リリース済みコードのバグ修正は fix:）
- リリース前 fixup フロー: git commit --fixup <SHA> で積み、タグ前に git rebase -i --autosquash で統合
- push 済みコミットには autosquash しない（force push 回避）
- 前タグより前から存在するバグは通常の fix: のまま

## renga 側のギャップ

CLAUDE.md 実装フローは「今回の実装で入ったバグ→feature コミット」「以前から存在→別 fix: コミット」しか扱っておらず、『前タグ以降に追加したが別コミットのコードへの手直し』が fix:（未リリースなのに CHANGELOG 掲載）に倒れる。

## やること

self-improve 経由で、kiwi/agents の fixup 方針を renga の実績・用語に合わせて CLAUDE.md（実装フロー / コミット粒度）と CONTRIBUTING.md（Commit messages / Releasing）に転記する。英語版ドキュメント（CONTRIBUTING.md）と日本語（CLAUDE.md）の整合を取る。
