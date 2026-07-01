---
schema_version: 1
status: done
priority: medium
area: core
labels: []
---

# done issue に対して update / edit を実行すると not found になる

## 問題

`update`/`edit` はどちらも `find_active_issue`（`src/issue.rs`）を使っており、
`done/` に置かれた issue は frontmatter とディレクトリが一致している限り
「not found」になる。ラベル・担当者・本文の修正だけしたくても
`reopen` → 編集 → `done` の3手順が必要で、その間ステータスが一時的に
`open` に戻ってしまう（#198 の retro で意図的に切り離した論点）。

## 比較

GitHub Issues / GitLab / Jira はいずれも「closed でもラベル・本文・担当者は
編集可能、ステータス遷移だけ reopen が要る」という設計。

## 対応案

- `update`/`edit` を `find_issue(&ctx.issues_dir, id, true)` ベースに変更し、
  `done/` の issue も対象にする
- `update --status` の許容値（現状 `open`/`pending`/`in-progress` のみ）は
  そのまま維持し、`done` への遷移は引き続き `renga done` 専用にする
- 監査性は git 履歴が担保するので追加の仕組みは不要と考える

