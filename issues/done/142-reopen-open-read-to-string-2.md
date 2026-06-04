---
schema_version: 1
status: done
priority: medium
area: core
labels: []
---

# reopen: 同一ファイルが open/ にある場合に read_to_string を2回呼ぶ

src/commands/reopen.rs の 28-31 行目で path == dest のとき最初の read_to_string を呼んでステータスチェックし、43 行目でさらに read_to_string を再度呼んでいる。
path != dest の場合は 43 行目の 1 回だけだが、path == dest の場合は内容を 2 回読んでいる。
実害は小さいが、最初の read 結果を再利用するか、ロジックを整理して 1 回の読み込みにすべき。
