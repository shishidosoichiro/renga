---
description: retro issue の起票と self-improve エージェントの起動。宍戸さんにミスや改善を指摘されたとき、同種のミスがセッション内で2回以上起きたとき、CLAUDE.md・.claude/ を変更したいときに必ず使う。
---

# /retro skill

セッション振り返り（retro issue）を起票し、self-improve エージェントで指示ファイルを改善する定型手順。

## Steps

1. retro issue を起票する:
   ```sh
   renga create "retro: <内容>" --area agent --label retro
   ```
2. 起票した issue ファイル（`issues/open/<N>-retro-*.md`）に振り返りを記載する:
   ```markdown
   # セッション振り返り YYYY-MM-DD

   ## うまくいったこと
   ## 失敗・見落とし・やり直し
   ## 指示ファイルにあればよかったこと
   ## その他気づき
   ```
3. `Agent(subagent_type="self-improve")` を issue 番号を渡して起動する

## Rules

- `CLAUDE.md`・`.claude/` の変更は必ずこの手順を経由する（self-improve だけが直接編集できる）。「局所的な変更だから直接やる」という判断は行わない
- 指摘に応じた修正だけして self-improve を省略してはならない
