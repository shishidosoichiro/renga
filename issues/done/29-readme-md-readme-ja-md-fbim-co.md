---
status: done
priority: medium
area: docs
labels: []
---

# README.md / README.ja.md のコマンドテーブルに fbim completions が欠落

## 問題

README.md 60〜69行目のコマンドテーブルに `fbim completions` が含まれていない。README.ja.md の同テーブル（60〜69行目）も同様。

spec.md 95行目には `fbim completions <bash|zsh|fish>` が Commands セクションに明記されている。

実際のコマンドテーブルには `fbim help` は載っているが `fbim completions` が欠落している。

## 関連箇所

- README.md:60-69 コマンドテーブル
- README.ja.md:60-69 コマンドテーブル
- spec.md:95 `fbim completions <bash|zsh|fish>`
- spec.ja.md:95 `fbim completions <bash|zsh|fish>`
