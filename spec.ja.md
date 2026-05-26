# FBIM 仕様

File-Based Issue Management の仕様。

---

## ディレクトリ構造

```
issues/
  README.md           一覧（fbim list で自動生成）
  N-name.md           open または pending な issue
  done/
    N-name.md         完了した issue
```

## ファイル命名規則

```
N-short-name.md
```

- `N`: 正の整数（ゼロ埋めなし）。`fbim create` が既存ファイルの最大番号 + 1 を割り当てる
- `short-name`: ケバブケースの短い説明（英数字・ハイフンのみ、30文字以内）

**ID はファイル名から取得する。** frontmatter や本文には ID を持たない。

**後方互換**: 旧来の4桁・5桁ゼロ埋め ID（`0042-*.md`, `00042-*.md`）は引き続き読み込める。新規 issue は常にゼロ埋めなしで作成する。

## frontmatter

```markdown
---
status: open
priority: high/medium/low   # 省略可能
area: エリア名              # 省略可能（例: core, cli, docs）
labels: []
---
```

frontmatter は省略可能。省略した場合は `status: unknown`、`priority` は `-`（unknown）、`area` は空（グループなし）として扱う。

`area` と `priority` は frontmatter がある場合でも省略可能。`area` を省略すると issue はグループなし扱いになり、`issues/README.md` では見出しなしで表示される。`priority` を省略すると `medium` がデフォルトになる。

**status の値**

| 値 | 意味 |
|---|---|
| `open` | 対応が必要。作業対象 |
| `pending` | 決定待ち・確認待ち。作業保留 |
| `done` | 完了（`done/` に移動済み） |
| `unknown` | frontmatter がないか parse できない。fbim では書き込めない読み取り専用の状態 |

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
fbim init
fbim create <title> [--slug <slug>] [--priority high|medium|low] [--area <area>] [--body <text>]
fbim done <N>
fbim pending <N>
fbim reopen <N>
fbim list [--status open|pending|done|unknown] [--area <area>] [--label <label>] [--json]
fbim show <N>
fbim completions <bash|zsh|fish>
fbim help [command]
```

`N` はゼロ埋めなしの整数 ID（例: `42`）。レガシーのゼロ埋め ID（例: `00042`）も受け付ける。

## `.fbim.yml` の仕様

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

`fbim` コマンドはカレントディレクトリから上位へ辿り、最初に次のいずれかに該当するディレクトリで止まる。

1. `.fbim.yml` が存在する — `issues_dir` の値を issues ディレクトリとして使う
2. `issues/` サブディレクトリが存在する

何も見つからない場合はカレントディレクトリの `issues/` にフォールバックする。
