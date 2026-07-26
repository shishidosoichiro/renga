---
schema_version: 1
status: done
priority: low
area: cli
labels: [found_in_impl]
---

# cli: one unreadable issue file wipes out every shell completion candidate

`emit_issues` が `all_issues(...).unwrap_or_default()` でエラーを握り潰しているため、1ファイルでも読めないと候補が 0 件になる。

```rust
// src/commands/completions.rs
let issues = all_issues(&ctx.issues_dir, statuses, None, None, None, None).unwrap_or_default();
```

`all_issues` は `std::fs::read_to_string(&path)?` で最初の I/O エラーをそのまま返すので、権限のないファイル・壊れた symlink・`README.md` という名前のディレクトリが1つあるだけで `Err` になり、`unwrap_or_default()` が全候補を捨てる。

## 退行

書き換え前の `emit_issues_recursive` はファイル単位で `read_title(path).unwrap_or_else(|| stem)` にフォールバックしていたため、1ファイルが読めなくても他の候補は出ていた。「TAB でエラーを出さない」方針は正しいが、粒度がファイル単位からツリー全体に落ちている。

## 修正案

`all_issues` の読み込みループを、パースだけでなく I/O エラーもファイル単位でスキップする形にする（`renga list` から見ても「1ファイル読めないだけで一覧全体が失敗する」より望ましい）。あるいは completions 側で `collect_issue_files` を回してファイル単位に握り潰す。

## 判断: #240 のコミットでは直さず open のまま残す

`found_in_impl` だが、#240 のコミット前修正には含めなかった。理由:

- completions 側だけで握り潰す案は、`all_issues` が持つ「frontmatter が壊れていても Unknown + body からタイトルを拾う」フォールバックを再実装することになる。#239/#240/#242 を生んだ「走査・判定ロジックの重複」を消すのが #240 の主眼なので、それを打ち消す
- `all_issues` 側を直す案は `renga list` の挙動変更（読めないファイルがあっても一覧が出る）を伴う。方向としては望ましく前例もある（CHANGELOG の "skip unparseable issue files instead of aborting"）が、ユーザーに見える挙動変更なので独立した判断・コミットに分けるべき
- 実害は限定的: `all_issues` が `Err` を返す状況（権限なし・壊れた symlink 等）では `renga list` 自体もエラーで失敗するため、補完が空になるのは `list` と一貫している

修正する場合は `all_issues` 側で I/O エラーもファイル単位でスキップする案を採る。

## 関連

- `src/commands/completions.rs::emit_issues`、`src/issue.rs::all_issues`
- #240

