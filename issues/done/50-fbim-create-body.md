---
status: done
priority: medium
area: cli
labels: []
---

# fbim create で --body - を指定すると標準入力を本文として使えるようにする

`--body -` と指定した場合、標準入力からテキストを読み込んで issue の本文として使う。

## 変更箇所

- `src/cli.rs`: `--body` のヘルプに `-` で stdin を使える旨を追記
- `src/commands/create.rs`: `args.body == Some("-")` のとき `io::stdin()` から読み込む
- `README.md` / `README.ja.md`: `--body -` の使い方を記載
- テスト追加
