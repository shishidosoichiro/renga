---
schema_version: 1
status: open
priority: low
area: cli
labels: []
---

# complete_update_shows_open_issues_and_flags を2つのテストに分割する

complete_update_shows_open_issues_and_flags は issue 候補の確認とフラグの確認という2つの責務を1テストで検証している。complete_done_shows_open_issues や complete_list_flags が別々になっているパターンと一貫性がなく、将来 update completion の仕様変更時にどちらが壊れたか分かりにくい。complete_update_shows_open_issues と complete_update_flags に分割する。
