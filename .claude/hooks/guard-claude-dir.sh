#!/bin/bash
# PreToolUse hook (matcher: Edit|Write)
# このリポジトリの .claude/ 配下・AGENTS.md・CLAUDE.md の変更を self-improve エージェント以外に許可しない（retro #226）。
# 正規ルート: area: agent の issue（改善 issue を含む）を起票 → self-improve エージェントが編集する（#261）。
# 指示は AGENTS.md に一本化し、CLAUDE.md は AGENTS.md を import するだけのファイルにした（#282）。
# 注意: ガード対象はプロジェクトの .claude/・AGENTS.md・CLAUDE.md のみ。パス中の ".claude/" だけでマッチさせると
# ~/.claude/（グローバル領域。memory 等）まで誤ブロックする（実運用で発生した誤検知の修正）。
set -euo pipefail

input=$(cat)
file_path=$(printf '%s\n' "$input" | jq -r '.tool_input.file_path // empty')
agent_type=$(printf '%s\n' "$input" | jq -r '.agent_type // empty')

[ -z "$file_path" ] && exit 0

# プロジェクトルートを特定する。CLAUDE_PROJECT_DIR が無ければ stdin JSON の cwd にフォールバック
project_dir="${CLAUDE_PROJECT_DIR:-}"
if [ -z "$project_dir" ]; then
  project_dir=$(printf '%s\n' "$input" | jq -r '.cwd // empty')
fi
[ -z "$project_dir" ] && exit 0
project_dir="${project_dir%/}"

# 相対パスはプロジェクトルート基準に正規化する
case "$file_path" in
  /*) : ;;
  *) file_path="$project_dir/$file_path" ;;
esac

case "$file_path" in
  "$project_dir/.claude/"*|"$project_dir/AGENTS.md"|"$project_dir/CLAUDE.md")
    if [ "$agent_type" = "self-improve" ]; then
      exit 0
    fi
    jq -n '{
      hookSpecificOutput: {
        hookEventName: "PreToolUse",
        permissionDecision: "deny",
        permissionDecisionReason: ".claude/・AGENTS.md・CLAUDE.md の変更は area: agent の issue を起票し、宍戸さんの承認後に self-improve エージェントに渡して行う。直接編集は禁止。"
      }
    }'
    exit 0
    ;;
esac

exit 0
