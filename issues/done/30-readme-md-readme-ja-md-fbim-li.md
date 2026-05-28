---
status: done
priority: medium
area: docs
labels: []
---

# README.md / README.ja.md の fbim list テーブルに --label オプションが欠落

## 問題

README.md 67行目および README.ja.md 67行目の `fbim list` コマンド表記に `--label <label>` が含まれていない。

spec.md 93行目には `fbim list [--status ...] [--area <area>] [--label <label>] [--milestone <milestone>] [--json]` と明記されている。

実装（src/cli.rs:121-125）でも `--label` は実装済み。

## 関連箇所

- README.md:67 fbim list の行（--label が欠落）
- README.ja.md:67 fbim list の行（--label が欠落）
- spec.md:93 完全なシグネチャ
- src/cli.rs:121-125 --label の実装
