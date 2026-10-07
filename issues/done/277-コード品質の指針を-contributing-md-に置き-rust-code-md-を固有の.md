---
type: Issue
schema_version: 1
status: done
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


## 実施結果（2026-10-07）

一次資料（Google "What to look for in a code review"・review/index.md、Rust API Guidelines checklist）を読んでから書いた。CONTRIBUTING.md に "Code quality" 節を作り、指針はリンクだけを置いた（中身を写さない）。

### rust-code.md の振り分け

| 項目 | 振り分け | 理由 |
|---|---|---|
| `unwrap()` / `expect()` はテスト外で使わない | (b) clippy | `Cargo.toml` の `[lints.clippy]` に `unwrap_used`・`expect_used` を deny で置き、`clippy.toml` の `allow-*-in-tests` でテストを除外した。`tests/integration.rs` の共有ヘルパーは `#[test]` 関数でないため除外が効かず、クレート単位の `allow` を理由のコメント付きで置いた |
| `#![deny(missing_docs)]` を `src/lib.rs` に置く | (b) 既に機械判定 | コードに既にある。規則の文は不要 |
| 公開アイテムに `///` と `# Examples`（doctest） | (a) 削除 | Rust API Guidelines の C-EXAMPLE・C-FAILURE が公開 API について同じことを求める |
| 非公開アイテムは WHY が自明でないときだけ `//` | (a) 削除 | Google ガイドの Comments（what より why）で足りる |
| ユーザー向けエラーは `error:` を付けて stderr、exit 1 | (c) 残す | CLI の出力契約で、指針からは導けない |
| ドメインエラーは thiserror、アプリエラーは anyhow | (c) 残す | 依存クレートの選択。C-GOOD-ERR は型の性質を求めるだけで、選択は決めない |
| FS を伴うテストは TempDir、モックなし | (c) 残す | Renga の決定 |
| 統合テストは `tests/` | (c) 残す | 「`renga` バイナリを実行する」という Renga の方針と合わせて残した |

(c) は CONTRIBUTING.md に移した。`.claude/rules/rust-code.md` は参照だけを残す案もあったが、削除した。CLAUDE.md・AGENTS.md が `@CONTRIBUTING.md` で全セッションに読み込むので、参照だけの rules は何も足さない。

AGENTS.md にあった rust-code.md と同じ3節（エラーハンドリング・ドキュメント・テスト方針）も、CONTRIBUTING.md への参照1文に置き換えた。

### 付随する変更

- CONTRIBUTING.md の lint コマンドを `cargo clippy --all-targets -- -D warnings` にした。CI は未対応で、#278 に起票した
- review エージェントの観点を Google のガイドの観点に揃え、`unwrap` の目視確認は clippy に任せて削った

### 見送ったもの

- src/ の修正: 不要だった。src/ の `unwrap`/`expect` はすべて `#[cfg(test)]` 内で、lint を通った
- `missing_docs` を `Cargo.toml` の `[lints.rust]` に移す案: 試すと `src/main.rs` と `tests/integration.rs` にもクレートの doc を求めて失敗した。現状の `src/lib.rs` の属性で足りるため見送った。再検討の条件は、lint の置き場所を `Cargo.toml` に一本化したくなったとき

### レビュー（agent-config-reviewer）を受けて直したもの

- `tests/integration.rs` の `allow` に `reason = "..."` を持たせた。CLAUDE.md の「`#[allow(...)]` で黙らせない」と字面が衝突するため、方針の実装であることを属性に残す
- review.md で Google のガイドの観点名を列挙していたのをやめ、リンク先の観点に従う書き方にした（一次資料には Consistency・Every Line・Context・Good Things もあり、列挙は写しになる）
- README.md・README.ja.md の開発コマンドも `--all-targets` に揃えた

### retro #173 との関係

#173（全体レビューで unwrap 観点の確認結果を報告し漏らした）の根拠は、AGENTS.md のエラーハンドリング規約と review.md の unwrap 項目だった。今回、unwrap の確認は clippy の deny に移り、目視の確認項目は無くなった。#173 は skipped-step 型の retro として改善 issue の流れで扱うため、ここでは close しない。
