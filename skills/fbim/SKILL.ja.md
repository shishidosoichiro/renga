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
