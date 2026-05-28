---
status: done
priority: low
area: test
labels: []
---

# readme.rs のカバレッジが低い（62.77%）

## 問題

cargo llvm-cov の結果で readme.rs のカバレッジが 62.77%（Lines: 94行中 59行のみカバー）。

関数カバレッジは 37.50%（8関数中 3関数のみ）。

## 未テストの主なパス

- `generate()` 関数の area_order を使ったグルーピングパス
- `generate()` のグループなし issue（area が空文字）の表示パス
- `write_readme()` の実際のファイル書き込みパス（統合テスト経由では間接的にカバーされているが直接テストがない）

## 関連箇所

- src/readme.rs（カバレッジ: Functions 37.50%、Lines 62.77%）
- tests/integration.rs（readme.rs の直接テストなし）
