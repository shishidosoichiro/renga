@CONTRIBUTING.md

# FBIM — 実装ガイド

## エージェント活用方針

| コンテキスト | トリガー | 指示ファイル |
|---|---|---|
| コード品質・仕様・ドキュメントのレビュー | `Agent(subagent_type="review")` | `.claude/agents/review.md` |
| 自己改善 | `Agent(subagent_type="self-improve")` ※事前に retro issue を起票する | `.claude/agents/self-improve.md` |
| OSS ポジショニング・ローンチ計画 | `Agent(subagent_type="marketing-strategist")` | `.claude/agents/marketing-strategist.md` |
| OSS 公開用ドキュメント執筆・改善 | `Agent(subagent_type="docs-writer")` | `.claude/agents/docs-writer.md` |
| OSS ローンチ実行（投稿文・公開順序） | `Agent(subagent_type="launch-orchestrator")` | `.claude/agents/launch-orchestrator.md` |

**`.claude/` の変更は self-improve 経由のみ**: `CLAUDE.md`・`.claude/agents/` を変更する場合は、必ず retro issue（`area: agent, labels: [retro]`）を起票してから `self-improve` エージェントを呼ぶ。「局所的な変更だから直接やる」という判断は行わない。

File-Based Issue Management。詳細仕様は `spec.ja.md` を参照。
`skills/` は Claude Code スキルの配布用ディレクトリ（ユーザーが `~/.claude/skills/` にシンボリックリンクして使う）。インストール方法・コマンド一覧は `README.md` の "Claude Code skill" セクションを参照。

## エラーハンドリング

- `unwrap()` / `expect()` はテストコード以外で使わない
- ユーザー向けエラーメッセージは `error:` プレフィックスで stderr に出力し、exit code 1 で終了する
- ドメインエラーは `thiserror`、アプリエラーは `anyhow` で扱う

## ドキュメント

- `src/lib.rs` に `#![deny(missing_docs)]` を置く。公開アイテムへの doc コメント漏れをコンパイルエラーにする
- 公開アイテムには `///` で doc コメントを付け、`# Examples` セクションを書いて doctest にする
- 非公開アイテムは WHY が自明でない場合のみ `//` で書く

## テスト方針

- ファイルシステムを伴うテストは `tempfile::TempDir` を使う。モックは使わない
- 統合テストは `tests/` に置く

## 後方互換

- issue ファイルの 4 桁・5 桁のゼロ埋め ID（`NNNN-*.md`、`NNNNN-*.md`）は読み込めるが、新規作成はゼロ埋めなしの整数（`1`、`42` など）

## ドキュメント更新ルール

コードを変更したときは、影響するドキュメントを必ず同じコミットまたは直後のコミットで更新する。

| 変更の種類 | 更新が必要なドキュメント |
|---|---|
| CLI の動作・引数・出力形式 | `README.md`, `README.ja.md` |
| issue ファイルの形式・ID・タイトルの仕様 | `spec.md`, `spec.ja.md` |
| 公開 struct / enum / fn | `src/` の doc コメント（`///`） |
| リリース | `CHANGELOG.md`（git-cliff で生成）, `Cargo.toml` のバージョン |
| 開発フロー・規約 | `CONTRIBUTING.md` |

英語版と日本語版（`README.md` / `README.ja.md`、`spec.md` / `spec.ja.md`）は常に同期する。片方だけ更新しない。

## 実装フロー

コードを変更するときは以下の順序で進める。

1. 実装 + テスト追加
2. カバレッジ確認（`cargo llvm-cov --summary-only -- --test-threads=1`）
3. `Agent(subagent_type="review")` でレビューを受ける（Claude Code のサブエージェント）。レビュー観点にはカバレッジ確認（`cargo llvm-cov --summary-only -- --test-threads=1`）を含める
4. レビューで見つかった問題は `fbim create` で起票してから修正する
5. 指摘を反映してからコミット
6. issue を close する（`fbim done <N>`）

カバレッジは実装時とレビュー時の両方で確認する。

**変更の順序**: 新規追加を先にコミット・確認してから、削除や簡略化を行う（追加 → 確認 → 破壊）。

## 判断方針

- **ベストプラクティスを先に調べる**: 実装・設計の判断を述べる前に、WebSearch / WebFetch でベストプラクティスを調べる。知っているつもりで進めない。自分の推論だけに頼らず、実践の中で検証された方法（RFC・仕様書・設計パターン・公式ドキュメント等）を参照する。
- **容赦なく指摘・提案・批判する**: ユーザーの選択・意見に迎合しない。問題があれば全コンテキストで指摘する。

## Issue 管理

このリポジトリ自体の issue は FBIM で管理する（自己ホスト）。

```sh
fbim create "タイトル" --area <area>
fbim list
fbim done <N>
```

`/fbim` スキルでも同じ操作ができる。

| area | 使うとき |
|---|---|
| `cli` | コマンドライン引数・ヘルプ表示 |
| `core` | issue のパース・ファイル操作・プロジェクトルート探索 |
| `config` | `.fbim.yml` の読み込み・設定 |
| `test` | テストの追加・修正 |
| `docs` | ドキュメント・README・CONTRIBUTING |
| `ci` | CI/CD パイプライン |
| `agent` | CLAUDE.md・`.claude/agents/` の変更・retro issue |
| `misc` | 上記に当てはまらないもの |
