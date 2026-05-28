---
status: done
priority: medium
area: core
labels: []
---

# create がファイル名衝突チェックなしで fs::write し既存 issue を無警告上書きする

- **ファイル**: `src/commands/create.rs:35`（`fs::write(&path, &content)`）
- **再現シナリオ**: 2プロセスが同時に `fbim create` を実行して同じ next_id を計算した場合、または手動で同名ファイルが置かれている場合、後から書いた方が先のファイルを黙って上書きする。エラーも警告も出ない
