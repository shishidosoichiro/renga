---
type: Issue
schema_version: 1
status: done
priority: high
area: core
labels: [bug, found_at:0.17.0]
---

# 同じ ID のファイルがあるとき、find_issue が返すファイルがファイルシステムの順序で変わる

## 現象

2026-10-08 の CI（GitHub Actions、run 37761438644）で、`reopen_rejects_collision_within_area_bucket` が失敗した。
- 期待していた動作: `reopen 1` が「already exists」で失敗する
- 実際の動作: 成功し、`issues/open/1-foo.md` を作った

このテストは、7月の追加以来ずっと CI で通っていた。手元の macOS でも通る。

## 原因

`find_issue` は WalkDir を、ファイルシステムが返す順のまま辿る。テストは同じ ID のファイルを `core/done/` と `core/open/` に置く。
- `done` 側が先に見つかった場合: 衝突を検出する
- `open` 側が先に見つかった場合: area の無い open の issue として扱い、`issues/open/` へ移す

ext4 のディレクトリの順序はファイルシステムごとのハッシュ種で決まる。CI のランナーのイメージが更新され、順序が変わった可能性が高い（推測。Linux 上での再現は未確認）。

## 対応

`find_issue` と `collect_issue_files` の WalkDir に `sort_by_file_name()` を付け、どの環境でも同じ順で辿る。同じ ID があるときは `done/` が `open/` より先に見つかる。

再検討の条件: 同じ ID のファイルを作らせない対応（#195）が入ったら、この順序への依存を見直す。

