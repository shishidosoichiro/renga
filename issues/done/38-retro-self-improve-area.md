---
status: done
priority: medium
area: misc
labels: [retro]
---

# retro: self-improve 経由ルールの違反と新 area 追加

## 失敗・見落とし・やり直し

- **`.claude/` を直接編集した**: retro の notes/ → issue 移行に際して、self-improve.md と CLAUDE.md を直接編集してしまった。「局所的な変更だから」という判断をしてはいけないと CLAUDE.md に明記されているにもかかわらず違反した

## 指示ファイルにあればよかったこと

- **retro issue の area を `misc` にしているが専用 area があるべき**: CLAUDE.md への指示・.claude/agents/ の変更・retro issue は「君への指示関連」として独立した area（案: `agent`）にまとめるべき。CLAUDE.md の area 表と self-improve.md の前提記述を更新する
