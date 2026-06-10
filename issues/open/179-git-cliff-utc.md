---
schema_version: 1
status: open
priority: medium
area: docs
labels: []
---

# git-cliff のリリース日が UTC 基準になる

git cliff --tag v0.12.0 -o CHANGELOG.md を JST の 2026-06-10 に実行しても、CHANGELOG の日付が UTC 基準の 2026-06-09 になった。手書き修正は禁止しているため、リリース後に cliff.toml の date filter に timezone を指定するか、UTC 日付を正式運用にするかを検討する。
