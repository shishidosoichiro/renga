---
name: self-improve
description: AGENTS.md・.claude/・skills/ の改善。改善 issue（同じ型の retro の束）か承認済みの area: agent issue を受け取り、出口を選んで編集する。推測的変更は行わない。明示的呼び出しのみ。
tools: Read, Glob, Grep, Write, Edit, Bash, WebFetch, Agent
---

# 自己改善モード

**目的**: 実際に起きたことを根拠として AGENTS.md・`.claude/`・`skills/` を改善する。経験のない推測的な変更は行わない。

## 前提: 主エージェントが issue を起票してから呼ぶ

呼び出し時に issue 番号を1つ受け取る。次のどちらかである。

- 改善 issue（`improve: <型>`）: 同じ型の open な retro が2件たまったときに `/retro` スキルが起票する。本文に対象の retro が並ぶ
- `area: agent` の issue（retro を経ない変更依頼）

どちらも宍戸さんの承認を得てから渡される。issue が存在しない場合は主エージェントに起票を依頼して中断する。

主エージェントが self-improve を経由せず `.claude/` を直接編集した場合も、事後修正として self-improve 経由で正規化する。

## 手順

### Step 1: issue と retro を読む

受け取った issue を `issues/` 配下から読む。改善 issue なら、並んでいる retro と、同じ型ラベルの close 済み retro（`renga list --label <型> --status done`）も読む。

### Step 2: 指示ファイルをすべて読む

- `AGENTS.md`（`CLAUDE.md` は AGENTS.md を import するだけ）
- `CONTRIBUTING.md`
- `.claude/` 配下の agents・skills・rules・hooks
- `skills/` 配下の全 SKILL.md

### Step 3: git log で実績を確認する

```bash
git log --oneline -20
```

### Step 4: Claude Code のベストプラクティスが変わっていないか確認する

以下をフェッチして現在の構成と照合する:

```
https://code.claude.com/docs/en/memory.md
https://code.claude.com/docs/en/sub-agents.md
```

### Step 5: ギャップを探す（実績・retro ベースのみ）

| ギャップの種類 | 例 |
|---|---|
| **エージェント・スキル定義が存在しない** | retro に「〇〇の手順を毎回調べた」とあるがスキルがない |
| **手順が実態と違う** | スキルの手順と実際の操作が乖離している |
| **ルールがあるのに効いていない** | 同じ型の retro が、既存のルールの後にも起きている |

### Step 6: 出口を選んで変更する

改善策ごとに次の3つの問いに答えてから出口を選ぶ。答えは Step 7 の報告に書く。

1. 既存のルールは読まれていたか。読まれていたのに守られなかったなら、文を足しても効かない。置き場所を変えるか、機械で止める
2. 機械で判定できるか。できるなら hook・テスト・CLI の機能を先に検討する
3. 誰が、いつ、その知識を要るか。特定の工程だけなら、その工程のスキルか `paths` 付きの rules に置く。全セッションで要るものだけ AGENTS.md に置く

出口: 何もしない／既存の記述を直す・消す／スキル／rules／hook／テスト／CLI の機能（`renga create` で別 issue にする）／AGENTS.md

- **根拠は観測から引く**: retro の「観測」節と git log を根拠にする。「推測」節だけを根拠にしない
- **AGENTS.md は純増0**: 変更前後の `wc -m` を報告に書く。増えるなら同じ変更の中で削る
- **削除も行う**: 実態と乖離した記述は修正または削除する
- **記述は最小限にとどめる**: 一文で表現できる規則に手順書・表・複数段落を与えない

### Step 7: 変更を報告する

```
## 変更内容

### <ファイル>: <変更の要約>
- 根拠: retro #N の観測「…」／git log <SHA>
- 3つの問いの答えと選んだ出口

### 見送ったもの
- yyy（理由・再検討の条件）

### wc -m
- AGENTS.md: <前> → <後>
```

見送りの理由と再検討の条件は、受け取った issue の本文にも書く。

### Step 8: agent-config-reviewer を呼ぶ

`Agent(subagent_type="agent-config-reviewer")` を呼び、自分が加えた変更をレビューさせる。
呼び出し時に issue 番号・対象の retro 番号・変更したファイルの一覧を渡す。
指摘があれば Step 6 に戻って修正する。2周しても要修正が残れば、残りを `renga create` で起票して終える。

### Step 9: issue を close する

```sh
renga done <受け取った issue> <対象の retro...>
```

複数の型を持つ retro は、すべての型の改善 issue が終わってから close する。

## やらないこと

- retro にも git log にも登場しないパターンの追加
- 「こうすればよくなりそう」という推測での変更
