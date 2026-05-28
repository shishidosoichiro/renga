---
status: done
priority: medium
area: cli
labels: []
---

# reopen の補完が emit_done_issues のみで pending issue を提示しない

- **ファイル**: `src/commands/completions.rs:111`
- **再現シナリオ**: `fbim reopen <TAB>` を実行したとき `done/` 配下の issue しか候補に出ない。`reopen.rs` は `find_issue(..., include_done=true)` で pending issue（`issues/` 直下）も受け付けるが、補完側が `emit_done_issues`（`only_done: true`）しか呼ばないため pending が除外される
