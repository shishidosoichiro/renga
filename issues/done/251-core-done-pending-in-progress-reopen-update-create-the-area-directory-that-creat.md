---
schema_version: 1
status: done
priority: high
area: core
labels: [found_in_impl]
---

# core: done/pending/in-progress/reopen/update create the area directory that create rejects

## 背景

#243 の修正で `create` と `update --area` は ID プレフィックス形の area（`2024 Q1` → `2024-q1`）を拒否するようになった。しかし status 遷移コマンドと area を指定しない `update` は、issue の frontmatter に既に入っている area をそのまま `ctx.canonical_dir()` に渡して再配置するだけで、検証しない。

- `src/commands/done.rs:51`
- `src/commands/pending.rs:51`
- `src/commands/in_progress.rs:51`
- `src/commands/reopen.rs:48`
- `src/commands/update.rs:168`（`validate_input` は `input.area` が `Some` のときだけ検証する）

結果として、`create` が拒否するはずのディレクトリを renga 自身が作ってしまう。

## 再現（今回の差分を適用した状態のバイナリ）

```
.renga.yml: group_by: [area]
issues/open/1-task.md   (area: 2024 Q1, group_by 有効化前に作られた issue)

$ renga done 1
issues/2024-q1/done/1-task.md      # 仕様上「不正」な配置を renga が新規作成した

$ renga create "Next one"
issues/open/2025-next-one.md       # #243 のバグが再発
```

`renga update 1 --priority high` も同様に `issues/2024-q1/open/` へ配置する（`--area` を渡していないので検証されない）。

## 論点

1. 実効 area（入力 area がなければ frontmatter の area）に対して検証すべき。ただし単純に bail すると、legacy データの issue を `done` すらできなくなる。
2. 代替案: 検証に失敗する area は `group_by` を無視してフラットな `issues/<status>/` に置き、warning を出す（`done.rs` が unparseable frontmatter に対して既にやっている fallback と同じ扱い）。ユーザーは `renga update <ID> --area <正しい名前>` で復旧できる。
3. 併せて #250（`next_id` 側の根本修正）も行えば、万一そのディレクトリが存在しても ID は飛ばない。

## ドキュメント

spec.md / spec.ja.md は「`create`・`update` はエラーになる」と書いているが、`update` が拒否するのは `--area` を明示したときだけ。仕様文もこの挙動決定に合わせて直す必要がある。

## 出典

レビュー（review エージェント）で発見。手動再現済み。

