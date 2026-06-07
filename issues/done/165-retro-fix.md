---
schema_version: 1
status: done
priority: medium
area: agent
labels: [retro]
---

# retro: テスト追加コミットを fix と誤分類した

issue 152 は実装コードを変えず tests/integration.rs の未カバーパスを追加しただけだったが、found_at ラベルの『以前からあったバグは fix』規約を強く読みすぎて fix: cover dynamic completion candidates でコミットした。Conventional Commits では test: がテスト追加/修正、fix: がバグ修正なので、純テスト変更では test: を優先する。今後はコミット前に変更対象と目的を照合し、src の挙動修正がないテストカバレッジ追加は test: を使う。
