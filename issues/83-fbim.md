---
status: open
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

## 変更した場合の影響範囲

- `Cargo.toml` の `name`・バイナリ名
- crates.io のクレート名
- `README.md` / `README.ja.md` 全体
- `spec.md` / `spec.ja.md`
- `skills/fbim/` ディレクトリ名・SKILL.md 内のコマンド例
- `install.sh`
- GitHub リポジトリ名（`shishidosoichiro/fbim`）
