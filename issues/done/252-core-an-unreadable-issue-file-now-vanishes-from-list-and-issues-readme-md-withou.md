---
schema_version: 1
status: done
priority: medium
area: core
labels: [found_in_impl]
---

# core: an unreadable issue file now vanishes from list and issues/README.md without any error

## 背景

#248 の修正で `all_issues` の `std::fs::read_to_string(&path)?` を `.unwrap_or_default()` に変えた。狙い（補完候補が全滅しなくなる）は達成されているが、コメントの「keep listing it rather than aborting the whole listing or dropping it silently」は既定の `renga list` には当てはまらない。読めないファイルは `Status::Unknown` になり、`list` の既定フィルタ（open / pending / in-progress）から外れて消える。

## 再現（今回の差分を適用した状態のバイナリ）

```
issues/open/1-important-task.md   (area: core, priority: high)
issues/open/2-second.md

$ chmod 000 issues/open/1-important-task.md
$ renga list          # 何も出ない。exit 0
$ renga done 2        # 成功。副作用で issues/README.md を再生成
$ cat issues/README.md
# issue 1 の行が消えている（テーブルごと消滅）
```

修正前は `renga list` が `Permission denied (os error 13)` で落ちていた。落ちるのも良くはないが、少なくとも異常が分かった。今は exit 0 で静かに消える。`renga validate` を実行して初めて分かる。

生成物である `issues/README.md` から消えるのは次回の mutation で自己修復されるが、その間 git diff に「issue が消えた」変更が乗る。

## 提案

いずれか:

1. 読み取り失敗時に stderr へ warning を出す（`renga` には既に warning 機構がある）。補完経路でも stderr は実害が小さい。
2. 寛容さを補完経路に閉じる（`completions.rs` は既に `all_issues(...).unwrap_or_default()` している）。`all_issues` にエラーを返させたまま、補完側だけ「読めるものだけ集める」別関数を使う。

現状の「frontmatter が壊れたファイルと同じ Unknown 扱い」は一貫性としては筋が通るので、案 1（warning 追加）が最小の変更と思われる。

## 確認済み（懸念なし）

書き込み系が空内容で上書きする経路はない。`Issue::raw_content` を書き込みに使うのは `validate.rs:263`（`correct_status_directory`）だけで、`validate` は `all_issues` を使わず自前で `read_to_string(path)?` している。`done` / `update` / `reopen` 等も対象ファイルを `?` 付きで読み直しており、読めなければエラーで停止する。

## 出典

レビュー（review エージェント）で発見。手動再現済み。

