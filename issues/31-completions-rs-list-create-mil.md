---
status: open
priority: medium
area: cli
labels: []
---

# completions.rs の list / create 補完候補に --milestone が欠落

## 問題

src/commands/completions.rs の `write_candidates` 関数で、`list` サブコマンドと `create` サブコマンドの補完候補に `--milestone` が含まれていない。

### list サブコマンド（completions.rs:117-129）

`--status` / `--area` / `--label` / `--json` は補完されるが、`--milestone` が欠落。

実装（src/cli.rs:126-128）では `--milestone` は実装済み。

### create サブコマンド（completions.rs:131-143）

`--slug` / `--priority` / `--area` / `--body` は補完されるが、`--milestone` が欠落。

実装（src/cli.rs:88-91）では `--milestone` は実装済み。

## 関連箇所

- src/commands/completions.rs:117-143
- src/cli.rs:88-91, 126-128
