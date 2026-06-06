---
schema_version: 1
status: done
priority: medium
area: docs
labels: []
---

# README.md/README.ja.md のコマンド表が複数 ID 対応後も単数 <ID> のまま

## 問題

`README.md` / `README.ja.md` の Commands テーブルが以下のまま:

```
| renga done <ID>         | Mark an issue as done |
| renga pending <ID>      | Put an issue on hold  |
| renga in-progress <ID>  | ...                   |
| renga reopen <ID>       | ...                   |
```

実装は複数 ID に変わったが README が更新されていない。

## 修正方針

`<ID>...` に変更する。英日同時更新。
