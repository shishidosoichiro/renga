# FBIM — File-Based Issue Management

ファイルで issue を管理するシステム。GitHub Issues や Redmine のような外部ツールを使わず、Git リポジトリ内のファイルだけで issue を追跡する。

## なぜファイルベースか

- **Git で完結する**: issue の履歴・変更者・コメントがすべて git log に残る。外部サービスへの依存がない
- **コードと並走できる**: issue ファイルをコード変更と同じ PR に含められる。「この修正はこの issue を解決する」という対応関係が一目でわかる
- **オフラインで動く**: ネットワーク接続なしに issue を作成・更新・参照できる
- **移行が容易**: Redmine や GitHub Issues に移行するときも、ファイルを読めば全データが手元にある。ロックインがない
- **ツールを選ばない**: エディタでも、スクリプトでも、AI ツールでも操作できる。インターフェースがファイルなので何とでも組み合わせられる

## 動作要件

- Python 3.8 以上（`bin/fbim`・`bin/gen-issues-readme` に必要）
- bash（`bin/next-id` に必要）
- PyYAML（`.fbim.yml` による設定をカスタマイズする場合のみ。`pip install pyyaml`）

## アクション

### issue を作成する

`issues/NNNN-short-name.md` を作成する。`NNNN` は連番。frontmatter に `status`・`priority`・`area` を付ける。

```
issues/0042-api-auth-missing-scope.md
```

テンプレートや命名規則の詳細は [spec.md](spec.md) を参照。

### issue を更新する

対象ファイルを直接編集する。本文・frontmatter のどちらも自由に変更できる。

### issue を完了にする

`issues/NNNN-*.md` を `issues/done/NNNN-*.md` に移動し、frontmatter の `status` を `done` に変更する。

### issue を保留にする

frontmatter の `status` を `pending` に変更する。決定待ち・後回しにしたい issue に使う。

### issue を再開する

`issues/done/NNNN-*.md` を `issues/NNNN-*.md` に移動し、frontmatter の `status` を `open` に変更する。

### 一覧を確認する

`issues/README.md` に open/pending の issue 一覧が生成される。手動編集は禁止。`bin/gen-issues-readme` を実行して再生成する。

## ツール

### bin/ CLI

`bin/fbim` をプロジェクトの PATH に追加するか、フルパスで呼び出す。

```sh
export PATH="$PATH:/path/to/fbim/bin"
```

| コマンド | 動作 |
|---|---|
| `fbim create <title>` | issue を作成する |
| `fbim done <NNNN>` | issue を完了にする |
| `fbim pending <NNNN>` | issue を保留にする |
| `fbim reopen <NNNN>` | issue を再開する |
| `fbim list [--json]` | issue 一覧を表示する（`--json` で JSON 出力） |
| `fbim show <NNNN>` | issue の詳細を表示する |
| `fbim help [コマンド]` | ヘルプを表示する |

`fbim list --json` の出力は `yq` や `jq` にパイプできる。

```sh
fbim list --json | jq '.[] | select(.area == "authz")'
```

### Claude Code スキル

`skills/fbim/` に Claude Code 用のスキルを同梱している。シンボリックリンクでインストールする。

```sh
ln -s /path/to/fbim/skills/fbim ~/.claude/skills/fbim
```

インストール後は以下のコマンドが使えるようになる。

| コマンド | 動作 |
|---|---|
| `/fbim タイトル` | issue を作成する |
| `/fbim done NNNN` | issue を完了にする |
| `/fbim pending NNNN` | issue を保留にする |
| `/fbim reopen NNNN` | 完了した issue を再開する |
| `/fbim help` | ヘルプを表示する |

## カスタマイズ

プロジェクトルートに `.fbim.yml` を置くと、issue 一覧の area 表示をカスタマイズできる。

```yaml
area_order:
  - backend
  - frontend
  - infra
  - misc

area_labels:
  backend: "バックエンド"
  frontend: "フロントエンド"
  infra: "インフラ"
  misc: "その他"
```

`.fbim.yml` がない場合は area 名をそのまま使い、アルファベット順で表示する。
