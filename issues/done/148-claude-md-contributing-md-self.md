---
schema_version: 1
status: done
priority: medium
area: agent
labels: [retro]
---

# CLAUDE.md と CONTRIBUTING.md の構造を整理して self-improve の発動率を改善する

## うまくいったこと

特になし（このセッションは問題対処が中心）

## 失敗・見落とし・やり直し

「completion に --add-label を追加するのを忘れているよ」と宍戸さんにミスを指摘された際、CLAUDE.md に「ミスを指摘されたら self-improve を呼ぶ」と明記されているにもかかわらず発動しなかった。宍戸さんに「移らないんだね。指示が効いていない？」と明示的に言われるまで気づかなかった。

**なぜ発動しなかったか：** CLAUDE.md の構造的な問題により重要なルールが埋もれていた。
- `## 判断方針`（最も根本的な行動原則）が末尾（94行目）にある
- self-improve トリガーが `## エージェント活用方針` のエージェント表の中に埋まっており、bold も IMPORTANT もない
- 全体的に重要度の差が視覚的に伝わらない

## 指示ファイルにあればよかったこと

**CLAUDE.md の構造改善：**
1. `## 判断方針` を冒頭（`## エージェント活用方針` より前）に移動
2. self-improve トリガーを `## 判断方針` に統合し bold で強調
3. `## エージェント活用方針` はエージェント表と `.claude/` 変更ルールのみに絞る
4. 重要ルールを bold で視覚的に目立たせる（控えめに）

**CONTRIBUTING.md の構造改善：**
`@CONTRIBUTING.md` で CLAUDE.md にインポートされているため、CONTRIBUTING.md 側の構造も整理する。フラット構造・bold 強調・番号付きリストに沿って改善する。

## その他気づき

**調査済みベストプラクティス（参考 URL）：**
- https://howborisusesclaudecode.com/ — Boris Cherny（Claude Code 作者）の実例
- https://code.claude.com/docs/en/best-practices — Anthropic 公式
- https://www.datacamp.com/tutorial/writing-the-best-claude-md — DataCamp ガイド
- https://www.builder.io/blog/claude-md-guide — Builder.io ガイド
- https://ranthebuilder.cloud/blog/claude-code-best-practices-lessons-from-real-projects/ — 実プロジェクトからの知見

Boris Cherny のスタイル（実例）：

```markdown
# Development Workflow
**Always use `bun`, not `npm`.**

# 1. Make changes
# 2. Typecheck (fast)
bun run typecheck

# 3. Run tests
bun run test -- -t "test name"        # Single suite
bun run test:file -- "glob"           # Specific files
```

- h1 見出しのみのフラット構造（h2/h3 を使わない）
- 必須ルールは `**bold**` で行頭に置く
- ワークフローは番号付きリスト
- コマンドはコードブロック＋インラインコメント

Anthropic 公式の原則：
- 重要ルールには `IMPORTANT` や `YOU MUST`（控えめに）
- CLAUDE.md は advisory。毎回確実に動かすべきルールは hooks へ
- 200行以下。「この行を削除すると Claude は誤るか？」No なら削除

