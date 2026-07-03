@CONTRIBUTING.md

# Renga — 実装ガイド

## 判断方針

- **ベストプラクティスを先に調べる**: 実装・設計の判断を述べる前に、WebSearch / WebFetch でベストプラクティスを調べる。知っているつもりで進めない。自分の推論だけに頼らず、実践の中で検証された方法（RFC・仕様書・設計パターン・公式ドキュメント等）を参照する。
- **容赦なく指摘・提案・批判する**: 宍戸さんの選択・意見に迎合しない。問題があれば全コンテキストで指摘する。
- **根本原因を特定する**: エラーや問題を回避するのではなく、根本原因を特定して解決する。`--no-verify`・`#[allow(...)]`・コンパイルエラーを黙らせる回避策は使わない。
- **ミスや改善を指摘されたら即座に self-improve を起動する**: 宍戸さんにミスや改善を指摘されたとき、または同じ種類のミスがセッション内で2回以上起きたとき → **その場で retro issue を起票してから `self-improve` エージェントを呼ぶ**。指摘に応じた修正だけして self-improve を省略してはならない。
- **不要な抽象化・過剰な一般化をしない**: タスクに必要な最小限の実装にとどめる。

## エージェント活用方針

| コンテキスト | トリガー | 指示ファイル |
|---|---|---|
| コード品質・仕様・ドキュメントのレビュー | `Agent(subagent_type="review")` | `.claude/agents/review.md` |
| 自己改善 | `Agent(subagent_type="self-improve")` ※事前に retro issue を起票する | `.claude/agents/self-improve.md` |
| OSS ポジショニング・ローンチ計画 | `Agent(subagent_type="marketing-strategist")` | `.claude/agents/marketing-strategist.md` |
| OSS 公開用ドキュメント執筆・改善 | `Agent(subagent_type="docs-writer")` | `.claude/agents/docs-writer.md` |
| OSS ローンチ実行（投稿文・公開順序） | `Agent(subagent_type="launch-orchestrator")` | `.claude/agents/launch-orchestrator.md` |

**`.claude/` の変更は self-improve 経由のみ**: `CLAUDE.md`・`.claude/agents/` を変更する場合は、必ず retro issue（`area: agent, labels: [retro]`）を起票してから `self-improve` エージェントを呼ぶ。「局所的な変更だから直接やる」という判断は行わない。

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
| CLI の動作・引数・出力形式 | `README.md`, `README.ja.md`, `skills/` 配下の該当 `SKILL.md` |
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
   - **前タグ以降に追加した別コミットのコードへの手直し**（今回のコミットとは別の、まだリリースされていない変更を直す場合）→ `git commit --fixup <SHA>` で積み、リリース前に統合する（後述「リリース前 fixup フロー」）。`fix:` にしない
   - **以前のバージョンから存在していたバグ** → コミット後に別の `fix:` コミットで修正する。issue 化する場合は `found_at:X.Y.Z`（バグが混入したバージョン）ラベルを付け、修正時に `fixed_at:X.Y.Z` ラベルを付けてクローズする
5. 今回の実装で入ったバグを修正する
6. `renga done <N>...` で issue を close してから、コード変更・issue close をまとめてコミットする
7. 以前からあったバグがあれば `renga create` で起票してから、別の `fix:` コミットで修正し同時に close する

カバレッジは実装時とレビュー時の両方で確認する。

**コミット前の自問**: 「このコードをスタッフエンジニアがレビューしたら承認するか？」と自問する。feat と fix が混在していないか、テストが不十分でないか、ドキュメントが更新されているかを確認する。

**変更の順序**: 新規追加を先にコミット・確認してから、削除や簡略化を行う（追加 → 確認 → 破壊）。

**コミットメッセージの言語**: `type`・`scope`・`description` はすべて英語で書く（`Use English for the description` — CONTRIBUTING.md 参照）。issue ファイルのタイトル・本文、CLAUDE.md、`.claude/agents/` は日本語でよい。

**コミット粒度の規律**（retro #134, retro #143）:

- `feat:` と `fix:` を同一コミットに混ぜない（ただし「今回の実装で入ったバグ」は feature コミットに含めてよい）
- 「以前のバージョンから存在していたバグ」はコミット後に必ず別の `fix:` コミットにする
- 「ついでに修正」禁止: 本来のタスクと無関係な変更が見つかったら `renga create` で別 issue を起票し、別コミットで対処する
- 複数の breaking change は別々のコミットに分ける

**feat / fix / fixup の使い分け**（retro #211）:

| 状況 | 使う型 |
|---|---|
| 新機能の実装 | `feat:` |
| リリース済み（前タグ以前）コードのバグ修正 | `fix:`（git-cliff で CHANGELOG に掲載される） |
| 前タグ以降に追加した未リリースコードへの手直し | `fixup!`（リリース前に統合する。CHANGELOG には残さない） |

「以前のバージョンから存在していたバグ」と「前タグ以降に追加したコードへの手直し」を取り違えない。前者は `fix:`、後者は `fixup!`。未リリースのコードへの手直しを `fix:` にすると、まだ世に出ていない変更が CHANGELOG に「修正」として載ってしまう。

### リリース前 fixup フロー（retro #211）

前タグ以降に追加したコード（今回のコミットとは別コミット）への手直しは、`fix:` ではなく fixup で積む。

```sh
git commit --fixup <対象コミットSHA>
```

タグを打つ前（CONTRIBUTING.md の Releasing 手順内）に統合する。

```sh
# 前タグがある場合
GIT_SEQUENCE_EDITOR=true git rebase -i --autosquash $(git describe --tags --abbrev=0)
# 前タグがない場合（初回リリース）
GIT_SEQUENCE_EDITOR=true git rebase -i --autosquash $(git rev-list --max-parents=0 HEAD)
```

- `git rebase --autosquash` は **push 前のローカルコミットにのみ適用する**。push 済みのコミットに適用すると force push が必要になり履歴が壊れる。push 後に気づいた手直しは新たな `fixup!` コミットとして積む。
- 前タグより前から存在するバグへの修正は通常の `fix:` のまま残す（fixup にしない）。
- fixup autosquash 以外の対話的 rebase（複数コミットの構造変更）を行う場合は Plan モードに入る。

## Issue 管理

このリポジトリ自体の issue は Renga で管理する（自己ホスト）。

```sh
renga create "タイトル" --area <area>
renga list
renga done <N>...
```

`/renga` スキルでも同じ操作ができる。

area テーブル・バグラベル規約・retro issue 起票ルールの詳細は、issue ファイルを開いたときに自動的に読み込まれる（`.claude/rules/issue-management.md`）。
