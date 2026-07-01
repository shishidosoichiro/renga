# Renga 仕様

File-Based Issue Management の仕様。

---

## ディレクトリ構造

```
issues/
  README.md           一覧（renga list で自動生成）
  open/
    N-name.md         open な issue
  in-progress/
    N-name.md         作業中の issue
  pending/
    N-name.md         保留中の issue
  done/
    N-name.md         完了した issue
  unknown/
    N-name.md         frontmatter がないか parse できない issue
```

## ファイル命名規則

2種類のレイアウトを混在して使用できる。

**フラットファイル**（デフォルト）:
```
N-short-name.md
```

**ディレクトリ形式**（`--dir=true` で作成。添付ファイルやメモを issue と同じ場所に置ける）:
```
N-short-name/
  README.md     issue の本文（フラットファイルと同じ形式）
  ...           その他のファイル
```

- `N`: 正の整数（ゼロ埋めなし）。`renga create` が既存ファイルの最大番号 + 1 を割り当てる
- `short-name`: ケバブケースの短い説明（英数字・ハイフンのみ、30文字以内）

**ID はファイル名またはディレクトリ名から取得する。** frontmatter や本文には ID を持たない。

**後方互換**: 旧来の4桁・5桁ゼロ埋め ID（`0042-*.md`, `00042-*.md`）は引き続き読み込める。新規 issue は常にゼロ埋めなしで作成する。

`renga update <ID> --dir=true` でフラットファイルをディレクトリに展開、`--dir=false` でディレクトリをフラットファイルに畳み込む（`README.md` 以外のファイルがある場合はエラー）。

## frontmatter

```markdown
---
schema_version: 1           # 省略可能（このフィールドが導入される前に作成したファイルには含まれない）
status: open
priority: high/medium/low   # 省略可能
area: エリア名              # 省略可能（例: core, cli, docs）
labels: []
milestone: v1.0             # 省略可能（例: v1.0, 2026-Q3, sprint-3）
assignee: alice             # 省略可能（担当者）
---
```

通常の読み取り・一覧表示では frontmatter は省略可能。省略した場合は `status: unknown`、`priority` は `-`（unknown）、`area` は空（グループなし）として扱う。一方で `renga validate` はより厳密に検査し、frontmatter がない、parse できない、または frontmatter に `status` フィールドがないファイルを error として報告する。これにより、作業完了前に壊れた issue ファイルを検出できる。

`status` frontmatter フィールドが issue status の正とする情報源。ステータス別ディレクトリ（`open/`, `pending/`, `in-progress/`, `done/`）は発見性のための同期済みレイアウトである。ファイルが frontmatter の status と異なるディレクトリに置かれている場合、`renga validate` は error として報告する。`renga validate --auto-correct` は frontmatter の status に合わせてファイルを移動する。

`schema_version` は frontmatter のフォーマットバージョンを示す。新規作成ファイルには `schema_version: 1` が含まれる。このフィールドがない旧ファイルは内部的に `None`（不在）として保持され、ツールはバージョン 1 と同等に扱う。

`area`・`priority`・`milestone`・`assignee` は frontmatter がある場合でも省略可能。`area` を省略すると issue はグループなし扱いになり、`issues/README.md` では見出しなしで表示される。`priority` を省略すると `medium` がデフォルトになる。`milestone` を省略するとどのマイルストーンにも属さない。`assignee` を省略すると担当者なしとなる。

**status の値**

| 値 | 意味 |
|---|---|
| `open` | 対応が必要。作業対象 |
| `pending` | 決定待ち・確認待ち。作業保留 |
| `in-progress` | 現在作業中 |
| `done` | 完了（`done/` に移動済み） |
| `unknown` | frontmatter がないか parse できない。renga では書き込めない読み取り専用の状態 |

**priority の意味**

| 値 | 意味 |
|---|---|
| `high` | すぐに対応が必要 |
| `medium` | 要対応だが緊急ではない |
| `low` | 提案・改善の余地 |
| `-` | 不明 — frontmatter がないかフィールドが欠落。読み取り専用 |

## タイトル

**タイトルは本文の最初の `# 見出し` 行から取得する。** frontmatter にはタイトルを持たない。

`# 見出し` がない場合はファイル名のステムをタイトルとして使う。

## issue ファイルのテンプレート

```markdown
---
schema_version: 1
status: open
priority: medium
area: エリア名
labels: []
---

# タイトル

何をするかの説明。
```

## コマンド

```
renga init
renga create <title> [--id <N>] [--slug <slug>] [--priority high|medium|low] [--area <area>] [--body <text|-\>] [--milestone <milestone>] [--assignee <assignee>] [--label <label>]... [--dir=true|false]
renga create --json
renga done <ID>...
renga pending <ID>...
renga in-progress <ID>...
renga reopen <ID>...
renga list [--status open|pending|in-progress|done|unknown] [--area <area>] [--label <label>] [--milestone <milestone>] [--assignee <assignee>] [--json]
renga show <ID> [--json]
renga edit <ID>
renga update <ID> [<title>] [--priority high|medium|low] [--area <area>] [--status open|pending|in-progress] [--milestone <milestone>] [--assignee <assignee>] [--label <label>]... [--add-label <label>]... [--remove-label <label>]... [--body <text|->]
renga update <ID> --dir=true|false
renga update <ID> --json
renga info
renga migrate
renga validate [ID]... [--auto-correct]
renga completions <bash|zsh|fish>
renga help [command]
```

`N` はゼロ埋めなしの整数 ID（例: `42`）。レガシーのゼロ埋め ID（例: `00042`）も受け付ける。

`renga create --json` は標準入力から 1 つの JSON object を読む。対応フィールドは
`title`（必須）、`id`、`slug`、`priority`、`area`、`body`、`milestone`、`assignee`、`labels`。
positional 引数やフィールド指定フラグとは併用できない。

`renga update <ID> --json` は標準入力から 1 つの JSON object を読む。対応フィールドは
`title`、`priority`、`area`、`status`、`milestone`、`assignee`、`labels`、`add_labels`、
`remove_labels`、`body`。positional 引数やフィールド指定フラグとは併用できず、
`<ID>` は更新対象の指定として残る。

`--milestone` または `--assignee` に空文字列を渡すと、そのフィールドを frontmatter から削除する（例: `renga update 1 --assignee ''`）。

`renga validate` は ID を指定しない場合、`issues/` 配下の全 issue ファイルを検査する。1つ以上の ID を渡すと対象 issue ID だけを検査するが、指定した ID の重複ファイルは引き続き検出する。`--auto-correct` は status ディレクトリ不整合を修正し、frontmatter status が示すディレクトリへファイルを移動する。

issue ファイルが `done/` 配下にあっても frontmatter status が active（`open`、`pending`、`in-progress`）なら、active issue 用コマンドはその issue を操作できる。これは frontmatter status を正とする情報源とみなすためである。この場合、コマンドは warning を表示し、`renga validate <ID> --auto-correct` を推奨する。通常の `status: done` issue は、status を変更するコマンド（`done`・`pending`・`in-progress`・`reopen`）については reopen されるまで対象外である。ただし `update` と `edit` は例外で、done issue のフィールドを直接編集できる。`update` は `--status done` を受け付けないため、これによって issue が done に遷移することはない。

## `.renga.yml` の仕様

プロジェクトルートに置く設定ファイル。省略可能。

```yaml
issues_dir: issues    # デフォルト: issues

area_order:           # 一覧での area の表示順（省略時はアルファベット順）
  - backend
  - frontend
  - misc

area_labels:          # area の表示名（省略時は area 名をそのまま使う）
  backend: "バックエンド"
  frontend: "フロントエンド"
  misc: "その他"
```

## issues/ の探索

`renga` コマンドはカレントディレクトリから上位へ辿り、最初に次のいずれかに該当するディレクトリで止まる。

1. `.renga.yml` が存在する — `issues_dir` の値を issues ディレクトリとして使う
2. `issues/` サブディレクトリが存在する

何も見つからない場合はカレントディレクトリの `issues/` にフォールバックする。
