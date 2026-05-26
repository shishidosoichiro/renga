---
status: open
priority: medium
area: core
labels: []
---

# make_slug が trim 後に 30 バイト切り捨てするため切り捨て位置が '-' のとき末尾ダッシュのファイル名が生成される

- **ファイル**: `src/issue.rs:344`
- **再現シナリオ**: タイトル `"aaaaaaaaaaaaaaaaaaaaaaaaaaa bb"` など 29文字の単語＋スペース＋続きになると slug が `"aaaaaaaaaaaaaaaaaaaaaaaaaaa-bb"` になり、30バイト切り捨てで `"aaaaaaaaaaaaaaaaaaaaaaaaaaa-"` という末尾ダッシュのファイル名が生成される
