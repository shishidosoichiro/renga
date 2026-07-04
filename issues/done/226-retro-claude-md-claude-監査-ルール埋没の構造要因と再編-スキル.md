---
schema_version: 1
status: done
priority: medium
area: agent
labels: [retro]
---

# retro: CLAUDE.md・.claude/ 監査 — ルール埋没の構造要因と再編（スキル化・rules 化・hooks 導入）

# セッション振り返り 2026-07-04

## うまくいったこと

- Fable 5 による CLAUDE.md・`.claude/` の全量監査を実施。context-mode で生データ（約190KB: 全 agents/rules/skills/settings + 公式ドキュメント5本）をサンドボックス側に留め、会話コンテキストには要点のみ取り込む調査手順が機能した
- 公式ドキュメントと過去 retro 実績の突き合わせにより、改善を「根拠付き」で特定できた

## 失敗・見落とし・やり直し

- **本セッション**: 「3ファイル以上の変更は Plan モード」ルール（CLAUDE.md 明記）を見落とし、宍戸さんの指摘で Plan モードに入った。ルール不発動の4件目の実例
- **過去 retro からの集約**（いずれも CLAUDE.md に明記済みのルールが不発動だった事例）:
  1. 「ミスを指摘されたら self-improve 起動」の不発動（宍戸さんに明示されるまで気づかず）
  2. コミット前レビュー・カバレッジ確認の省略
  3. `.claude/` 直接編集（「局所的な変更だから」という判断の禁止ルール違反）

## 指示ファイルにあればよかったこと

- 指示の「追加」ではなく「構造の変更」が必要という結論（下記分析参照）。ルールを増やすほど埋没が悪化する

## 分析（将来の改善エージェント向け記録）

### 症状の共通原因

- (a) コミット時にしか使わない手続き知識（実装フロー 約70行）が常時ロードされ、他ルールの salience を下げている。常時ロード量: CLAUDE.md 141行(557語) + @CONTRIBUTING.md 143行(841語) ≒ 2千トークン弱
- (b) 規律の強制がすべて散文で、遵守率がモデル性能に依存する（Sonnet で崩れる）
- 出典: Anthropic 公式ベストプラクティス（claude-code-best-practices）—「ルールがあるのに従わない場合、ファイルが長すぎてルールが埋もれているサイン」

### 三重重複とドリフト実績

- fixup・リリース手順が CLAUDE.md（fixup 言及11回）・CONTRIBUTING.md・`.claude/skills/release/SKILL.md` の3箇所に重複
- ドリフト実績あり: コミット 55a2349 で release スキルの stale な手動 cargo publish 手順を除去

### 技術的事実（hooks 設計の根拠。公式 hooks リファレンスで確認済み）

- PreToolUse hook の stdin JSON には `agent_type` フィールドが含まれ、サブエージェント種別を判定できる（Hook B で正規ルートのみ許可できる根拠）
- PreToolUse の `additionalContext` が読まれるのは「次のモデルリクエスト時」。`git commit` を許可しつつ注入しても読まれるのはコミット実行後 → 「非ブロック注入」は実効性がない（Hook C1 を弱いと判断した根拠）
- permissions の deny ルール（Bash パターン）はフラグ順序が変わると素通りする（`git commit -m "x" --no-verify` は `Bash(git commit --no-verify*)` にマッチしない）。hook はコマンド文字列全体を正規表現で見るので順序非依存（Hook A を permission でなく hook にした根拠）

## 承認済み改善プラン（宍戸さん承認 2026-07-04）

### Phase 1: 構造再編

1. `.claude/skills/commit/SKILL.md` 新設: CLAUDE.md の「実装フロー」「コミット粒度の規律」「feat/fix/fixup の使い分け」「リリース前 fixup フロー」「ドキュメント更新ルール表」を移設し、コミット時に呼ぶ手順書として再構成。description はコミット・レビュー分類・fixup 判断で自動トリガーする書き方にする
2. `.claude/rules/rust-code.md` 新設（frontmatter `paths: ["src/**/*.rs", "tests/**/*.rs"]`）: 「エラーハンドリング」「ドキュメント」「テスト方針」を移設
3. `.claude/skills/retro/SKILL.md` 新設: retro issue 起票 → self-improve 起動の定型手順（`.claude/rules/issue-management.md` のフォーマット参照）
4. CLAUDE.md スリム化: 残すのは判断方針・エージェント活用表・Plan モード基準・後方互換・issue 管理・各スキル/rules への1行ポインタ。目標 ~60行（現141行）
5. CONTRIBUTING.md は変更なし（人間コントリビューター向けの正）

### Phase 2: hooks 決定論化（A・B のみ採用）

- `.claude/settings.json` 新設（チェックイン）: PreToolUse 2本を登録（matcher `Bash` → block-no-verify.sh、matcher `Edit|Write` → guard-claude-dir.sh）
- **Hook A** `.claude/hooks/block-no-verify.sh`: stdin JSON の `tool_input.command` を読み、`git commit` 呼び出し内の `--no-verify` / `-n` を検出したら exit 2 + stderr で理由を返す。マッチは `git commit` の内側に限定し、パイプ先の別コマンド（`grep -n` 等）には反応させない
- **Hook B** `.claude/hooks/guard-claude-dir.sh`: `tool_input.file_path` が `.claude/**` または `CLAUDE.md` の場合、`agent_type == "self-improve"` なら許可（exit 0）、それ以外は JSON で `permissionDecision: "deny"` + 理由「retro issue 起票 → self-improve 経由で行う」を返す。リポジトリルートの `skills/`（配布物）は対象外
- 両スクリプトは `chmod +x`。hooks は Claude のツール呼び出しにのみ作用し、人間の git 操作には影響しない。hooks の有効化はセッション開始時ロードのため次セッションから

### 見送り判断と再検討条件（将来のエージェントは再提案の前にここを確認すること）

- **Hook C1**（git add/status 時の非ブロック注入）: 複合コマンド（`git add -A && git commit`）で素通りする + 注入が繰り返されるコスト。C2 に対する明確な優位なし → 不採用
- **Hook C2**（コミット前ワンタイムゲート: 初回 git commit を deny してチェックリスト提示、マーカーファイルで再試行を許可）: 採用保留。**再検討条件: Phase 1 の /commit スキル運用後もレビュー・カバレッジの素通りが retro に再発した場合に導入**
- **エージェントへの model 固定**（review に fable/opus 等）: 宍戸さん判断で inherit 維持（2026-07-04）
- **Rust 編集ごとの fmt/clippy 自動実行 hook**: retro に fmt/clippy 起因の失敗の記録なし（「1変更=1根拠」原則により見送り）

## 実装後の修正記録（2026-07-04）

Hook B（`guard-claude-dir.sh`）の初版は `*/.claude/*` というパターンでパス中のどこにある `.claude/` にもマッチしていたため、リポジトリ外のユーザーグローバル領域まで誤ってブロックした。実測: 主エージェントのセッションで `~/.claude/projects/<project>/memory/*.md` への Write が Hook B の deny で拒否された（settings.json が同一セッション内で即時有効化されることもこのとき確認）。原因は、ガード対象の意図（このリポジトリの `.claude/` 配下と `CLAUDE.md` のみ）に対してマッチ範囲が広すぎたこと。修正として、hook プロセスに渡る `CLAUDE_PROJECT_DIR` 環境変数（未設定時は stdin JSON の `cwd` にフォールバック）を基準に、`"$CLAUDE_PROJECT_DIR/.claude/"*` と `"$CLAUDE_PROJECT_DIR/CLAUDE.md"` への完全前方一致のみを deny 対象とした。回帰テスト（`~/.claude/projects/**`・`~/.claude/plans/**` → 許可、リポジトリの `.claude/` → 引き続き deny、cwd フォールバック動作、別プロジェクトの `CLAUDE.md` → 許可）を含む全 24 ケースの PASS を確認済み。

Hook A（`block-no-verify.sh`）の初版にも誤検知があった。実測: `git commit -m "$(cat <<'EOF' … EOF)"` 形式のコミットで、heredoc 本文中の説明文字列「git commit --no-verify / -n」に反応してブロックされた（実際のコマンドに `--no-verify` フラグはなし）。原因は、引用符除去が単純なクォート対の sed 置換のみで、コマンド置換 + heredoc のネストを処理できず、さらにセグメント分割（`|;&`・改行）が引用符除去より先だったため heredoc 本文の各行が独立セグメントとして検査されていたこと。修正として検査順序を再構成した: (1) heredoc 本体（`<<MARKER` 〜 行頭 `MARKER`。`<<-`・引用付きマーカー対応）を行単位のステートマシンで除去 → (2) シングル/ダブルクォート内文字列を除去 → (3) セグメント分割して既存の正規表現を適用。実フラグは常に非引用領域にあるため検出漏れは生じない。回帰テスト（実測失敗ケースの heredoc 内説明文字列 → 許可、メッセージ内説明文字列 → 許可、実フラグ + heredoc → ブロック、heredoc なし `-n` 実フラグ → ブロック）4 ケースを追加し、既存 13 ケース含む Hook A 全 17 ケース、Hook B 全 11 ケース、計 28 ケースの PASS を確認済み。

