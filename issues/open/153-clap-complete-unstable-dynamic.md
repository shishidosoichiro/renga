---
schema_version: 1
status: open
priority: low
area: cli
labels: []
---

# clap_complete unstable-dynamic が stable になったら __complete と手書きシェルスクリプトを置き換えを検討する

`clap_complete` の `unstable-dynamic` feature（v4.6.5 時点）には以下の API がある:

- `CompleteEnv` — 環境変数 `COMPLETE=$SHELL` で補完モードを自動トリガー
- `ValueCompleter` trait — `fn complete(&self, current: &OsStr) -> Vec<CompletionCandidate>` で動的候補を返せる
- `ArgValueCompleter` — 引数に `ValueCompleter` を付与するラッパー
- `CompletionCandidate::new("open").help("Active issue")` — 説明付き候補

stable になれば `__complete` サブコマンドと手書きの ZSH/BASH/FISH スクリプトをまるごと削除できる見込み。

`PossibleValue::new("high").help("High priority")` の定義はそのまま活きる。

context-aware 補完（前の引数値を参照）は issue #5784 で未解決だが、Renga の用途では影響なし。

