---
type: Issue
schema_version: 1
status: done
priority: medium
area: core
labels: []
---

# issue frontmatter に OKF の type キーを入れる

## 目的

issue の frontmatter に OKF の `type` キーを入れる。Renga の issue ディレクトリを OKF の bundle として読めるようにする第一歩にする。

## OKF の要点

Open Knowledge Format v0.2 の仕様は https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md 。

- `type` は唯一の必須キー。`type` だけを持つ文書も完全に準拠する（§4.1）
- type の値は中央登録しない。説明的な値を選ぶ。消費側は未知の type を許容しなければならない（MUST tolerate）
- 未知のキーを拒否してはならない（MUST NOT reject）。round-trip では保持すべき（SHOULD preserve）とされる

## Renga の現状

- frontmatter の更新は1フィールドずつ書き換え、再シリアライズしない（`src/issue.rs:702`）。そのため未知のキーは保持される
- `schema_version: 1` がある（spec.md）

## 吟味すべき論点

1. **値**: `type: issue` か `type: Issue` か。OKF の例は `BigQuery Table`・`Playbook` のような Title Case である
2. **書く範囲**: 新規作成時だけ書くのか、`migrate` で既存の issue にも足すのか
3. **schema_version**: 上げるかどうか。キーの追加だけなら上げない案もある
4. **読み込み時の扱い**: `type` が `issue` 以外のファイルを、issue として読むか、無視するか、警告するか。将来 ADR などを同じ木に置く余地に関わる
5. **status の語彙の衝突**: OKF の `status` は `draft | stable | deprecated` で、無いときは `stable` とみなす（§5.4）。Renga の `status: open/done` は OKF の消費側から見ると規定外の値になる。`type` だけ入れても、この点では完全には準拠しない
6. **予約ファイル名と索引**: OKF は `index.md`・`log.md` を予約している（§3.1）
   - Renga が生成する `issues/README.md` は、OKF の消費側には type の無い concept 文書に見える
   - dir 形式の `N-title/README.md` も concept 文書として扱われる。こちらは問題にならない
7. **title**: Renga のタイトルは本文の H1 にある。OKF は `title` が無ければファイル名から導いてよい（MAY）としており、矛盾はしない

## 背景

ADR を OKF・MADR（https://adr.github.io/madr/）互換の書式で書く運用が考えられる。ただし、Renga 自身の判断記録を ADR にすることは見送る。Renga では issue 本文の判断記録で足りており、置き場を二重にすると、記録どうしの整合を保つ仕組みが別に要るため。

代わりに `type` だけを先に入れ、OKF との互換に備える（宍戸さんの提案、2026-10-07）。

再検討の条件: OKF の status 語彙が改訂されたら見直す。Renga で issue 以外の文書種別を扱う要望が出たときも見直す。


## 決定と実装（2026-10-07）

- **値**: `type: Issue` にした。OKF の例（`Playbook`・`Metric` など）が Title Case のため
- **新規の issue**: create は frontmatter の先頭行に書く
- **既存の issue**: `renga migrate` の4段目で足す
  - 対象: frontmatter に `type` キーが無い issue
  - 足し方: 先頭行に挿入し、ほかの行は変えない
- **migrate が触らないもの**
  - すでに `type` を持つファイル。値は問わず、空や `null` でも触らない。足すとキーが重複するため
  - frontmatter が無いか、parse できないファイル
- **schema_version**: 据え置き。キーを足しただけで、既存の読み込みは変わらないため
- **読み込み**: `type` は必須にしない

### 今回やらなかったこと（再検討の条件付き）

- **validate で `type` の欠落を警告すること**: migrate していないリポジトリでは、全件に警告が出るため見送った。再検討するのは、`type` で文書の種類を分ける機能を入れるとき
- **`type` が `Issue` 以外のファイルの読み方**: 今は issue として読む。再検討するのは、issue 以外の文書を同じ木に置く要望が出たとき
- **OKF の `status` 語彙（`draft | stable | deprecated`）との衝突**: 未解決のまま残した。再検討するのは、OKF を読むツールとの実際の連携を考えるとき
- **OKF の予約ファイル名 `index.md` と、`issues/README.md` の関係**: 上と同じ条件で再検討する
- **`list --json` への `type` の出力**: 再検討するのは、要望が出たとき

