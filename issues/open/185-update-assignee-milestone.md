---
schema_version: 1
status: open
priority: medium
area: cli
labels: []
---

# update --assignee でアサイニーをクリアする方法がない（milestone も同様）

renga update --assignee <name> で assignee をセットできるが、一度セットした assignee を削除する手段がない。milestone も同様。spec.md にもこのユースケースへの言及がない。解決案: --assignee '' を空文字で渡したときにフィールドを削除する、または --clear-assignee フラグを追加する。milestone も同じ問題がある（既存 issue の可能性あり）。
