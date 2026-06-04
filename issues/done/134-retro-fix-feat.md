---
schema_version: 1
status: done
priority: high
area: agent
labels: [retro]
---

# retro: fix と feat を分けずにコミットする習慣の改善

## 起きたこと

- レビューで指摘された bugfix（`expect()` 除去、`init` 冪等チェック、`update --status` のファイル移動）をfeatureコミットに混ぜてしまった
- 「ついでに直す」習慣が抜けていない
- 破壊的変更を複数まとめようとしていた

## 参照

- https://qiita.com/uno_ha07/items/5820d195510861b5be71（CLAUDE.md ベストプラクティス）
- 核心：「自己改善ループ」— ミスを記録して指示に反映し、同じミスを繰り返さない

## 改善したいこと

1. **コミット粒度の規律**: `feat:` と `fix:` を混ぜない。レビュー指摘の修正は必ず別の `fix:` コミットにする
2. **「ついでに修正」の禁止**: 本来のタスクと無関係な修正は別 issue → 別コミットにする
3. **破壊的変更の分離**: 複数の breaking change は別々のコミットに入れる
