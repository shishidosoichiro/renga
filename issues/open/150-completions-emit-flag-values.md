---
schema_version: 1
status: open
priority: medium
area: cli
labels: [found_in_impl]
---

# completions: emit_flag_values の説明付き出力に統合テストがない

今回追加した emit_flag_values の説明付き出力パス（val.get_help() が Some のとき NAME\tDESCRIPTION を出力する行）を直接実行する統合テストがない。__complete renga create --priority "" および __complete renga update --priority ""、__complete renga update --status "" のテストを追加すべき。カバレッジレポートで completions.rs が 53.65% に留まる主因の一つ。
