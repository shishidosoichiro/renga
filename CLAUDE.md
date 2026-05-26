@CONTRIBUTING.md

# FBIM — Rust 実装ガイド

FBIM (File-Based Issue Management) を Rust で実装する。
Python (`bin/fbim`) と bash (`bin/next-id`, `bin/gen-issues-readme`) を単一の Rust バイナリ `fbim` に置き換える。
既存の issue ファイル形式・CLI インターフェースとの後方互換を保つ。

## コマンド仕様

既存 Python 実装と同一インターフェース。詳細は `spec.ja.md` を参照。

```
fbim create <title> [--slug <slug>] [--priority high|medium|low] [--area <area>] [--body <text>]
fbim done <NNNNN>
fbim pending <NNNNN>
fbim reopen <NNNNN>
fbim list [--status open|pending|done] [--area <area>] [--label <label>] [--json]
fbim show <NNNNN>
fbim help [command]
```

## クレート選定

| 用途 | クレート |
|---|---|
| CLI 引数パース | `clap` (derive feature) |
| アプリエラーハンドリング | `anyhow` |
| ドメインエラー定義 | `thiserror` |
| YAML フロントマター | `serde` + `serde_yaml` |
| ディレクトリ走査 | `walkdir` |
| 正規表現 | `regex` |
| テスト用一時ディレクトリ | `tempfile` |

`serde_yaml` は frontmatter ブロックのみに使う。`.fbim.yml` の設定読み込みにも同クレートを使う。

## プロジェクト構造

```
src/
  main.rs          # エントリポイント。clap でサブコマンドをディスパッチする
  lib.rs           # クレートルート。#![deny(missing_docs)] を置く
  cli.rs           # clap の Derive 定義（Cli / Subcommand enum）
  project.rs       # プロジェクトルート探索（.fbim.yml / issues/ を上位へ辿る）
  config.rs        # .fbim.yml の読み込みと Config 構造体
  issue.rs         # Issue 構造体・フロントマターのパース・ファイル検索
  readme.rs        # issues/README.md の生成
  commands/
    mod.rs
    create.rs
    done.rs
    pending.rs
    reopen.rs
    list.rs
    show.rs
```

`next-id` と `gen-issues-readme` の機能は `fbim` バイナリ内部に統合する。
外部サブプロセス呼び出しは不要。

## エラーハンドリング

- コマンド関数の戻り値は `anyhow::Result<()>`
- ドメインエラー（issue が見つからないなど）は `thiserror` で定義する
- `unwrap()` / `expect()` はテストコード以外で使わない
- ユーザー向けエラーメッセージは `error:` プレフィックスで stderr に出力し、exit code 1 で終了する

```rust
#[derive(Debug, thiserror::Error)]
pub enum FbimError {
    #[error("issue {0} not found")]
    IssueNotFound(String),
    #[error("issues directory not found (run 'mkdir -p issues/done')")]
    IssuesDirNotFound,
}
```

## コードスタイル

- `rustfmt` デフォルト設定を使う（`cargo fmt`）
- `clippy` 警告をすべて解消する（`cargo clippy -- -D warnings`）
- コメントは WHY が自明でない箇所にのみ書く。WHAT を説明するコメントは書かない

## ドキュメント

### ソースコードの doc コメント

OSS プロジェクトとして不特定多数のコントリビューターが読む前提で書く。

- `src/lib.rs` に `#![deny(missing_docs)]` を置く。公開アイテムへの doc コメント漏れをコンパイルエラーにする
- モジュール冒頭に `//!` でそのモジュールの責務を一言で書く
- 公開 `struct` / `enum` / `fn` / `trait` にはすべて `///` で doc コメントを付ける
- doc コメントには `# Examples` セクションを書き、`cargo test --doc` で実行できる doctest にする（実装が固まってから追加する）
- 非公開アイテムへのコメントは WHY が自明でない場合のみ `//` で書く

```rust
//! Issue ファイルのパースと操作。

/// YAML フロントマターを持つ issue ファイルを表す。
pub struct Issue {
    /// ゼロ埋め5桁の ID（例: "00042"）。
    pub id: String,
    ...
}

impl Issue {
    /// frontmatter の `status` を `done` に更新し、`done/` ディレクトリへ移動する。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let issue = Issue::load("issues/00042-foo.md")?;
    /// issue.close()?;
    /// ```
    pub fn close(&self) -> anyhow::Result<()> { ... }
}
```

### プロジェクトドキュメント（ファイル）

| ファイル | 役割 | 更新タイミング |
|---|---|---|
| `README.md` | インストール・使い方・設定の概要 | インターフェース変更時 |
| `CONTRIBUTING.md` | 開発環境のセットアップ・PR ガイドライン | 開発フロー変更時 |
| `CHANGELOG.md` | Keep a Changelog 形式のリリースノート | リリース前 |
| `spec.md` / `spec.ja.md` | issue ファイル形式の仕様（既存） | 仕様変更時 |

`README.md` には以下を含める:
- インストール方法（`cargo install` / バイナリ配布）
- クイックスタート（5ステップ以内で動くまで）
- コマンド一覧表
- `.fbim.yml` による設定例
- Claude Code スキルのインストール手順

`CONTRIBUTING.md` には以下を含める:
- 開発環境の前提条件（Rust toolchain のバージョン）
- `cargo test` / `cargo clippy` / `cargo fmt` の実行方法
- issue の作り方・PR の出し方
- コードレビューで見るポイント

### `cargo doc`

```sh
cargo doc --no-deps --open   # ローカルで API ドキュメントを確認する
```

CI で `cargo doc --no-deps` を実行し、ドキュメント生成エラーを検出する。

## テスト方針

- ユニットテストはロジックのある各モジュールに `#[cfg(test)]` ブロックで書く
- ファイルシステムを伴うテストは `tempfile::TempDir` を使う。モックは使わない
- 統合テストは `tests/` に置く（`cargo test` で実行される）
- テスト名は `snake_case` で、何をテストするかが読んでわかる名前にする

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn parse_frontmatter_returns_status_field() {
        let content = "---\nstatus: open\n---\n\n# Title\n";
        let fm = parse_frontmatter(content).unwrap();
        assert_eq!(fm.status, Status::Open);
    }
}
```

## 後方互換

- issue ファイルの 4 桁 ID（`NNNN-*.md`）と 5 桁 ID（`NNNNN-*.md`）の両方を読み込む
- 新規作成は常に 5 桁

## 既存実装との対応

Python 実装を読み解いてゼロから書き直す。既存コードを Rust に逐語訳しない。
Rust らしい構造（`Result`/`Option`、イテレータ、`From` トレイトなど）を自然に使う。

## ドキュメント更新ルール

コードを変更したときは、影響するドキュメントを必ず同じコミットまたは直後のコミットで更新する。

| 変更の種類 | 更新が必要なドキュメント |
|---|---|
| CLI の動作・引数・出力形式 | `README.md`, `README.ja.md` |
| issue ファイルの形式・ID・タイトルの仕様 | `spec.md`, `spec.ja.md` |
| 公開 struct / enum / fn | `src/` の doc コメント（`///`） |
| リリース | `CHANGELOG.md`, `Cargo.toml` のバージョン |
| 開発フロー・規約 | `CONTRIBUTING.md` |

英語版と日本語版（`README.md` / `README.ja.md`、`spec.md` / `spec.ja.md`）は常に同期する。片方だけ更新しない。

## ビルドと CI

```sh
cargo build --release     # リリースビルド
cargo test                # テスト
cargo fmt --check         # フォーマット確認
cargo clippy -- -D warnings  # lint
```

CI は既存の `.gitlab-ci.yml` を Rust 向けに書き換える。

## Issue 管理

このリポジトリ自体の issue は FBIM で管理する（自己ホスト）。

```sh
fbim create "タイトル" --area <area>   # issue を作成する
fbim list                              # open/pending の一覧を確認する
fbim done <NNNNN>                      # issue を完了にする
```

Claude Code スキル（`/fbim`）でも同じ操作ができる。スキルは以下でインストールする。

```sh
ln -sf /path/to/fbim/skills/fbim ~/.claude/skills/fbim
```

area の目安:

| area | 使うとき |
|---|---|
| `cli` | コマンドライン引数・ヘルプ表示 |
| `core` | issue のパース・ファイル操作・プロジェクトルート探索 |
| `config` | `.fbim.yml` の読み込み・設定 |
| `test` | テストの追加・修正 |
| `docs` | ドキュメント・README・CONTRIBUTING |
| `ci` | CI/CD パイプライン |
| `misc` | 上記に当てはまらないもの |
