---
schema_version: 1
status: done
priority: medium
area: docs
labels: [found_in_impl]
---

# spec.md/spec.ja.md の slug 上限記述が新しい80バイト方式に未更新

## 問題

issue #214 の対応で `make_slug`（`src/issue.rs`）の切り詰め方式を「文字数で30文字」から
「バイト数で80バイト」に全面変更したが、`spec.md` / `spec.ja.md` の該当記述が更新されていない。

```
spec.md:41:  - `short-name`: Kebab-case short description (ASCII alphanumeric and hyphens only, up to 30 characters).
spec.ja.md:41: - `short-name`: ケバブケースの短い説明（英数字・ハイフンのみ、30文字以内）
```

CLAUDE.md のドキュメント更新ルールに「issue ファイルの形式・ID・タイトルの仕様 → spec.md, spec.ja.md」と
明記されている通り、この変更は同じコミットで spec.md / spec.ja.md を更新する必要がある。

## 対応

`up to 30 characters` / `30文字以内` を `up to 80 bytes` / `80バイト以内` 相当の記述に更新する。

