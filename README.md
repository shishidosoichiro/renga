# FBIM — File-Based Issue Management

ファイルで issue を管理するシステム。GitHub Issues や Redmine のような外部ツールを使わず、Git リポジトリ内のファイルだけで issue を追跡する。

## なぜファイルベースか

- **Git で完結する**: issue の履歴・変更者・コメントがすべて git log に残る。外部サービスへの依存がない
- **コードと並走できる**: issue ファイルをコード変更と同じ PR に含められる。「この修正はこの issue を解決する」という対応関係が一目でわかる
- **オフラインで動く**: ネットワーク接続なしに issue を作成・更新・参照できる
- **移行が容易**: Redmine や GitHub Issues に移行するときも、ファイルを読めば全データが手元にある。ロックインがない
- **ツールを選ばない**: エディタでも、スクリプトでも、AI ツールでも操作できる。インターフェースがファイルなので何とでも組み合わせられる

## アクション

### issue を作成する

`issues/NNNN-short-name.md` を作成する。`NNNN` は連番。frontmatter に `status`・`priority`・`area` を付ける。

```
issues/0042-authz-policy-missing-context.md
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

### bin/ スクリプト

`bin/` にシェルスクリプトと Python スクリプトを同梱している。使う側のリポジトリの `bin/` にシンボリックリンクを張って使う。

```sh
ln -s ~/kiwi/fbim/bin/next-id bin/next-id
ln -s ~/kiwi/fbim/bin/gen-issues-readme bin/gen-issues-readme
```

| スクリプト | 動作 |
|---|---|
| `bin/next-id issues/` | 次の issue 番号をゼロ埋め4桁で出力する |
| `bin/gen-issues-readme` | `issues/README.md` を再生成する |

### Claude Code スキル

`skills/fbim/` に Claude Code 用のスキルを同梱している。シンボリックリンクでインストールする。

```sh
ln -s ~/kiwi/fbim/skills/fbim ~/.claude/skills/fbim
```

インストール後は以下のコマンドが使えるようになる。

| コマンド | 動作 |
|---|---|
| `/fbim タイトル` | issue を作成する |
| `/fbim done NNNN` | issue を完了にする |
| `/fbim pending NNNN` | issue を保留にする |
| `/fbim reopen NNNN` | 完了した issue を再開する |
