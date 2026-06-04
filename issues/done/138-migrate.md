---
schema_version: 1
status: done
priority: medium
area: core
labels: []
---

# migrate: 移動先ファイルが存在する場合の衝突チェックがない

src/commands/migrate.rs の 52 行目 std::fs::rename(path, &dest) は dest が既に存在する場合に無条件で上書きする（Linux の rename(2) 挙動）。
別ステータスディレクトリに同名ファイルが先に存在する場合にデータ消失の危険がある。
移動前に dest.exists() を確認して bail! するか、ユーザーに選択させるべき。
