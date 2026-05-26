---
status: open
priority: medium
area: test
labels: []
---

# fbim init コマンドのテストが存在せずカバレッジ 0%

## 問題

cargo llvm-cov の結果で commands/init.rs のカバレッジが 0%（Regions: 0/13、Functions: 0/1、Lines: 0/9）。

統合テスト（tests/integration.rs）に `fbim init` の実行テストが存在しない。

## 再現

`cargo llvm-cov --summary-only -- --test-threads=1` を実行すると以下が出力される:

`commands/init.rs: 13 regions, 13 missed (0.00%), 1 function, 1 missed (0.00%), 9 lines, 9 missed (0.00%)`

## カバレッジすべき挙動

1. issues/done が存在しない場合: ディレクトリ作成と「Initialized ...」メッセージ
2. issues/done がすでに存在する場合: 「... already initialized」メッセージ

## 関連箇所

- src/commands/init.rs
- tests/integration.rs（テストなし）
