---
type: Issue
schema_version: 1
status: open
priority: medium
area: cli
labels: []
---

# feat: add --append to renga update for appending text to issue body

`renga update <ID> --append <text|-> ` で issue body の末尾にテキストを追記できるようにする。

## 背景

`--body` は既存 body 全体を置換する（タイトル見出しは保持）。判断記録や作業ログを issue に積み上げたい場合、既存 body を読んでから全文を組み立て直す必要があり手間。追記専用のオプションがあれば、既存 body を読まずに `renga update <ID> --append '...'` するだけで済む。

## 設計

- `update.rs` に `--append <text|->` を追加。`--body` と同じ stdin (`-`) 対応。
- 既存 body の末尾に `\n\n{text}` を追記する。タイトル見出しの扱いは `--body` のロジックを再利用（見出しが無ければ既存見出しを補う必要はない — 既存 body に既に見出しがあるので単純追記でよい）。
- `--body` と `--append` は同時指定不可（相互排他）。
- 独立サブコマンド（`renga append <ID> <text>`）ではなく `update` の兄弟フラグとして実装する。理由: `done`/`pending` は状態遷移で複数 ID を取る設計だが、追記は複数 issue に同じ文言を足す需要がなく、body 編集系（`--body`）と同じコードパスを再利用できるため。
- `--json` 経由の入力にも `append` フィールドとして対応する。

## 参考

宍戸さんとの設計会話（2026-07-14）より起票。

## 追加のユースケース（2026-10-07 追記）

エージェントが作業ログを issue に積み上げる運用では、`--body` の全置換が事故のもとになる。既存の本文を読み落とすと、それまでの記録を消してしまう。

全置換を避けるため、エディタやファイルの直接編集で本文に追記する回避策もある。ところが、hook などで「`renga update --body` が呼ばれたこと」を進捗の印として監視していると、その回避策は検出されない。追記の手段があれば、こうした迂回をしなくて済む。

