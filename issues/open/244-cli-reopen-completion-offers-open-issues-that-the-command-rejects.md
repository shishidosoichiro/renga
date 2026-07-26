---
schema_version: 1
status: open
priority: low
area: cli
labels: []
---

# cli: reopen completion offers open issues that the command rejects

`renga reopen <TAB>` が open な issue も候補に出すが、`reopen` はそれらを拒否する。

```sh
renga create "Open One"
renga __complete renga reopen ""
# 1	Open One      ← 候補に出る
renga reopen 1
# error: issue 1 already exists as an open issue   (exit 1)
```

## 原因

`src/commands/completions.rs` の dispatch で `show` と `reopen` が同じ match arm を共有しており、両方 open + done を連結出力している。

```rust
"show" | "reopen" => {
    emit_open_issues(&mut out, ctx)?;
    emit_done_issues(&mut out, ctx)?;
}
```

`show` は全 issue を対象にするので正しいが、`reopen` は done issue のみを受け付ける。

## 修正案

`reopen` を独立した arm にして `emit_done_issues` だけを呼ぶ。

## 関連

- #240 で completions を `all_issues` ベースに書き換えた際に発見。この書き換えによる退行ではなく、arm 共有は以前から。#240 で `done` 側は「frontmatter が done の issue を候補から外す」ようになったので、`reopen` だけ逆方向の不整合が残っている状態
- `src/commands/completions.rs`（dispatch の `"show" | "reopen"` arm）

