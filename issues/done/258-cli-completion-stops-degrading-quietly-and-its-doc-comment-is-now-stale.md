---
schema_version: 1
status: done
priority: low
area: cli
labels: []
---

# cli: completion stops degrading quietly and its doc comment is now stale

## 症状

`src/commands/completions.rs:280` のコメントは

> Completion degrades quietly: a TAB press is no place to surface an error.

と書いているが、#252 で `all_issues` が stderr に warning を出すようになったため、補完中に warning が出るようになった。zsh/bash/fish のスクリプトは `2>/dev/null` しているので実害は無いが、コメントの主張と実装が食い違っている。

さらに `show` の補完は `emit_open_issues` と `emit_done_issues` を続けて呼ぶため `all_issues` が 2 回走り、**同じ warning が 2 回**出る（実測）:

```
$ renga __complete renga show ""
warning: cannot read .../issues/open/2-broken/README.md: Is a directory (os error 21)
1	Ok
2	2-broken
warning: cannot read .../issues/open/2-broken/README.md: Is a directory (os error 21)
```

## もう一点: doc コメントの陳腐化

`emit_issues` の doc コメント（`src/commands/completions.rs:269-271`）:

> `emit_open_issues` and `emit_done_issues` partition the issues exactly, so `show`/`reopen` can concatenate both without duplicates.

`reopen` は `emit_done_issues` のみを呼ぶよう変更されたので、`reopen` は concatenate しない。記述を `show` だけに直す。

## 検討する方向

- `show` は `emit_issues(out, ctx, |_| true)`（= `emit_all_issues`）1 回で済む。open/done を厳密に分割しているなら結果は同じ
- 補完経路では warning を抑止する（`all_issues` に quiet フラグを足すか、warning の出力先を呼び出し側に委ねる）

