@CONTRIBUTING.md

# Renga — 実装ガイド

## 判断方針

- **ベストプラクティスを先に調べる**: 実装・設計の判断を述べる前に、WebSearch / WebFetch でベストプラクティスを調べる。知っているつもりで進めない。自分の推論だけに頼らず、実践の中で検証された方法（RFC・仕様書・設計パターン・公式ドキュメント等）を参照する。
- **容赦なく指摘・提案・批判する**: 宍戸さんの選択・意見に迎合しない。問題があれば全コンテキストで指摘する。
- **根本原因を特定する**: エラーや問題を回避するのではなく、根本原因を特定して解決する。`--no-verify`・`#[allow(...)]`・コンパイルエラーを黙らせる回避策は使わない。
- **ミスや改善を指摘されたら即座に `/retro` を実行する**: 宍戸さんにミスや改善を指摘されたとき、または同じ種類のミスがセッション内で2回以上起きたとき → その場で `/retro` スキルの手順（retro issue 起票 → `self-improve` 起動）を実行する。
- **不要な抽象化・過剰な一般化をしない**: タスクに必要な最小限の実装にとどめる。

## エージェント活用方針

| コンテキスト | トリガー | 指示ファイル |
|---|---|---|
| コード品質・仕様・ドキュメントのレビュー | `Agent(subagent_type="review")` | `.claude/agents/review.md` |
| 自己改善 | `Agent(subagent_type="self-improve")` ※事前に retro issue を起票する | `.claude/agents/self-improve.md` |
| OSS ポジショニング・ローンチ計画 | `Agent(subagent_type="marketing-strategist")` | `.claude/agents/marketing-strategist.md` |
| OSS 公開用ドキュメント執筆・改善 | `Agent(subagent_type="docs-writer")` | `.claude/agents/docs-writer.md` |
| OSS ローンチ実行（投稿文・公開順序） | `Agent(subagent_type="launch-orchestrator")` | `.claude/agents/launch-orchestrator.md` |

**`.claude/` の変更は self-improve 経由のみ**: `CLAUDE.md`・`.claude/` を変更する場合は `/retro` スキルの手順に従う（retro issue 起票 → `self-improve` 起動）。「局所的な変更だから直接やる」という判断は行わない。

**Plan モードをいつ使うか**:
- 3ファイル以上を変更するタスク
- breaking change を含むタスク
- 設計判断が複数ある場合（例: API の設計、ディレクトリ構造の変更）

File-Based Issue Management。詳細仕様は `spec.ja.md` を参照。
`skills/` は Claude Code スキルの配布用ディレクトリ（ユーザーが `~/.claude/skills/` にシンボリックリンクして使う）。インストール方法・コマンド一覧は `README.md` の "Claude Code skill" セクションを参照。

## 実装フロー・コード規約（詳細は自動読み込み）

- コミット手順・レビュー指摘の分類・feat / fix / fixup の判断・ドキュメント更新ルールは `/commit` スキル（`.claude/skills/commit/SKILL.md`）に従う
- Rust コード規約（エラーハンドリング・doc コメント・テスト方針）は `src/`・`tests/` の編集時に自動読み込みされる（`.claude/rules/rust-code.md`）
- リリース手順は `/release` スキルと CONTRIBUTING.md の Releasing 節に従う

## 後方互換

- issue ファイルの 4 桁・5 桁のゼロ埋め ID（`NNNN-*.md`、`NNNNN-*.md`）は読み込めるが、新規作成はゼロ埋めなしの整数（`1`、`42` など）

## Issue 管理

このリポジトリ自体の issue は Renga で管理する（自己ホスト）。

```sh
renga create "タイトル" --area <area>
renga list
renga done <N>...
```

`/renga` スキルでも同じ操作ができる。

area テーブル・バグラベル規約・retro issue 起票ルールの詳細は、issue ファイルを開いたときに自動的に読み込まれる（`.claude/rules/issue-management.md`）。
