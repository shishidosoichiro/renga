---
status: done
priority: medium
area: cli
labels: []
---

# fbim create で --id を指定してイシュー番号を任意に設定できるようにする

`--id <N>` を指定した場合、自動採番の代わりに指定した番号でファイルを作成する。
指定した番号のファイルが既に存在する場合はエラーにする。

## 変更箇所

- `src/cli.rs`: `CreateArgs` に `id: Option<String>` を追加
- `src/commands/create.rs`: `args.id` が Some のときはそれを使い、None のときは `next_id` を使う
- `README.md` / `README.ja.md`: `--id` オプションを記載
- テスト追加
