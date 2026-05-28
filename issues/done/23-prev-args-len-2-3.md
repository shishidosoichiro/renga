---
status: done
priority: medium
area: cli
labels: []
---

# 補完の prev を args[len-2] で計算するため引数3トークン以上でフラグ直前値を取得できない

- **ファイル**: `src/commands/completions.rs:107`
- **再現シナリオ**: `fbim list --status open <TAB>` のとき args は `["fbim", "list", "--status", "open", ""]`（5要素）で `args.len()-2=3` → `prev="open"` になり `"--status"` ブランチにマッチせず、ステータス値候補ではなくフラグ一覧が表示される
