@CONTRIBUTING.md

# Renga — 実装ガイド

## エージェント活用方針

| コンテキスト | トリガー | 指示ファイル |
|---|---|---|
| コード品質・仕様・ドキュメントのレビュー | `Agent(subagent_type="review")` | `.claude/agents/review.md` |
| 自己改善 | `Agent(subagent_type="self-improve")` ※事前に retro issue を起票する | `.claude/agents/self-improve.md` |
| OSS ポジショニング・ローンチ計画 | `Agent(subagent_type="marketing-strategist")` | `.claude/agents/marketing-strategist.md` |
| OSS 公開用ドキュメント執筆・改善 | `Agent(subagent_type="docs-writer")` | `.claude/agents/docs-writer.md` |
| OSS ローンチ実行（投稿文・公開順序） | `Agent(subagent_type="launch-orchestrator")` | `.claude/agents/launch-orchestrator.md` |

**`.claude/` の変更は self-improve 経由のみ**: `CLAUDE.md`・`.claude/agents/` を変更する場合は、必ず retro issue（`area: agent, labels: [retro]`）を起票してから `self-improve` エージェントを呼ぶ。「局所的な変更だから直接やる」という判断は行わない。

**self-improve を呼ぶタイミング**:
- 宍戸さんにミスや改善を指摘されたとき → その場で retro issue を起票して self-improve を呼ぶ
- セッション内で同じ種類のミスが2回以上起きたとき

**Plan モードをいつ使うか**:
- 3ファイル以上を変更するタスク
- breaking change を含むタスク
- 設計判断が複数ある場合（例: API の設計、ディレクトリ構造の変更）

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
3. **コミット前に** `Agent(subagent_type="review")` でレビューを受ける（Claude Code のサブエージェント）。レビュー観点にはカバレッジ確認（`cargo llvm-cov --summary-only -- --test-threads=1`）を含める
4. レビュー指摘を分類する（判定基準: 「このバグは今回変更したコードに由来するか？」）
   - **今回の実装で入ったバグ**（新規追加ファイルや今回変更した箇所に起因）→ コミット前に修正し feature コミットに含める。issue 化する場合は `found_in_impl` ラベルを付ける
   - **以前のバージョンから存在していたバグ** → コミット後に別の `fix:` コミットで修正する。issue 化する場合は `found_at:X.Y.Z`（バグが混入したバージョン）ラベルを付け、修正時に `fixed_at:X.Y.Z` ラベルを付けてクローズする
5. 今回の実装で入ったバグを修正する
6. コミット（feature + 同一実装内バグ修正をまとめて）
7. 以前からあったバグがあれば `renga create` で起票してから別の `fix:` コミットで修正する
8. issue を close する（`renga done <N>`）

カバレッジは実装時とレビュー時の両方で確認する。

**コミット前・issue close 前の自問**: 「このコードをスタッフエンジニアがレビューしたら承認するか？」と自問する。feat と fix が混在していないか、テストが不十分でないか、ドキュメントが更新されているかを確認する。

**変更の順序**: 新規追加を先にコミット・確認してから、削除や簡略化を行う（追加 → 確認 → 破壊）。

**コミットメッセージの言語**: `type`・`scope`・`description` はすべて英語で書く（`Use English for the description` — CONTRIBUTING.md 参照）。issue ファイルのタイトル・本文、CLAUDE.md、`.claude/agents/` は日本語でよい。

**コミット粒度の規律**（retro #134）:

- `feat:` と `fix:` を同一コミットに混ぜない
- レビューで指摘された修正は、feat コミットには含めず必ず別の `fix:` コミットにする
- 「ついでに修正」禁止: 本来のタスクと無関係な変更が見つかったら `renga create` で別 issue を起票し、別コミットで対処する
- 複数の breaking change は別々のコミットに分ける

## 判断方針

- **ベストプラクティスを先に調べる**: 実装・設計の判断を述べる前に、WebSearch / WebFetch でベストプラクティスを調べる。知っているつもりで進めない。自分の推論だけに頼らず、実践の中で検証された方法（RFC・仕様書・設計パターン・公式ドキュメント等）を参照する。
- **容赦なく指摘・提案・批判する**: 宍戸さんの選択・意見に迎合しない。問題があれば全コンテキストで指摘する。
- **根本原因を特定する**: エラーや問題を回避するのではなく、根本原因を特定して解決する。`--no-verify`・`#[allow(...)]`・コンパイルエラーを黙らせる回避策は使わない。

## Issue 管理

このリポジトリ自体の issue は Renga で管理する（自己ホスト）。

```sh
renga create "タイトル" --area <area>
renga list
renga done <N>
```

`/renga` スキルでも同じ操作ができる。

| area | 使うとき |
|---|---|
| `cli` | コマンドライン引数・ヘルプ表示 |
| `core` | issue のパース・ファイル操作・プロジェクトルート探索 |
| `config` | `.renga.yml` の読み込み・設定 |
| `test` | テストの追加・修正 |
| `docs` | ドキュメント・README・CONTRIBUTING |
| `ci` | CI/CD パイプライン |
| `agent` | CLAUDE.md・`.claude/agents/` の変更・retro issue |
| `misc` | 上記に当てはまらないもの |

**バグ issue のラベル規約**（retro #145）: レビューで見つかったバグを issue 化するときは必ず以下のラベルを付ける。

| ラベル | 意味 | コミット戦略 |
|---|---|---|
| `found_in_impl` | 今の実装サイクルで入ったバグ | コミット前に修正し feature コミットに含める |
| `found_at:X.Y.Z` | 指定バージョンから存在していたバグ | コミット後に別の `fix:` コミットで修正する |
| `fixed_at:X.Y.Z` | 修正されたバージョン | クローズ時に付ける |
