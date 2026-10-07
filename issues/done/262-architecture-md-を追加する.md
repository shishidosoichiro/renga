---
type: Issue
schema_version: 1
status: done
priority: medium
area: docs
labels: []
---

# ARCHITECTURE.md を追加する

## 目的

ARCHITECTURE.md を足し、コードを触る人とエージェントが最初に読む地図にする。

## 根拠

- 原典は matklad の記事 https://matklad.github.io/2021/02/06/ARCHITECTURE.md.html
  - 推奨: 概要・コードマップ・横断的関心事の3節にする。不変条件（多くは「何かが無いこと」）と境界を明示する。名前で挙げてリンクはしない。頻繁に変わらないことだけを書く
  - 手本: rust-analyzer の architecture.md。モジュールごとに `Architecture Invariant:` を書いている
- Renga の src は約4.7k行で、原典が想定する規模（10k〜200k行）より小さい
  - それでも、エージェントは `find_issue` / `find_active_issue` / `find_editable_issue` の使い分けで迷ってきた（#198・#213）
  - 地図があれば、この種の迷いは減ると見込む

## 決めたこと

- **節**: Overview / Code map / Cross-cutting concerns
  - 不変条件と境界は Code map の中に書く
  - 横断的関心事は、Testability・Error handling・Backward compatibility・Configuration の4つ
- **Testability に書く3点**
  - 何を実物で動かすか
  - コードをどう分けるか
  - 起こしにくい状態をどう作るか
- **英語のみで書く**。CONTRIBUTING.md と同じく、コントリビュータ向けの文書のため
- **CLI の振る舞いは spec.md が正**。ARCHITECTURE.md にはリンクだけ置き、写さない
- **見出しに `{#id}` 記法を使わない**。GitHub ではこの記法が原文のまま表示されるため
- **見直しのルール**: 頻繁に変わらないことだけを書く。そのうえで、書いた内容と矛盾する変更をしたら同じコミットで直す
  - 原典は「コードとは同期させず、年に数回見直す」としている
  - それでも矛盾したら直すのは、古くなった不変条件が、何も書かないより有害なため
- **書く手順**: 目次（見出し・問い・冒頭文）で合意してから本文を書く。書き上げたら、書き手にしか通じない語句を洗い出す
- **導線**: CLAUDE.md・AGENTS.md・`/commit` スキルからの導線は `.claude/` 系の変更になるので、#261 の self-improve で一緒に扱う

再検討の条件: 書いたまま更新されず、コードと食い違ったまま見つかったら、置き場か粒度を見直す。

