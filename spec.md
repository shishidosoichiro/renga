# FBIM 仕様

File-Based Issue Management の詳細仕様。

---

## ディレクトリ構造

```
issues/
  README.md          一覧（自動生成。手動編集不可）
  NNNN-name.md       open または pending な issue
  done/
    NNNN-name.md     完了した issue
```

## ファイル命名規則

```
NNNN-short-name.md
```

- `NNNN`: ゼロ埋め4桁の連番。次番号は `bin/next-id issues/` で取得する
- `short-name`: ケバブケースの短い説明（英数字・ハイフンのみ）
- 同じ内容の issue が既に存在する場合は新規作成しない

## frontmatter

```markdown
---
status: open
priority: high/medium/low
area: エリア名（例: authz, authn, test, docs）
labels: []
---
```

**status の値**

| 値 | 意味 |
|---|---|
| `open` | 対応が必要。作業対象 |
| `pending` | 決定待ち・確認待ち。作業保留 |
| `done` | 完了（`done/` に移動済み） |

**priority の意味**

| 値 | 意味 |
|---|---|
| `high` | すぐに修正が必要。正確性・整合性に問題がある |
| `medium` | 要確認。設計判断や方針の議論が必要 |
| `low` | 提案。改善の余地があるが緊急ではない |

## issue ファイルのテンプレート

```markdown
---
status: open
priority: high/medium/low
area: エリア名
labels: []
---

# タイトル

何をするかの説明。

## 背景（任意）

詳細な文脈や関連情報。

## 関連（任意）

- 関連する issues や ADR への参照
```

## `issues/README.md` の管理

- 手動編集禁止。`bin/gen-issues-readme` を実行して再生成する
- issue を新規作成・`done/` 移動・status 変更のたびに実行する

## アクション

### create

```
/issue タイトル
/issue create タイトル
```

1. `bin/next-id issues/` で次番号を取得する
2. `issues/NNNN-short-name.md` を frontmatter テンプレートで作成する（short-name はタイトルから生成）
3. `bin/gen-issues-readme` を実行する

### done

```
/issue done NNNN
```

1. `issues/NNNN-*.md` を `issues/done/NNNN-*.md` に移動する
2. frontmatter の `status` を `done` に変更する
3. `bin/gen-issues-readme` を実行する

### pending

```
/issue pending NNNN
```

1. `issues/NNNN-*.md` の frontmatter `status` を `pending` に変更する
2. `bin/gen-issues-readme` を実行する

### reopen

```
/issue reopen NNNN
```

1. `issues/done/NNNN-*.md` を `issues/NNNN-*.md` に移動する
2. frontmatter の `status` を `open` に変更する
3. `bin/gen-issues-readme` を実行する

---

## `bin/next-id` の仕様

`bin/next-id <dir>` は指定ディレクトリ（および `done/` サブディレクトリ）の `NNNN-` ファイルを走査し、最大番号 + 1 をゼロ埋め4桁で出力する。ファイルが存在しない場合は `0001` を返す。

## `bin/gen-issues-readme` の仕様

`issues/` と `issues/done/` の全ファイルを走査し、frontmatter の `status` / `priority` / `area` を読み取って `issues/README.md` を再生成する。open と pending を先に、done を後に列挙する。
