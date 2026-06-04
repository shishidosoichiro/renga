---
schema_version: 1
status: done
priority: medium
area: test
labels: []
---

# migrate テスト: 移動先ファイルが既存の場合のテストがない

tests/integration.rs に migrate コマンドのテストは追加されているが、移動先に同名ファイルが既存する衝突ケースのテストがない。
issue #138 の修正と合わせて、衝突時の正しいエラー動作を検証するテストを追加する必要がある。
