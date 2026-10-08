---
type: Issue
schema_version: 1
status: done
priority: medium
area: agent
labels: []
---


# エージェント向けの指示を AGENTS.md に一本化し、CLAUDE.md を AGENTS.md の import だけにする

## 問題

エージェント向けの指示が CLAUDE.md（2015字）と AGENTS.md（6606字）に二重にある。判断方針・retro の手順・ドキュメント更新の規則などが両方に書かれている。

2026-10-07 のセッションでは、両方を直すコミットが2つあった（701c31f が #261・#262、363ac7f が #277）。二重管理の手間と、片方だけ直し忘れる危険が続いている。

## 根拠（公式ドキュメント）

https://code.claude.com/docs/en/memory.md#when-claude-code-reads-agents-md

- Claude Code は、作業ディレクトリとその親に CLAUDE.md が無いとき、AGENTS.md を自動で読む。Renga の親ディレクトリには CLAUDE.md が無い（2026-10-07 に確認）
- ユーザー単位の `~/.claude/CLAUDE.md` と `.claude/rules/` は、AGENTS.md の読み込みを止めない。AGENTS.md と一緒に読まれる
- AGENTS.md の中の `@path`（例: 先頭の `@CONTRIBUTING.md`）は、CLAUDE.md と同じく展開される
- 確認できなかった点: サブエージェントに AGENTS.md が渡るか。ドキュメントに明記が無い

## やること

方針は 2026-10-08 に改めた（宍戸さんに報告済み）。当初は CLAUDE.md を削除する案だったが、次の理由で import の1行を残す。

- AGENTS.md の自動読み込みは Claude Code v2.1.277 以降に限られる。各自が `/config` の Project instructions を変えている場合もある
- `@AGENTS.md` の1行を置けば、どの環境でも確実に読まれる（memory.md「Share one file with other coding tools」、https://www.deployhq.com/blog/ai-coding-config-files-guide 、https://agyn.io/blog/claude-md-agents-md-compatibility ）

1. CLAUDE.md にしかない内容を AGENTS.md に統合し、CLAUDE.md の中身を `@AGENTS.md` の1行にする
2. 本文は役割と使う場面をツールに依存しない言葉で書き、サブエージェントの定義ファイル（`.claude/agents/<name>.md`）を表で示す。ツール固有の記述は末尾の「## Claude Code」「## Codex」節に分ける
   - 「Codex ではサブエージェントが使えない」とは書かない。Codex は 2026年3月にサブエージェントを正式に提供し、custom agent を TOML で定義できる
   - Codex 用の custom agent 定義は範囲外とし、#283 に切り出す
   - self-improve は Claude Code で実行すると明記する。`.claude/` を守る guard hook が Claude Code でしか効かないため
3. CLAUDE.md を名指ししている箇所を、「CLAUDE.md は AGENTS.md を import するだけのファイル」という前提で直す。「CLAUDE.md の純増0」の規則は AGENTS.md を対象にする
4. guard hook は CLAUDE.md の保護を残し、AGENTS.md を保護対象に加える
5. AGENTS.md の文字数（`wc -m`）は統合前の「CLAUDE.md + AGENTS.md」の合計を下回り、行数は200行以内にする

## 確かめること（移行後）

- 新しいセッションで、AGENTS.md の内容と `@CONTRIBUTING.md` が読み込まれていること
- サブエージェント（例: review）に AGENTS.md の内容が渡っていること。渡らない場合は、必要な指示をエージェント定義側で参照させる

## 再検討の条件

Claude Code の読み込み規則が変わったら見直す。サブエージェントに指示が渡らないことが分かった場合も見直す。


## 実施結果（2026-10-08）

- CLAUDE.md にしかなかった内容（サブエージェントの起動方法、`/retro`・`/commit`・`/release` スキル、Plan モード）を AGENTS.md の「## Claude Code」節に移した。CLAUDE.md は `@AGENTS.md` の1行にした
- guard hook の保護対象に AGENTS.md を加えた。CLAUDE.md の保護は残した
- AGENTS.md: 6606字・163行 → 7116字・176行。統合前の合計は 8621字（CLAUDE.md 2015 + AGENTS.md 6606）。CLAUDE.md は 2015字 → 11字
- サブエージェント: sub-agents.md「What loads at startup」によれば、サブエージェントは主セッションと同じ CLAUDE.md の階層を読む（Explore・Plan と `omitClaudeMd` の定義を除く）。CLAUDE.md が AGENTS.md を import するので、AGENTS.md も渡るはず。実機では未確認

## 見送ったもの

- Codex 側の保護: `.codex/config.toml` は `.claude` を read にしているが、AGENTS.md は書き込める。移行前の CLAUDE.md も Codex からは守られていなかったので、今回の保護の引き継ぎには当たらない。再検討の条件: Codex が self-improve を経ずに AGENTS.md を変えた事例が出たとき、または #283 で Codex の custom agent を定義するとき
- 重複の整理: AGENTS.md の実装フロー・コミット規律・ドキュメント更新ルールは `/commit` スキルと、area 表・ラベル規約は `.claude/rules/issue-management.md` と重なる。Claude Code では両方が読まれるようになった。今回は一本化の移行に絞った。#267・#270 では AGENTS.md と `/commit` スキル・rules・review.md の両方を直しており、この重なりでも二重修正は起きている。再検討の条件: 片方だけ直して食い違う事例が出たとき
- guard hook での `CLAUDE.local.md` の保護: CLAUDE.md を import として残したので、AGENTS.md が読まれなくなる心配は無い。事例も無い。再検討の条件: `CLAUDE.local.md` が指示の置き場として使われた事例が出たとき
