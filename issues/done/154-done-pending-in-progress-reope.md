---
schema_version: 1
status: done
priority: high
area: cli
labels: []
---

# done/pending/in-progress/reopen: 引数なし (0 ID) で exit 0 になる

## 問題

`num_args(1..)` を指定しているが、`Vec<String>` フィールドに `required = true` を付けていないため 0 引数が許容される。

```
$ renga done          # exit 0、何も出力しない
$ renga pending       # exit 0、何も出力しない
$ renga in-progress   # exit 0、何も出力しない
$ renga reopen        # exit 0、何も出力しない
```

ユーザーが引数を指定し忘れた場合にエラーにならず、気づかずに見過ごす恐れがある。

## 修正方針

`#[arg(num_args(1..), required = true)]` に変更する。
または `num_args(1..)` を削除して代わりに `required = true` で単純に必須 Vec にする。

## 追加すべきテスト

- `done_no_args_fails`
- `pending_no_args_fails`
- `in_progress_no_args_fails`
- `reopen_no_args_fails`
