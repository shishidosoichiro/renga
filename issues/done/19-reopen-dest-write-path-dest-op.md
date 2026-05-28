---
status: done
priority: medium
area: core
labels: []
---

# reopen が dest への write を path!=dest チェック前に実行し同名 open issue を無警告上書きする

- **ファイル**: `src/commands/reopen.rs:25`
- **再現シナリオ**: `issues/5-foo.md`（open）と `done/5-foo.md` が共存しているとき `fbim reopen 5` を実行すると、`fs::write(&dest, &updated)` が `path != dest` チェックより先に走り、open 側のファイルが done 側の内容で黙って上書きされてデータが失われる
