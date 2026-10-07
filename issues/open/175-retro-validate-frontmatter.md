---
type: Issue
schema_version: 1
status: open
priority: medium
area: agent
labels: [retro, unverified-premise]
---

# retro: validate の frontmatter なし判定を実装バグとして誤分類した

## 観測

#170 で、`validate` が frontmatter の無いファイルをエラーにする挙動を、実装バグとして起票した。宍戸さんに「エラーが妥当だと思う」と指摘された。

## そのとき何が見えていたか

- spec.md の「frontmatter is optional / unknown」という文言を読み、`validate` の要件と解釈した
- `validate` の役割（壊れた issue を検出して exit 1 にする）と、通常の読み取りでの許容との違いは整理していなかった

## 推測

仕様の文言がどのコマンドの振る舞いを定めたものかを確かめずに、結論に使った可能性がある。
