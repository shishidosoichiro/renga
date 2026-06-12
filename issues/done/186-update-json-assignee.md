---
schema_version: 1
status: done
priority: low
area: test
labels: []
---

# update --json に assignee を含む統合テストがない

integration.rs の update_from_json_stdin テスト（行 1296）は milestone を含む JSON で更新するが、assignee フィールドは含めていない。assignee を update --json で設定できることの統合テストカバーが不足している。milestone も list_json_includes_milestone, show_json_includes_milestone, create_without_milestone_omits_field, list_filters_by_milestone のテストが存在しないが、assignee ではそれらが揃っている点は良い。
