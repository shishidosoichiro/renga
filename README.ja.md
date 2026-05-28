# FBIM — File-Based Issue Management

[![CI](https://github.com/shishidosoichiro/fbim/actions/workflows/ci.yml/badge.svg)](https://github.com/shishidosoichiro/fbim/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

issue をファイルで管理する CLI ツールです。`fbim create "タイトル"` で Markdown ファイルが作られて、`fbim done 1` で閉じられます。

> English version: [README.md](README.md)

## クイックスタート

```sh
# 1. インストール
bash <(curl -fsSL https://raw.githubusercontent.com/shishidosoichiro/fbim/main/install.sh)

# 2. 初期化（git リポジトリがなくても動く）
fbim init

# 3. 最初の issue を作る
fbim create "最初のタスク"
```

以上。`issues/` ディレクトリにファイルが作られる。

## Claude Code integration

Claude が作業しながら issue を作り、完了したら close する——コーディングのフローを止めずに issue 管理が回る。

スキルをインストールすれば、Claude Code の中で `/fbim` を使って issue を管理できる。

```sh
ln -sf /path/to/fbim/skills/fbim ~/.claude/skills/fbim
```

あとは Claude Code セッションの中で直接使うだけ。

```
/fbim create "入力バリデーションを追加する"
/fbim list
/fbim done 3
```

| コマンド | 動作 |
|---|---|
| `/fbim [create] <タイトル>` | issue を作成する |
| `/fbim done <N>` | 完了にする |
| `/fbim pending <N>` | 保留にする |
| `/fbim reopen <N>` | 再開する |
| `/fbim list` | open/pending の一覧を表示する |
| `/fbim show <N>` | 詳細を表示する |

## こんな人に向いている

**ソロ開発者・小規模チーム**で、外部サービスのセットアップなしにすぐ作業を始めたい人向け。

- Claude Code などの AI ツールを使っていて、ターミナルを離れずに issue 管理をしたい
- GitHub Issues を設定する前のプロジェクトで使いたい
- ネットワークのない環境やプライベートなマシンで開発している
- issue の変更履歴をコードと同じ git に残したい

コメント・担当者・通知・Web UI が必要なら、GitHub Issues や Linear を使うほうがいい。FBIM は意図的にシンプルに絞っている。

## 何が違うのか

- **AI ネイティブ**: issue ファイルは普通の Markdown なので、LLM がそのまま読み書きできる。エージェントが issue を開き、修正して、close する——一つのセッションで完結する。
- **オフラインで動く**: ネットワーク接続もアカウントも API トークンも不要。`fbim init` だけで始められる。
- **設定不要**: どのディレクトリにも置くだけで使える。プロジェクト設定も外部サービスも要らない。
- **コードと一緒に暮らす**: issue ファイルはただのファイル。好きなエディタで開けて、grep で検索できて、修正コードと一緒に git にコミットできる。
- **データは手元に**: エクスポートは要らない。fbim を使い続けても使わなくなっても、ファイルはそのまま読める。

## インストール

インストールスクリプトがプラットフォームに合ったバイナリをパッケージレジストリから取得してインストールする。

```sh
bash <(curl -fsSL https://raw.githubusercontent.com/shishidosoichiro/fbim/main/install.sh)
```

ソースからビルドする場合:

```sh
cargo install --path /path/to/fbim
```

## コマンド

| コマンド | 動作 |
|---|---|
| `fbim init` | issues ディレクトリを初期化する |
| `fbim create <タイトル> [--id <N>] [--slug <slug>] [--priority high\|medium\|low] [--area <area>] [--body <テキスト\|-\>] [--milestone <milestone>]` | issue を作成する（`--body -` で標準入力から本文を読む） |
| `fbim done <N>` | issue を完了にする |
| `fbim pending <N>` | issue を保留にする |
| `fbim reopen <N>` | issue を再開する |
| `fbim list [--status open\|pending\|done\|unknown] [--area <area>] [--label <label>] [--milestone <milestone>] [--json]` | issue 一覧を表示する |
| `fbim show <N>` | issue の詳細を表示する |
| `fbim validate` | 全 issue のスキーマエラー・ID 重複を検出する |
| `fbim completions bash\|zsh\|fish` | シェル補完スクリプトを表示する |
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
