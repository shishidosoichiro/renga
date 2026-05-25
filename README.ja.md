# FBIM — File-Based Issue Management

[![pipeline status](https://gitlab.home/kiwi/ifbm/badges/main/pipeline.svg)](https://gitlab.home/kiwi/ifbm/-/pipelines)
[![coverage](https://gitlab.home/kiwi/ifbm/badges/main/coverage.svg)](https://gitlab.home/kiwi/ifbm/-/pipelines)

issue をファイルで管理する CLI ツールです。`fbim create "タイトル"` で Markdown ファイルが作られて、`fbim done 1` で閉じられます。

> English version: [README.md](README.md)

## クイックスタート

```sh
# 1. インストール
bash <(curl -fsSL https://gitlab.home/kiwi/ifbm/-/raw/main/install.sh)

# 2. 初期化（git リポジトリがなくても動く）
fbim init

# 3. 最初の issue を作る
fbim create "最初のタスク"
```

以上。`issues/` ディレクトリにファイルが作られる。

## こんな人に向いている

**ソロ開発者・小規模チーム**で、外部サービスのセットアップなしにすぐ作業を始めたい人向け。

- 新しいプロジェクトを始めたばかりで、まだ GitHub Issues を設定したくない
- ネットワークのない環境やプライベートなマシンで開発している
- Claude Code などの AI ツールを使っていて、ターミナルを離れずに issue 管理をしたい
- issue の変更履歴をコードと同じ git に残したい

コメント・担当者・通知・Web UI が必要なら、GitHub Issues や Linear を使うほうがいい。FBIM は意図的にシンプルに絞っている。

## なぜファイルベースか

- **セットアップ不要**: `fbim init` だけで始まる。アカウントもトークンも設定ファイルも要らない。
- **普通の Markdown**: issue ファイルはエディタで直接開けるし、grep で検索できる。git を使っているなら、履歴も差分もそこに残る。
- **オフラインで動く**: ネットワーク接続がなくても動く。
- **エクスポート不要**: データは手元のファイル。ツールを変えるときにエクスポートは要らない。

## インストール

インストールスクリプトがプラットフォームに合ったバイナリをパッケージレジストリから取得してインストールする。

```sh
bash <(curl -fsSL https://gitlab.home/kiwi/ifbm/-/raw/main/install.sh)
```

ソースからビルドする場合:

```sh
cargo install --path /path/to/fbim
```

## コマンド

| コマンド | 動作 |
|---|---|
| `fbim init` | issues ディレクトリを初期化する |
| `fbim create <タイトル>` | issue を作成する |
| `fbim done <N>` | issue を完了にする |
| `fbim pending <N>` | issue を保留にする |
| `fbim reopen <N>` | issue を再開する |
| `fbim list [--status open\|pending\|done] [--area <area>] [--json]` | issue 一覧を表示する |
| `fbim show <N>` | issue の詳細を表示する |
| `fbim help [コマンド]` | ヘルプを表示する |

```sh
# JSON 出力を jq にパイプする
fbim list --json | jq '.[] | select(.area == "auth")'
```

## fbim が issues/ を探す仕組み

`fbim` コマンドを実行すると、カレントディレクトリからファイルシステムのルートに向かって上位を辿り、最初に次のいずれかに該当するディレクトリで止まる。

1. `.fbim.yml` が存在する — そのファイルの `issues_dir` の値を issues ディレクトリとして使う（デフォルト: `issues`）
2. `issues/` サブディレクトリが存在する

サブディレクトリのどこから実行しても、上位の issues ディレクトリを自動的に見つける。何も見つからない場合は、カレントディレクトリの `issues/` にフォールバックする。

## シェル補完

サブコマンド・フラグ・イシュー番号のタブ補完を有効にする。

**bash** — `~/.bashrc` に追加:

```sh
eval "$(fbim completions bash)"
```

**zsh** — `~/.zshrc` に追加:

```sh
source <(fbim completions zsh)
```

**fish** — 一度だけ実行:

```sh
fbim completions fish > ~/.config/fish/completions/fbim.fish
```

## Claude Code スキル

Claude Code と組み合わせると特に便利。スキルをインストールすれば、コーディング中に `/fbim` で issue を作成・管理できる。

```sh
ln -sf /path/to/fbim/skills/fbim ~/.claude/skills/fbim
```

| コマンド | 動作 |
|---|---|
| `/fbim [create] <タイトル>` | issue を作成する |
| `/fbim done <N>` | 完了にする |
| `/fbim pending <N>` | 保留にする |
| `/fbim reopen <N>` | 再開する |
| `/fbim list` | open/pending の一覧を表示する |
| `/fbim show <N>` | 詳細を表示する |

Claude が作業しながら issue を作り、完了したら close する——コーディングのフローを止めずに issue 管理が回る。

## カスタマイズ

プロジェクトルートに `.fbim.yml` を置く。

```yaml
issues_dir: issues    # デフォルト: issues

area_order:           # 一覧での area の表示順（省略時はアルファベット順）
  - backend
  - frontend
  - infra
  - misc

area_labels:          # area の表示名
  backend: "バックエンド"
  frontend: "フロントエンド"
  infra: "インフラ"
  misc: "その他"
```

## 開発

```sh
cargo test            # テスト実行
cargo test --doc      # doctest 実行
cargo clippy -- -D warnings
cargo fmt --check
cargo doc --no-deps --open
```

ファイル形式と命名規則の仕様は [spec.ja.md](spec.ja.md) を参照。
