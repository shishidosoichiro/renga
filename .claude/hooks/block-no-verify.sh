#!/bin/bash
# PreToolUse hook (matcher: Bash)
# git commit 呼び出し内の --no-verify / -n を検出してブロックする（retro #226）。
# CLAUDE.md 判断方針「--no-verify を使わない」の決定論的な強制。
# permissions の deny ルールはフラグ順序が変わると素通りするため hook で実装している。
#
# 検査手順: 実フラグは引用符・heredoc の外にしか現れないため、
#   (1) heredoc 本体（<<MARKER 〜 行頭 MARKER）を除去し、
#   (2) シングル/ダブルクォート内の文字列（コミットメッセージ等）を除去してから、
#   (3) パイプ・`;`・`&&`・`||`・改行で区切った各セグメントのうち
#       `git commit` 呼び出しを含むセグメントだけを検査する。
# これにより、heredoc やメッセージ文字列の中に説明として書かれた
# 「git commit --no-verify / -n」や、パイプ先の別コマンドの -n（grep -n 等）には反応しない。
set -euo pipefail

command=$(jq -r '.tool_input.command // empty')

# (1) heredoc 本体を除去する（<<MARKER / <<-MARKER / <<'MARKER' / <<"MARKER" に対応）
heredoc_re="<<-?[[:space:]]*['\"]?([A-Za-z_][A-Za-z0-9_]*)"
sanitized=""
in_heredoc=0
marker=""
while IFS= read -r line; do
  if [ "$in_heredoc" = 1 ]; then
    trimmed="${line#"${line%%[![:space:]]*}"}"
    if [ "$trimmed" = "$marker" ]; then
      in_heredoc=0
    fi
    continue
  fi
  if [[ "$line" =~ $heredoc_re ]]; then
    marker="${BASH_REMATCH[1]}"
    in_heredoc=1
  fi
  sanitized+="$line"$'\n'
done <<< "$command"

# (2) 引用符内の文字列を除去する（行単位。実フラグは常に非引用領域にある）
stripped_all=$(printf '%s' "$sanitized" | sed -E "s/'[^']*'//g; s/\"[^\"]*\"//g")

# (3) セグメント分割して検査する
while IFS= read -r segment; do
  # `git -c key=val commit` のようなグローバルオプション（値付き含む）を挟んだ呼び出しも検出する
  if printf '%s\n' "$segment" | grep -qE '(^|[[:space:];&|])git([[:space:]]+-[^[:space:]]*([[:space:]]+[^-[:space:]][^[:space:]]*)?)*[[:space:]]+commit([[:space:]]|$)'; then
    # --no-verify、単独の -n、短縮フラグ結合形（-anm 等）の n を検出する
    if printf '%s\n' "$segment" | grep -qE '(^|[[:space:]])(--no-verify(=[^[:space:]]*)?|-[a-zA-Z]*n[a-zA-Z]*)([[:space:]]|$)'; then
      echo "git commit の --no-verify / -n は禁止されています。pre-commit フックの失敗は回避せず、根本原因を修正してからコミットしてください（CLAUDE.md 判断方針）。" >&2
      exit 2
    fi
  fi
done < <(printf '%s\n' "$stripped_all" | tr '|;&' '\n\n\n')

exit 0
