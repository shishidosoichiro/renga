# /fbim スキル

File-Based Issue Management (FBIM) の操作スキル。
すべての操作は `${CLAUDE_SKILL_DIR}/scripts/fbim` を呼び出して行う。

## 呼び出し形式

```
/fbim [create] <タイトル>          issue を作成する
/fbim done <NNNN>                  issue を完了にする
/fbim pending <NNNN>               issue を保留にする
/fbim reopen <NNNN>                issue を再開する
/fbim help [コマンド名]            ヘルプを表示する
```

`create` は省略可能。`/fbim タイトル` でそのまま issue を作成する。

---

## ルール（全コマンド共通）

- `$ARGUMENTS` が空の場合は `${CLAUDE_SKILL_DIR}/scripts/fbim help` を実行して表示する
- スクリプトのエラー出力はそのままユーザーに伝える

---

## create

`$ARGUMENTS` の先頭が `create` なら除いた残りをタイトルとして使う。そうでなければ `$ARGUMENTS` 全体がタイトル。

```
${CLAUDE_SKILL_DIR}/scripts/fbim create "<タイトル>" --slug <slug> --area <area>
```

- `--slug`: タイトルを英語に変換してケバブケースにしたもの（30文字以内）
- `--area`: 文脈から判断する。不明なら `misc`
- `--body`: 補足説明があれば付ける
- 作成したファイルパスをユーザーに伝える

---

## done

```
${CLAUDE_SKILL_DIR}/scripts/fbim done <NNNN>
```

移動先ファイルパスをユーザーに伝える。

---

## pending

```
${CLAUDE_SKILL_DIR}/scripts/fbim pending <NNNN>
```

変更したファイルパスをユーザーに伝える。

---

## reopen

```
${CLAUDE_SKILL_DIR}/scripts/fbim reopen <NNNN>
```

移動先ファイルパスをユーザーに伝える。

---

## help

```
${CLAUDE_SKILL_DIR}/scripts/fbim help [コマンド名]
```

出力をそのまま表示する。
