---
status: done
priority: medium
area: core
labels: []
---

# タイトル抽出でファイルステムへのフォールバックが未実装（spec と不一致）

## 問題

spec.md 68行目には「Falls back to the file stem if no heading is found」と明記されているが、実装（src/issue.rs の extract_title 関数）はヘッダーが見つからない場合に空文字列を返す。

Issue::parse（src/issue.rs:183）では extract_title(body) の結果をそのまま使用しており、ファイルステムへのフォールバックは行われていない。

## 再現シナリオ

ヘッダー（# ...）がないファイルを issues/ に配置した場合:
1. spec 上は ファイルステム（例: 1-my-issue から "1-my-issue"）がタイトルとして使われるべき
2. 実際は title が空文字列になる

fbim list / fbim show でタイトルが空で表示される。

## 関連箇所

- spec.md:68 「Falls back to the file stem if no heading is found」
- src/issue.rs:411-419 extract_title 関数（空文字列を返す）
- src/issue.rs:183 Issue::parse でフォールバックなし
