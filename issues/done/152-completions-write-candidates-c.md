---
schema_version: 1
status: done
priority: medium
area: cli
labels: [found_at:0.9.1]
---

# completions: write_candidates の未カバーパスが多い（グローバルフラグ・completions サブコマンド・create|update の emit_subcmd_flags・update + issue ID）

write_candidates の以下のパスが統合テスト未カバー（カバレッジ 53.65% の主因）:\n- args.len() <= 2 かつ subcmd が '-' で始まる（グローバルフラグ補完）\n- 'completions' サブコマンドのシェル名列挙\n- 'create' で前のトークンが '--' で始まらない場合（emit_subcmd_flags の create 向け）\n- 'update' で前のトークンが '--' で始まらない場合（emit_open_issues + emit_subcmd_flags）\n- 'list' で prev が '--status' 以外の場合（emit_subcmd_flags の list 向け）
