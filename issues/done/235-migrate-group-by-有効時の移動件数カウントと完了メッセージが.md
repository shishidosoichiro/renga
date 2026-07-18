---
schema_version: 1
status: done
priority: low
area: core
labels: []
---

# migrate: group_by 有効時の移動件数カウントと完了メッセージが不正確

## 問題1: 移動件数の二重カウント

`group_by` 有効時、フラット直下のファイル（例: `issues/1-task.md`）は migrate 内でステップ1（フラット→status）→ステップ2（status→area/status）と2段階で移動する。この場合、同じ1ファイルの移動で `moved` カウンタが2回インクリメントされ、`Migrated 2 issue(s).` のように実際のファイル数より多く表示される。

再現（手元で確認済み）:

```sh
mkdir -p issues
cat > .renga.yml <<YML
group_by: [area]
YML
printf -- '---\nschema_version: 1\nstatus: open\npriority: medium\narea: core\nlabels: []\n---\n\n# Task\n' > issues/1-task.md
renga migrate
# => "Migrated 2 issue(s)." だが実際に移動したのは1ファイルのみ（issues/core/open/1-task.md に正しく配置される）
```

最終的な配置は正しいが、報告件数が実態と一致しない。

## 問題2: 全件スキップ時の完了メッセージが変化

今回の diff で `migrate.rs` の末尾に `if moved == 0 { println!("Nothing to migrate."); return Ok(()); }` が追加された。これにより、group_by の有無に関わらず、既存のフラット→status 移行(ステップ1) が衝突で全件スキップされた場合の最終メッセージが `Migrated 0 issue(s).`（変更前）から `Nothing to migrate.`（変更後）に変わった。

この変化はどのテストでも assert されておらず、CHANGELOG/spec にも記載がない。意図的な改善であれば問題ないが、未検証・未文書化の出力変更である。

## 期待される対応

- 二重カウントを避けるため、ステップ1で移動したファイルをステップ2で再カウントしないようにするか、最終レポートを「distinct file 数」ベースにする
- 完了メッセージの変更を意図的なものとして受け入れるならテストで固定し、CHANGELOG（次回リリース時）に記載する
