# FBIM 仕様

File-Based Issue Management の仕様。

---

## ディレクトリ構造

```
issues/
  README.md           一覧（自動生成。手動編集不可）
  NNNNN-name.md       open または pending な issue
  done/
    NNNNN-name.md     完了した issue
```

## ファイル命名規則

```
NNNNN-short-name.md
```

- `NNNNN`: ゼロ埋め5桁の連番。次番号は `bin/next-id issues/` で取得する
- `short-name`: ケバブケースの短い説明（英数字・ハイフンのみ）
- 作成前に既存 issues を確認し、同等の内容が存在する場合は新規作成しない

**後方互換**: 旧来の4桁 ID（`NNNN-*.md`）は引き続きサポートする。すべてのツールは両形式を読み込む。新規 issue は常に5桁で作成する。

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
area: エリア名（例: authz, authn, test, docs）
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
- issue を新規作成・`done/` に移動・status 変更のたびに実行する

## アクション

### issue を作成する

1. `bin/next-id issues/` で次番号を取得する
2. タイトルからケバブケースの short-name を作る
3. `issues/NNNNN-short-name.md` をテンプレートに従って作成する
4. `bin/gen-issues-readme` を実行する

### issue を完了にする

1. `issues/NNNNN-*.md` を `issues/done/NNNNN-*.md` に移動する
2. frontmatter の `status` を `done` に変更する
3. `bin/gen-issues-readme` を実行する

### issue を保留にする

1. `issues/NNNNN-*.md` の frontmatter `status` を `pending` に変更する
2. `bin/gen-issues-readme` を実行する

### issue を再開する

1. `issues/done/NNNNN-*.md` を `issues/NNNNN-*.md` に移動する
2. frontmatter の `status` を `open` に変更する
3. `bin/gen-issues-readme` を実行する

### issue の内容を更新する

対象ファイルを直接編集する。

---

## `bin/next-id` の仕様

`bin/next-id <dir>` は指定ディレクトリ（および `done/` サブディレクトリ）の issue ファイルを走査し、最大番号 + 1 をゼロ埋め5桁で出力する。ファイルが存在しない場合は `00001` を返す。旧来の4桁ファイルも検出する。

## `bin/gen-issues-readme` の仕様

`issues/` と `issues/done/` の全ファイルを走査し、frontmatter の `status` / `priority` / `area` を読み取って `issues/README.md` を再生成する。open と pending を先に、done を後に列挙する。

area の表示順・表示名はプロジェクトルートの `.fbim.yml` で設定できる。`.fbim.yml` がない場合は area 名をそのまま使い、アルファベット順で表示する。

## `.fbim.yml` の仕様

プロジェクトルートに置く設定ファイル。省略可能。PyYAML が必要（`pip install pyyaml`）。

```yaml
area_order:       # issue 一覧での area の表示順（省略時はアルファベット順）
  - backend
  - frontend
  - misc

area_labels:      # area の表示名（省略時は area 名をそのまま使う）
  backend: "バックエンド"
  frontend: "フロントエンド"
  misc: "その他"
```
