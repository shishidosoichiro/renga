---
schema_version: 1
status: done
priority: low
area: core
labels: [found_at:0.15.0]
---

# core: a directory-based issue without README.md still exposes its attachments as issues

#241 の修正は `README.md` の存在を条件にしているため、dir 形式 issue から `README.md` が失われると #241 のデータ破壊パスがそのまま復活する。

## 再現

```sh
renga init
renga create "Dir Task" --dir=true
printf -- '---\nstatus: open\n---\n\n# Attachment\n' > issues/open/1-dir-task/9-design.md
rm issues/open/1-dir-task/README.md      # 事故・手作業・マージ結果など

renga list
# [9] open medium  Attachment              ← 添付が issue として現れる

renga done 9
# issues/done/9-design.md                 ← 添付が親ディレクトリの外へ移動（#241 と同じ破壊）
```

`validate` は `open/1-dir-task/9-design.md: status directory mismatch` を報告するが、これは「添付を issue として扱った上での誤検出」であり、本当の問題（`1-dir-task/` に `README.md` が無い）を指していない。

## 提案

1. `validate` が「`id_prefix` を持つディレクトリなのに `README.md` が無い」を専用のエラーとして報告する（今の `status directory mismatch` より原因に近い）
2. spec.md / spec.ja.md に「dir 形式 issue は `README.md` を持つディレクトリである。`README.md` の無い `N-name/` は dir 形式 issue ではなく、配下は通常どおり走査される」を明記する。現在の追記（「ディレクトリ形式 issue の中身は renga から見て不可分」）は `README.md` の有無という判定条件に触れていないため、実装より強い保証を約束してしまっている

## 関連

- `src/issue.rs::dir_based_issue_readme`、`src/commands/validate.rs`
- #241、#245

