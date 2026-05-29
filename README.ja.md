# Renga — File-Based Issue Management

[![CI](https://github.com/shishidosoichiro/renga/actions/workflows/ci.yml/badge.svg)](https://github.com/shishidosoichiro/renga/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/renga.svg)](https://crates.io/crates/renga)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

renga は issue をファイルで管理する CLI ツールです。普通の Markdown なので、エディタ・grep・git・AI エージェントがそのまま読み書きできます。`renga create "タイトル"` で issue を作成し、`renga done 1` で閉じます。

> English version: [README.md](README.md)

## クイックスタート

```sh
# 1. インストール
bash <(curl -fsSL https://raw.githubusercontent.com/shishidosoichiro/renga/main/install.sh)

# 2. 初期化（git リポジトリがなくても動く）
renga init

# 3. 最初の issue を作る
renga create "最初のタスク"
```

以上。`issues/` ディレクトリにファイルが作られる。

## AI エージェントとの推奨ワークフロー

```text
1. タスクの issue を作る
   renga create "入力バリデーションを追加する" --area core

2. エージェントが作業前に renga list で確認する

3. 作業中に判断・制約を issue 本文の ## Notes に追記する

4. 実装後に renga validate を実行する

5. 修正コミットと一緒に issue を close する
   renga done 1
   git add issues/ src/
   git commit -m "feat: 入力バリデーションを追加 (#1)"
```

issue ファイルとコードの変更が同じコミットに入る——git の履歴が全体像を語る。

## Claude Code integration

Claude が作業しながら issue を作り、完了したら close する——コーディングのフローを止めずに issue 管理が回る。

スキルをインストールすれば、Claude Code の中で `/renga` を使って issue を管理できる。

```sh
# Node.js がある場合
npx skills add shishidosoichiro/renga

# Node.js がない場合
mkdir -p ~/.claude/skills/renga
curl -fsSL https://raw.githubusercontent.com/shishidosoichiro/renga/main/skills/renga/SKILL.md \
  -o ~/.claude/skills/renga/SKILL.md
```

あとは Claude Code セッションの中で直接使うだけ。

```
/renga create "入力バリデーションを追加する"
/renga list
/renga done 3
```

| コマンド | 動作 |
|---|---|
| `/renga [create] <タイトル>` | issue を作成する |
| `/renga done <N>` | 完了にする |
| `/renga pending <N>` | 保留にする |
| `/renga reopen <N>` | 再開する |
| `/renga list` | open/pending の一覧を表示する |
| `/renga show <N>` | 詳細を表示する |

## GitHub Issues を使う前に

**ソロ開発者・小規模チーム**で、外部サービスのセットアップなしにすぐ作業を始めたい人向け。

- Claude Code などの AI ツールを使っていて、ターミナルを離れずに issue 管理をしたい
- GitHub Issues を設定する前のプロジェクトで使いたい
- ネットワークのない環境やプライベートなマシンで開発している
- issue の変更履歴をコードと同じ git に残したい

コメント・担当者・通知・Web UI が必要なら、GitHub Issues や Linear を使うほうがいい。Renga は意図的にシンプルに絞っている。

## 何が違うのか

- **AI ネイティブ**: issue ファイルは普通の Markdown なので、LLM がそのまま読み書きできる。エージェントが issue を開き、修正して、close する——一つのセッションで完結する。
- **オフラインで動く**: ネットワーク接続もアカウントも API トークンも不要。`renga init` だけで始められる。
- **設定不要**: どのディレクトリにも置くだけで使える。プロジェクト設定も外部サービスも要らない。
- **コードと一緒に暮らす**: issue ファイルはただのファイル。好きなエディタで開けて、grep で検索できて、修正コードと一緒に git にコミットできる。
- **データは手元に**: エクスポートは要らない。renga を使い続けても使わなくなっても、ファイルはそのまま読める。

## インストール

インストールスクリプトがプラットフォームに合ったバイナリを GitHub Releases から取得してインストールする。

```sh
bash <(curl -fsSL https://raw.githubusercontent.com/shishidosoichiro/renga/main/install.sh)
```

ソースからビルドする場合:

```sh
cargo install renga
```

## コマンド

| コマンド | 動作 |
|---|---|
| `renga init` | issues ディレクトリを初期化する |
| `renga create <タイトル> [--id <N>] [--slug <slug>] [--priority high\|medium\|low] [--area <area>] [--body <テキスト\|-\>] [--milestone <milestone>] [--label <label>]...` | issue を作成する（`--body -` で標準入力から本文を読む） |
| `renga done <N>` | issue を完了にする |
| `renga pending <N>` | issue を保留にする |
| `renga reopen <N>` | issue を再開する |
| `renga list [--status open\|pending\|done\|unknown] [--area <area>] [--label <label>] [--milestone <milestone>] [--json]` | issue 一覧を表示する |
| `renga show <N> [--json]` | issue の詳細を表示する |\n| `renga edit <N>` | `$EDITOR` で issue を開く（人間向け） |\n| `renga update <N> [--priority ...] [--area ...] [--status ...] [--milestone ...] [--label ...]... [--body <テキスト\\|->]` | issue のフィールドを更新する（AI・スクリプト向け） |
| `renga validate` | 全 issue のスキーマエラー・ID 重複を検出する |
| `renga completions bash\|zsh\|fish` | シェル補完スクリプトを表示する |
| `renga help [コマンド]` | ヘルプを表示する |

```sh
# JSON 出力を jq にパイプする
renga list --json | jq '.[] | select(.area == "auth")'
```

## renga が issues/ を探す仕組み

`renga` コマンドを実行すると、カレントディレクトリからファイルシステムのルートに向かって上位を辿り、最初に次のいずれかに該当するディレクトリで止まる。

1. `.renga.yml` が存在する — そのファイルの `issues_dir` の値を issues ディレクトリとして使う（デフォルト: `issues`）
2. `issues/` サブディレクトリが存在する

サブディレクトリのどこから実行しても、上位の issues ディレクトリを自動的に見つける。何も見つからない場合は、カレントディレクトリの `issues/` にフォールバックする。

## シェル補完

サブコマンド・フラグ・イシュー番号のタブ補完を有効にする。

**bash** — `~/.bashrc` に追加:

```sh
eval "$(renga completions bash)"
```

**zsh** — `~/.zshrc` に追加:

```sh
source <(renga completions zsh)
```

**fish** — 一度だけ実行:

```sh
renga completions fish > ~/.config/fish/completions/renga.fish
```

## カスタマイズ

プロジェクトルートに `.renga.yml` を置く。

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
