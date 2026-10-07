---
type: Issue
schema_version: 1
status: open
priority: medium
area: agent
labels: [retro, unread-source]
---

# retro: status authoritative source の既存判断を確認せず推奨した

## 観測

`status` の authoritative source を推奨する前に、`spec.md`、README の authoritative spec の記述、ディレクトリ別配置の導入履歴、`update --status` の既存挙動を確認していなかった。推奨の後、self-improve 相当のサブエージェントのレビューで、frontmatter を正とする結論になった。この結論は現在 `spec.md` の status の節に書かれている。

## そのとき何が見えていたか

- 既存の設計判断が spec・過去 issue・git 履歴にあるかは確かめていなかった

## 推測

既存設計に関わる判断を、新しい設計判断として扱った可能性がある。
