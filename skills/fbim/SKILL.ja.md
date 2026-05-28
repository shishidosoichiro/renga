# /fbim スキル

File-Based Issue Management (FBIM) の操作スキル。
すべての操作は `fbim` バイナリを呼び出して行う。

## 呼び出し形式

```
/fbim [create] <タイトル>          issue を作成する
/fbim done <NNNNN>                 issue を完了にする
/fbim pending <NNNNN>              issue を保留にする
/fbim reopen <NNNNN>               issue を再開する
/fbim list                         open/pending の一覧を表示する
/fbim show <NNNNN>                 issue の詳細を表示する
/fbim help [コマンド名]            ヘルプを表示する
```

`create` は省略可能。`/fbim タイトル` でそのまま issue を作成する。

---

## エージェント作業規約

AI エージェントが FBIM の issue を扱う際は、以下の規約に従う。

**作業開始前**
- `fbim list` を実行して open/pending の issue を確認する。
- 対象 issue を `fbim show <N>` で読んでから作業を始める。

**作業中**
- issue タイトルから自明でない判断・制約を発見したら、issue 本文の `## Notes` セクションに追記する（`create` 時は `--body`、既存 issue はファイルを直接編集）。
- issue ファイルを削除しない。status の変更は `fbim pending` または `fbim done` を使う。

**判断不能・ブロックされた場合**
- 作業を完了できない場合は `fbim pending <N>` を実行し、理由を issue 本文の `## Notes` に書く。
- 完了できないまま issue を `open` のままにして作業を終えない。

**作業完了後**
- issue を close する前に `fbim validate` を実行する。exit code 1 はエラーがあることを意味するので、先に修正する。
- `fbim done <N>` で issue を close する。ファイルを直接編集して `status: done` にすることは禁止。必ずコマンドを使う。
- 作業完了後に `fbim done` を実行せず issue を `open` のまま放置しない。

---

## ルール（全コマンド共通）

- `$ARGUMENTS` が空の場合は `fbim help` を実行して表示する
- スクリプトのエラー出力はそのままユーザーに伝える

---

## create

`$ARGUMENTS` の先頭が `create` なら除いた残りをタイトルとして使う。そうでなければ `$ARGUMENTS` 全体がタイトル。

```
fbim create "<タイトル>" --slug <slug> --area <area>
```

- `--slug`: タイトルを英語に変換してケバブケースにしたもの（30文字以内）
- `--area`: 文脈から判断する。不明なら `misc`
- `--body`: 補足説明があれば付ける
- 作成したファイルパスをユーザーに伝える

---

## done

```
fbim done <NNNNN>
```

移動先ファイルパスをユーザーに伝える。

---

## pending

```
fbim pending <NNNNN>
```

変更したファイルパスをユーザーに伝える。

---

## reopen

```
fbim reopen <NNNNN>
```

移動先ファイルパスをユーザーに伝える。

---

## list

```
fbim list
```

フィルターあり:

```
fbim list --area <area>
```

出力をユーザーに表示する。

---

## show

```
fbim show <NNNNN>
```

出力をユーザーに表示する。

---

## validate

```
fbim validate
```

issue ファイルをまとめて変更した後に実行する。出力をユーザーに表示する。
exit code 1 はエラーがあることを意味する（フロントマターのパース失敗・不正な status・ID 重複）。
警告（schema_version 欠落）は exit code 0 で終了する。

---

## help

```
fbim help [コマンド名]
```

出力をそのまま表示する。
