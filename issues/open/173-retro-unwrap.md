---
schema_version: 1
status: open
priority: medium
area: agent
labels: [retro]
---

# retro: 全体レビューで unwrap 観点の確認結果を報告し漏らした

ユーザーから『前に unwrap() が気になるとか言っていなかったか』と指摘された。全体レビュー時に unwrap/expect の実装規約違反を明示的なチェック項目として扱わず、既存コード内の unwrap() の有無と判断結果を報告しなかった。改善: レビュー時は AGENTS.md のエラーハンドリング規約（本番コードの unwrap/expect 禁止）をチェックリスト化し、例外扱いにする場合も理由を報告する。self-improve 専用サブエージェントはこの環境で利用可能な明示ツールとして見つからないため、この retro issue に改善案を残す。
