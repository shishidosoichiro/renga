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

`.renga.yml` で `group_by: [area]` を設定すると、status の上に area のディレクトリ階層が加わる（`issues/<area>/<status>/N-name.md`）。詳細は「`.renga.yml` の仕様」を参照。

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

ディレクトリ形式 issue の中身は renga から見て不可分である。`N-short-name/` 配下のファイルは `N-slug.md` という名前であっても独立した issue として走査されず、その番号が ID を予約することもない。issue を表すのは `README.md` のみ。

- `N`: 正の整数（ゼロ埋めなし）。`renga create` が既存ファイルの最大番号 + 1 を割り当てる
- `short-name`: ケバブケースの短い説明（Unicode の英数字・ハイフンのみ、80バイト以内）

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

group_by:             # status の上にネストする追加のディレクトリ階層（省略時は従来通りフラット）
  - area

defaults:             # create でフラグを省略したときのデフォルト値
  dir: false          # create --dir のデフォルト（キー省略時は flat）
```

`group_by` は list 型。現時点では要素数1・値は `"area"` のみサポートする（2要素以上、または `"area"` 以外の値はエラー）。

`group_by: [area]` を設定すると、issue は `issues/<area>/<status>/N-name.md` に配置される。`area` はディレクトリ名として `renga create` のスラグ化と同じルール（`make_slug`）で正規化され、`/` を含む非英数字は `-` にまとめられる（実際のネストされたフォルダにはならない）。`area` が未設定の issue は従来通り `issues/<status>/` 直下に置かれる。

`area` の値（スラグ化後）が予約済みのステータスディレクトリ名（`open`・`pending`・`in-progress`・`done`・`unknown`）と衝突する場合、`create`・`update --area` はエラーになる。既存データでこの衝突が見つかった場合、issue 自体は `list`・`show` から引き続き読め、`migrate` は該当 issue を warning 付きでスキップし、`validate` はエラーとして報告する（自動修正はしない）。issue を再配置するコマンド（`done`・`pending`・`in-progress`・`reopen`、および `--area` を指定しない `update`）は、衝突する area を area 無しとして扱い、warning を出してフラットな `issues/<status>/` に置く。これは frontmatter が壊れている場合と同じフォールバックであり、renga 自身の `create` が拒否するディレクトリ構成を renga が作らないようにするため。

area のスラグが issue ID のように見えるだけなら問題ない。area `2024 Q1` は `2024-q1` にスラグ化され、通常どおり動作する。area ディレクトリとディレクトリ形式 issue は名前ではなく形で区別する（issues ルート直下にあり、かつ status サブディレクトリを持つものが area）。

`area`・`status` の変更は該当コマンド（`update`・`done`・`pending`・`in-progress`・`reopen`）が自動的にファイルを正しいディレクトリへ移動する。`update` は実際には `area`・`status` を変更しない編集（`--assignee`・`--label` 等）でも、常に編集の副作用として issue を canonical ディレクトリへ再配置する。そのため、もともと配置がずれていた issue（例: recoverable な status ディレクトリ不一致で見つかった issue）は `area`・`status` に触れない編集でも自動的に正しい位置へ自己修復される。`renga validate --auto-correct` は `group_by` の設定（有効・無効どちらへの変更も）と実際の配置がずれている issue を検出・修正する。`group_by` を新たに有効にして既存 issue を一括で移行するには `renga migrate` を使う。

`defaults` は `create` でフラグを省略したときのデフォルト値の名前空間で、現時点では `defaults.dir` のみサポートする。将来 assignee・priority 等のデフォルトが必要になっても、新しいトップレベルキーを増やさずこの名前空間に足せる。`defaults.dir: true` を設定すると `renga create` はデフォルトで dir-based issue（`N-slug/README.md`）を作成する。コマンドラインで明示的に指定した `--dir=true`/`--dir=false` は常に config のデフォルトより優先される。`create --json` にも同様に適用される — JSON 入力には `dir` フィールド自体が存在しないため、JSON 経由での作成時に flat/dir-based を制御する唯一の手段は `defaults.dir` になる。`renga migrate` は `defaults.dir: true` のとき既存の flat issue を dir-based に変換するが、これは一方向のみである（`update --dir=false` が `README.md` 以外のファイルを含むディレクトリの畳み込みを拒否する既存仕様があるため、添付ファイルのある issue に対する一括畳み込みは信頼できず実装しない）。`group_by` と異なり、`validate` は `defaults.dir` を検査**しない** — issue が flat か dir-based かは（添付ファイルの有無等による）issue ごとの正当な選択であり、frontmatter から一意に導出される「唯一の正解」ではないため。

## issues/ の探索

`renga` コマンドはカレントディレクトリから上位へ辿り、最初に次のいずれかに該当するディレクトリで止まる。

1. `.renga.yml` が存在する — `issues_dir` の値を issues ディレクトリとして使う
2. `issues/` サブディレクトリが存在する

何も見つからない場合はカレントディレクトリの `issues/` にフォールバックする。
