# /issue スキル

File-Based Issue Management (FBIM) の操作スキル。

## 呼び出し形式

```
/issue [create] <タイトル>   issue を作成する
/issue done <NNNN>           issue を done に移動する
/issue pending <NNNN>        issue を pending にする
/issue reopen <NNNN>         done の issue を open に戻す
```

引数なし・または `create` のみの場合は使い方を表示する。

---

## ルール（全コマンド共通）

- `issues/` ディレクトリが存在しない場合は「`issues/` が見つかりません」と伝えて止まる
- コマンド完了後、必ず `bin/gen-issues-readme` を実行して `issues/README.md` を再生成する
- 再生成に失敗しても issue 操作は成功扱いとするが、エラーをユーザーに伝える

---

## create

**引数**: `$ARGUMENTS` からコマンド名（`create`）を除いたテキストをタイトルとして使う。コマンド名を省略した場合（`/issue タイトル`）は `$ARGUMENTS` 全体がタイトル。

### 手順

1. `$ARGUMENTS` が空、または `create` だけの場合は使い方を表示して終了する
2. 既存 issue を確認し、同内容が存在する場合は「すでに NNNN-name.md が存在します」と伝えて止まる
3. 次番号を取得する:
   - `issues/` と `issues/done/` の `NNNN-` ファイルを走査し、最大番号 + 1 をゼロ埋め4桁で算出する
   - ファイルが存在しない場合は `0001`
4. ファイル名の `short-name` を生成する:
   - タイトルを英語に変換し、ケバブケースで30文字以内に短縮する
5. `issues/NNNN-short-name.md` を以下の内容で作成する:

```
---
status: open
priority: medium
area: docs
labels: []
---

# <タイトル>

<タイトルを1文で説明した内容。ユーザーが追記できるように簡潔に。>
```

6. `bin/gen-issues-readme` を実行する
7. 作成したファイルパスを伝える

---

## done

**引数**: `$ARGUMENTS` から `done` を除いた番号（例: `0042`）

### 手順

1. `issues/NNNN-*.md` にマッチするファイルを探す（`NNNN` は4桁にゼロ埋めして検索）
2. 見つからない場合は「NNNN の issue が見つかりません」と伝えて止まる
3. `issues/done/` ディレクトリが存在しない場合は作成する
4. ファイルを `issues/done/NNNN-name.md` に移動する（`mv` コマンド）
5. frontmatter の `status:` を `done` に変更する
6. `bin/gen-issues-readme` を実行する
7. 移動先ファイルパスを伝える

---

## pending

**引数**: `$ARGUMENTS` から `pending` を除いた番号

### 手順

1. `issues/NNNN-*.md` にマッチするファイルを探す
2. 見つからない場合は「NNNN の issue が見つかりません」と伝えて止まる
3. frontmatter の `status:` を `pending` に変更する
4. `bin/gen-issues-readme` を実行する
5. 変更したファイルパスを伝える

---

## reopen

**引数**: `$ARGUMENTS` から `reopen` を除いた番号

### 手順

1. `issues/done/NNNN-*.md` にマッチするファイルを探す
2. 見つからない場合は「NNNN の done issue が見つかりません」と伝えて止まる
3. ファイルを `issues/NNNN-name.md` に移動する（`mv` コマンド）
4. frontmatter の `status:` を `open` に変更する
5. `bin/gen-issues-readme` を実行する
6. 移動先ファイルパスを伝える
