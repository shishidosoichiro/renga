---
schema_version: 1
status: open
priority: medium
area: agent
labels: [retro]
---

# retro: validate の frontmatter なし判定を実装バグとして誤分類した

ユーザーから #170 について『エラーが妥当だと思う』と指摘された。レビュー時に spec.md の『frontmatter is optional / unknown』をそのまま実装要件と解釈し、validate コマンドの役割（壊れた issue を検出して exit 1 にする）との違いを整理せず、実装バグとして起票したのが誤分類だった。改善: validate のような検査コマンドでは、通常の読み取り許容と検査時の異常検出を分けて判断し、挙動変更を提案する前に仕様文言・ユーザー期待・コマンド目的のどれが問題かを切り分ける。self-improve 専用サブエージェントはこの環境で利用可能な明示ツールとして見つからないため、この retro issue に改善案を残す。
