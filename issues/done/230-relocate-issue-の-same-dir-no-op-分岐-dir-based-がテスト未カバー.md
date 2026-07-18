---
schema_version: 1
status: done
priority: medium
area: test
labels: [found_in_impl]
---

# relocate_issue の same-dir no-op 分岐 (dir-based) がテスト未カバー

`src/issue.rs` の `relocate_issue` に、ディレクトリベース issue で `src_root == dst_root`（移動先が現在地と同じ）のとき rename をスキップして `write` のみ行う分岐がある（旧実装にはこの分岐は存在せず、常に無条件で rename していた）。

`cargo llvm-cov --summary-only -- --test-threads=1` で `src/issue.rs` の missed lines に 750 行目（`if src_root != dst_root { std::fs::rename(...)?; }` の閉じ括弧）が含まれており、この分岐がテストで一度も通っていないことを確認した。

再現ケース:
- `renga update <id> --status open`（ディレクトリベース issue が既に `open/` にある状態で同じステータスに更新）
- `renga reopen <id>`（ディレクトリが既に `open/` にあるが frontmatter の status が open 以外）

手動確認では正しく動作した（`update 1 --status open` を dir-based issue に対して実行し、`open/1-task/README.md` が壊れず存在することを確認）。バグではないが、リファクタで新設された分岐のためテストを追加すべき。

## 対応案

`tests/integration.rs` に以下を追加する:
- `update <id> --status <現在と同じstatus>`（dir-based issue）で成功し、issue が壊れないことを確認するテスト
