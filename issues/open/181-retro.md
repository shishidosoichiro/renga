---
type: Issue
schema_version: 1
status: open
priority: medium
area: agent
labels: [retro]
---

# retro: できるかどうかの質問に確認実行で返した

## 観測

宍戸さんの「cargo install renga ができるようになったんだね？」という可否の確認に、はい／いいえで答えず、確認のための実行に進もうとした。

## そのとき何が見えていたか

- AGENTS.md には、相談形の問いを実装の許可として扱わない規則があった（2026-06-07 の 3eb89a2。#164 を受けて追加）
- その規則の例は "what should we do first?"・"how should we proceed?"・"what do you recommend?" という相談形で、可否を問う質問の例は無かった

## 推測

可否を問う質問が、規則の対象と認識されなかった可能性がある。
