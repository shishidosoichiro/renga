---
status: open
priority: medium
area: cli
labels: []
---

# show の補完が emit_open_issues のみで done issue ID を提示しない

- **ファイル**: `src/commands/completions.rs:110`
- **再現シナリオ**: `fbim show <TAB>` を実行したとき done issue の ID が候補に出ない。`show.rs` は `find_issue(..., include_done=true)` で done issue も表示できるが、補完側が `emit_open_issues`（`only_done: false`）しか呼ばないため done/ 配下のファイルが除外される
