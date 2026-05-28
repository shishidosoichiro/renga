---
status: done
priority: medium
area: misc
labels: []
---

# ツール名の変更を検討する — fbim は発音しにくく略語として弱い

## 問題

`fbim`（File-Based Issue Management の略）はマーケティング上の問題がある。

- **発音できない**（「えふびーあいえむ」？「ふびむ」？）
- **略語は検索に弱い**
- **意味が直感的でない**（知らないと何もわからない）

bat・delta・dust・just・zoxide など有名な CLI ツールはいずれも発音でき、口頭で伝えられる。crates.io publish 前の今が名前を変える最後のチャンス。

## crates.io 調査済み（取得済み）

jot / tick / koto / fude / leaf / twig / peg / tack / pin / dot / quill / tally / notch / filer / fusen / fuda / kaki / kanri / kiroku / fumi / tobira

## 空き候補

| 名前 | 意味・備考 |
|---|---|
| `iss` | issue の略。発音しにくい |
| `issu` | issue をそのまま短縮 |
| `fil` | file の短縮。フランス語で「糸・流れ」 |
| `techo` | 手帳。日々の記録のイメージ。世界でも通じやすい |
| `kadai` | 課題。issue そのもの |
| `anken` | 案件。仕事の issue にぴったり |
| `hikae` | 控え。控えを取る・記録する |
| `satsu` | 冊。冊子・ノートのイメージ |
| `tsumu` | 積む。タスクが積み上がるイメージ |
| `renga` | 煉瓦/連歌。積み上げてものを作るイメージ |

## 決定

**`renga`** に決定。

理由：
- 発音できる（「レンガ」）
- ポジティブなイメージ — 煉瓦を積み上げて何かを作る・構築する
- done になった issue も消えるのではなく、プロジェクトの成果として積み上がっていくという設計思想と一致
- 「連歌」（複数人で紡いでいく詩）としての意味も、複数のエージェント・人間が協力して issue を管理する使い方と重なる
- crates.io で空き確認済み

## 変更した場合の影響範囲

- `Cargo.toml` の `name`・バイナリ名
- crates.io のクレート名
- `README.md` / `README.ja.md` 全体
- `spec.md` / `spec.ja.md`
- `skills/fbim/` ディレクトリ名・SKILL.md 内のコマンド例
- `install.sh`
- GitHub リポジトリ名（`shishidosoichiro/fbim`）
