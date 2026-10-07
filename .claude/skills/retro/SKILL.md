---
description: retro issue を記録する。宍戸さんにミスや改善を指摘されたとき、同種のミスがセッション内で2回以上起きたときに必ず使う。記録だけで終える。改善は同じ型の retro が2件たまったときに別の工程で回す。
---

# /retro skill

起きたことを retro issue に記録する。改善はしない（#261）。

## Steps

1. 起票する。型ラベルは下の表から1つ以上選ぶ
   ```sh
   renga create "retro: <起きたこと>" --area agent --label retro --label <型>
   ```
2. 本文を3節で書く。対策は書かない
   ```markdown
   ## 観測
   ## そのとき何が見えていたか
   ## 推測
   ```
   - 観測: 起きたことと、誰が何を指摘したか。事実だけ
   - そのとき何が見えていたか: 判断の時点で読んでいた指示・資料と、読んでいなかったもの
   - 推測: 原因の仮説
3. 型ごとに open な retro を数える
   ```sh
   renga list --label retro --json | jq -r '.[].labels[]' | grep -vx retro | sort | uniq -c
   ```
4. 2件以上の型があれば、その型の改善 issue（`renga list --label improve --json` で探す）が無いときは起票し、あるときは本文に retro 番号を追記する
   ```sh
   renga create "improve: <型>" --area agent --priority high --label improve --label <型>
   ```
   本文に対象の retro 番号を並べ、宍戸さんに報告する。いまの作業は止めない。self-improve に渡すのは宍戸さんが承認したとき（`.claude/agents/self-improve.md`）

## 型ラベル

| ラベル | 意味 |
|---|---|
| `skipped-step` | 決まっている工程（レビュー・Plan・self-improve 経由など）を飛ばした、または順序を誤った |
| `missed-sync` | 1か所を変えたとき、連動する箇所（completion・skills・AGENTS.md・手順書・同種の過去コミット）を確かめなかった |
| `wrong-commit` | コミットの型・粒度・件名を誤った |
| `acted-on-question` | 相談や可否の確認に、確認を取らず実行で返した |
| `unread-source` | 判断の前に一次資料（spec・タグ・履歴・既存 issue）を読まなかった |
| `unverified-premise` | 自分または宍戸さんの前提を確かめずに結論に使った |

どれにも当てはまらなければ新しい型を付けてよい。表への追記は次の改善工程で行う。

## Rules

- 振り返りでない変更提案には `retro` ラベルを付けない。`area: agent` の issue として起票し、宍戸さんの承認を得てから self-improve に渡す
