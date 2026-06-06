---
schema_version: 1
status: done
priority: medium
area: docs
labels: []
---

# spec.md/spec.ja.md のコマンド定義が複数 ID 対応後も単数 <ID> のまま

## 問題

`spec.md` / `spec.ja.md` の Commands セクションに以下の記述がある:

```
renga done <ID>
renga pending <ID>
renga in-progress <ID>
renga reopen <ID>
```

実装は複数 ID に変わったが仕様書が更新されていない。

## 修正方針

`<ID>...` または `<ID> [<ID>]...` に変更する。

両ファイルを同時に更新すること（英日同期ルール）。
