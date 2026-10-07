---
type: Issue
schema_version: 1
status: done
priority: low
area: agent
labels: []
---

# Codex の設定から非公開プロジェクトのドメイン許可を外す

## 問題

`.codex/config.toml` の 81〜83行目は、Codex のネットワーク許可に、非公開の参考プロジェクトの仮ドメインを加えている。この設定はすでに push 済みである。

Renga の開発で、このドメインへアクセスすることは無い。また、公開リポジトリに非公開プロジェクトの名前を書かないという規則（`.claude/rules/issue-management.md` の「判断記録の出典」、retro #268）にも反する。

## やること

コメント行を含む該当の3行を削除する。`.codex/` の変更なので、self-improve を通す。

