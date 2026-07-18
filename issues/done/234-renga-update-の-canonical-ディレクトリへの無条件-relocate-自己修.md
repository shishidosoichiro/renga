---
schema_version: 1
status: done
priority: medium
area: docs
labels: []
---

# renga update の canonical ディレクトリへの無条件 relocate（自己修復）副作用が spec に未記載

## 問題

今回の diff で `src/commands/update.rs` の挙動が「`--status` 指定時だけ移動」から「フィールド編集後に無条件で canonical (area, status) ディレクトリへ relocate する」に変更された（misplaced issue の自己修復を兼ねる、意図的な変更）。

`tests/integration.rs` の `update_operates_on_misplaced_active_issue_with_warning` で明示的にテストされている通り、`renga update 1 --assignee alice` のように area/status を一切変更しない編集でも、issue が `find_editable_issue` の「recoverable mismatch」経路で見つかった場合はファイルが別ディレクトリへ静かに移動する。

この副作用が `spec.md`/`spec.ja.md`/`README.md` のどこにも記載されていない。`spec.md` には次の一文があるのみ:

> Changing `area` or `status` (via `update`, `done`, `pending`, `in-progress`, or `reopen`) automatically relocates the file to the correct directory.

これは「area/status を変更した場合」の relocate だけを説明しており、「area/status を変更しなくても、misplaced issue なら relocate される」という今回追加された自己修復の挙動には触れていない。

## 期待される対応

spec.md / spec.ja.md の `update` または `.renga.yml` 節に、update が編集の副作用として misplaced issue を canonical ディレクトリへ自己修復することを明記する。
