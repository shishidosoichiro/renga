---
type: Issue
schema_version: 1
status: open
priority: medium
area: agent
labels: []
---

# コード品質の指針を CONTRIBUTING.md に置き、rust-code.md を固有の決定だけにする

## 問題

コードの書き方の規約が、人間のコントリビュータの見えない場所にある。

- **CONTRIBUTING.md**: 載っているのは、clippy・fmt が通ること、カバレッジ、公開アイテムの doc コメントだけ。設計や品質の指針は無い
- **`.claude/rules/rust-code.md`**: エージェント専用で、`src/`・`tests/` を編集するときだけ読み込まれる。中身は個別ルールで、`unwrap` 禁止、thiserror/anyhow、doctest、TempDir でモックなし
- **個別ルールの増え方**: 事故ごとに足していくと、量が増える一方になる

## 決めたこと（宍戸さん、2026-10-07）

自前の原則は書かない。定評のある一般的な指針を、参照として載せる。

- **主な指針**: Google Engineering Practices の "What to look for in a code review"（https://github.com/google/eng-practices/blob/master/review/index.md ）
  - 観点は、設計・機能・複雑さ（過剰な一般化を含む）・テスト・命名・コメント・スタイル・ドキュメント
  - 書くときとレビューするときに、同じ問いを使う
- **公開ライブラリ API を変えるときの参照**: Rust API Guidelines（https://rust-lang.github.io/api-guidelines/ ）
  - Renga は主に CLI で、公開している API が小さいので、全面には適用しない

### 見送った案

このセッションの事故から原則を5つ作る案（「データはその形式として扱う」など）は見送った。事故の対策を言い換えただけで、個別的すぎるため。

## やること

1. CONTRIBUTING.md に、品質の指針の節を作る。上の2つへの参照と、「レビューはこの観点で行う」を数行で書く。規約の正はここに置く
2. `.claude/rules/rust-code.md` を整理する
   - 上の指針で足りるものは消す
   - 機械で判定できるものは clippy の設定に移す
   - Renga 固有の決定だけを残し、CONTRIBUTING.md を参照させる（例: モックを使わず実ファイルシステムでテストする）
3. review エージェント（`.claude/agents/review.md`）の観点を、Google のガイドに揃える
4. CLAUDE.md・AGENTS.md の案内を、新しい置き場所に合わせる。純増は0にする

`.claude/` を変えるので、self-improve を通して実施する。書く前に、2つの指針の一次資料を読んで中身を確かめる。

## 再検討の条件

指針を載せたあとも同じ種類のレビュー指摘が続いたら、指針の置き場所か使い方を見直す。目安は、同じ観点の指摘が3回以上である。

