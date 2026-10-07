@CONTRIBUTING.md

# Renga — 実装ガイド

## 判断方針

- **ベストプラクティスを先に調べる**: 実装・設計の判断を述べる前に、ウェブ検索等でベストプラクティスを調べる。知っているつもりで進めない。RFC・仕様書・設計パターン・公式ドキュメント等を参照する。
- **容赦なく指摘・提案・批判する**: 宍戸さんの選択・意見に迎合しない。問題があれば全コンテキストで指摘する。
- **根本原因を特定する**: エラーや問題を回避するのではなく、根本原因を特定して解決する。`--no-verify`・`#[allow(...)]`・コンパイルエラーを黙らせる回避策は使わない。
- **ミスや改善を指摘されたら即座に retro を記録する**: 宍戸さんにミスや改善を指摘されたとき、または同じ種類のミスがセッション内で 2 回以上起きたとき → その場で retro issue を起票する。

## Ambiguity and constraints

Do not silently reinterpret the user's explicit request into a
nearest-match implementation. If the request is impossible,
contradictory, constrained, or ambiguous enough to change the
result, stop before acting, explain the issue, and ask one
clarifying question or list alternatives. Do not implement an
alternative or retry with a different interpretation without
confirmation.

When the user asks a consultation-style question such as "what should
we do first?", "how should we proceed?", or "what do you recommend?",
treat it as decision support, not implementation permission. Present
the options, tradeoffs, and one recommendation, then stop and ask for
confirmation before editing files, running long verification, committing,
or closing issues. Do not start implementation until the target issue or
change scope, the intended action, and explicit permission to proceed are
all clear.

## エージェント活用方針

Codex でサブエージェントまたは custom agent が利用可能な場合は、以下の用途で活用する。Claude Code 固有の `Agent(subagent_type=...)` 構文は使わず、その環境で利用できる同等の subagent / custom agent / tool を使う。

| コンテキスト | トリガー | 指示 |
|---|---|---|
| コード品質・仕様・ドキュメントのレビュー | コミット前レビュー | review / reviewer 相当のサブエージェントに依頼する |
| 自己改善 | 同じ型の open な retro が 2 件たまったとき | 改善 issue を起票し、宍戸さんの承認後に self-improve / worker 相当に依頼する |
| OSS ポジショニング・ローンチ計画 | マーケティング・公開戦略の相談 | marketing-strategist 相当があれば使う |
| OSS 公開用ドキュメント執筆・改善 | README・公開文書の大きな改善 | docs-writer 相当があれば使う |
| OSS ローンチ実行 | 投稿文・公開順序・告知導線の作成 | launch-orchestrator 相当があれば使う |

**エージェント設定ファイルの変更は self-improve 経由**: `AGENTS.md`、`CLAUDE.md`、`.claude/`、`.codex/agents/`、`.agents/` を変更する場合は、必ず `area: agent` の issue を起票し、宍戸さんの承認後に self-improve / worker 相当のサブエージェントを呼ぶ。「局所的な変更だから直接やる」という判断は行わない。CLI 仕様変更に伴う `skills/` 配下のドキュメント同期は通常のドキュメント更新として扱い、この規則の対象にしない。

**明示的な計画を出す条件**:
- 3 ファイル以上を変更するタスク
- breaking change を含むタスク
- 設計判断が複数ある場合（例: API の設計、ディレクトリ構造の変更）

File-Based Issue Management。詳細仕様は `spec.ja.md`、コードの地図と不変条件は `ARCHITECTURE.md` を参照。
`skills/` はエージェントスキルの配布用ディレクトリ（`~/.claude/skills/` または `~/.agents/skills/` にシンボリックリンクして使う）。インストール方法・コマンド一覧は `README.md` の "Claude Code skill" セクションを参照。

## コード規約

コード品質の指針と Renga 固有のコード規約は CONTRIBUTING.md の Code quality 節に従う。

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
| モジュール構成・不変条件・境界 | `ARCHITECTURE.md`（記述と矛盾する変更をしたら同じコミットで直す） |

英語版と日本語版（`README.md` / `README.ja.md`、`spec.md` / `spec.ja.md`）は常に同期する。片方だけ更新しない。

## 実装フロー

コードを変更するときは以下の順序で進める。

1. 実装 + テスト追加
2. カバレッジ確認（`cargo llvm-cov --summary-only`）
3. **コミット前に** review / reviewer 相当のサブエージェントでコードレビューを受ける。利用可能なレビュー用サブエージェントがない場合のみ、その理由を明記してセルフレビューする。観点: CONTRIBUTING.md の Code quality 節・生成される `CHANGELOG.md` の読みやすさ。レビュー観点にはカバレッジ確認（`cargo llvm-cov --summary-only`）を含める
4. レビュー指摘を分類する（判定基準: 「このバグは今回変更したコードに由来するか？」）。issue 化するときのラベルは「バグ issue のラベル規約」に従う
   - **今回の実装で入ったバグ**（新規追加ファイルや今回変更した箇所に起因）→ コミット前に修正し feature コミットに含める
   - **以前のバージョンから存在していたバグ** → コミット後に別の `fix:` コミットで修正する
5. 今回の実装で入ったバグを修正する
6. `renga done <N>...` で issue を close してから、コード変更・issue close をまとめてコミットする
7. 以前からあったバグがあれば `renga create` で起票してから、別の `fix:` コミットで修正し同時に close する

カバレッジは実装時とレビュー時の両方で確認する。

**コミット前の自問**: 「このコードをスタッフエンジニアがレビューしたら承認するか？」と自問する。feat と fix が混在していないか、テストが不十分でないか、ドキュメントが更新されているか、`git cliff` で生成される `CHANGELOG.md` に commit subject がそのまま載っても意味が通るかを確認する。

**変更の順序**: 新規追加を先にコミット・確認してから、削除や簡略化を行う（追加 → 確認 → 破壊）。

**コミットメッセージの言語**: `type`・`scope`・`description` はすべて英語で書く（`Use English for the description` — CONTRIBUTING.md 参照）。issue ファイルのタイトル・本文、`AGENTS.md`、`CLAUDE.md`、`.claude/agents/`、`.codex/agents/`、`.agents/` は日本語でよい。

**コミットメッセージの changelog 品質**: `feat:` / `fix:` の subject は git-cliff により `CHANGELOG.md` のリリースノート本文になる前提で書く。subject だけを読んで、ユーザーまたは将来のメンテナが「何が追加・変更・修正されたか」を理解できなければならない。内部作業状態・issue 処理・リリース準備の都合を subject にしない。

- 悪い例: `fix: clear release blockers`
- 良い例: `fix: validate only selected issues`
- 良い例: `fix: operate on misplaced active issues`
- 良い例: `feat: add assignee field to issue front matter`

`feat:` はユーザーが得る能力、`fix:` は修正された具体的な不具合や誤動作を書く。背景・根本原因・移行注意点が subject に収まらない場合は commit body に書く。

**コミット type の選択**: `type` は issue の `label` や `area` ではなく、実際の diff の種類で決める。production code の挙動を変えずテストだけを追加・修正する場合は、issue が `found_at:*` 付きでも `test:` を使う。`fix:` は production code のバグ修正を含む変更に使う。type 誤分類を修正したときは、直近の関連コミットを `git log --oneline --stat` で横断確認し、同じ誤分類が残っていないか確認してから完了報告する。

**コミット粒度の規律**:

- `feat:` と `fix:` を同一コミットに混ぜない（ただし「今回の実装で入ったバグ」は feature コミットに含めてよい）
- 「以前のバージョンから存在していたバグ」はコミット後に必ず別の `fix:` コミットにする
- 「ついでに修正」禁止: 本来のタスクと無関係な変更が見つかったら `renga create` で別 issue を起票し、別コミットで対処する
- 複数の breaking change は別々のコミットに分ける

## Issue 管理

このリポジトリ自体の issue は Renga で管理する（自己ホスト）。

```sh
renga create "タイトル" --area <area>
renga list
renga done <N>...
```

`/renga` スキルでも同じ操作ができる。

## issue の分類ルール

### area の選択

| area | 使うとき |
|---|---|
| `cli` | コマンドライン引数・ヘルプ表示 |
| `core` | issue のパース・ファイル操作・プロジェクトルート探索 |
| `config` | `.renga.yml` の読み込み・設定 |
| `test` | テストの追加・修正 |
| `docs` | ドキュメント・README・CONTRIBUTING |
| `ci` | CI/CD パイプライン |
| `agent` | `AGENTS.md`・`CLAUDE.md`・`.claude/`・`.codex/agents/`・`.agents/` の変更・retro・改善 issue |
| `misc` | 上記に当てはまらないもの |

### バグ issue のラベル規約

レビューで見つかったバグを issue 化するときは必ず以下のラベルを付ける。

| ラベル | 意味 | コミット戦略 |
|---|---|---|
| `found_in_impl` | 今の実装サイクルで入ったバグ | コミット前に修正し feature コミットに含める |
| `found_at:X.Y.Z` | 見つけた版（レビューで見つけたなら最新タグ。`v` は付けない） | リリース済みのコードのバグに付ける。コミット後に別の `fix:` コミットで修正する |
| `since:X.Y.Z` | 作り込んだ版。調べて分かったときだけ付け、推測で埋めない | — |
| `fixed_at:X.Y.Z` | 修正されたバージョン | クローズ時に付ける |

### retro issue の起票ルール

retro は記録だけにする。

```sh
renga create "retro: <内容>" --area agent --label retro --label <型>
```

- `area: agent`、`labels: [retro]` と型ラベルを1つ以上付ける。本文は「観測／そのとき何が見えていたか／推測」の3節。型の語彙は `.claude/skills/retro/SKILL.md`
- 記録だけで終える。同じ型の open な retro が 2 件になったら、改善 issue を起票して宍戸さんに報告する
