---
type: Issue
schema_version: 1
status: open
priority: medium
area: agent
labels: [retro]
---

# retro: 全体レビューで unwrap 観点の確認結果を報告し漏らした

## 観測

全体レビューの報告に、本番コードの `unwrap()` / `expect()` の有無と判断結果を含めなかった。宍戸さんに「前に unwrap() が気になるとか言っていなかったか」と指摘された。

## そのとき何が見えていたか

- AGENTS.md のエラーハンドリング規約に「`unwrap()` / `expect()` はテストコード以外で使わない」があった
- `.claude/agents/review.md` のチェック項目にも同じ観点があった（2026-05-26 の f1aaa3f から）。ただし起票した環境で review.md が読み込まれていたかは分からない
- 起票した環境では、self-improve 相当のサブエージェントが見つからなかった

## 推測

確認項目は少なくとも AGENTS.md にあったが、全体レビューの報告項目として扱われなかった可能性がある。
