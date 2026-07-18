---
schema_version: 1
status: done
priority: low
area: core
labels: [found_in_impl]
---

# validate --auto-correct が status ディレクトリ修正時に不要なファイル書き換え(mtime変化)を伴う

`src/commands/validate.rs` の `correct_status_directory` は、旧実装では `std::fs::rename` のみで issue を正しい status ディレクトリへ移動していた（内容の書き換えなし）。

共有ヘルパー `relocate_issue`（`src/issue.rs`）への統合後は、`relocate_issue(&issue.path, &issue.raw_content, &dest_dir)` を呼ぶため、必ず一度 `std::fs::write` で内容を書き戻してから rename する。

- `issue.raw_content` はディスク上の内容とバイト単位で同一なので、**内容自体**は変化しない。
- ただし副作用として:
  - ディレクトリベース issue: `README.md` を書き換えてからディレクトリごと rename するため、`README.md` の mtime が更新される
  - フラットファイル issue: 従来は単一の `rename`（同一 inode を維持）だったが、新実装は「tmp ファイルへ書き込み → rename → 元ファイル削除」という別ファイルの生成になるため、mtime だけでなく inode も変わる（プラットフォームによっては拡張属性やパーミッションが引き継がれない可能性がある）

挙動的な実害（コンテンツ破損・データロス）はないが、「純粋なリファクタリング（挙動変化なし）」という前提から外れる副作用のため、許容するかどうかの判断を記録する必要がある。

## 対応案

- 許容する場合: この issue に判断理由を記録してクローズする
- 避けたい場合: `relocate_issue` に「rename のみ・書き換え不要」の軽量パスを用意する（例: `content` が現在のファイル内容と同一なら write をスキップする、または `relocate_issue` とは別に「純粋な rename だけ行う」バリアントを用意し `correct_status_directory` はそちらを使う）

## 判断

許容する。理由:

- `grep -rn "mtime\|modified()\|metadata()\|created()\|SystemTime" src/` の結果はゼロ件で、renga は mtime/inode をいかなる判定・表示・ソートにも使っていない。実害が存在しない。
- 「rename のみの軽量パス」を追加すると、今回の Phase 1 リファクタで解消したばかりの「dir-based/flat-file 分岐の重複」が `relocate_issue` 内にもう一段増える形で復活する。全呼び出し元（update/pending/done/in-progress/reopen/validate）が単一の関数で統一されているという今回のリファクタの価値と、mtime を保存する価値を比較すると後者が明らかに小さい。
- group_by（issue #229）実装後、`validate --auto-correct` は area 不一致の修正でも同じ経路を通ることになる。今のうちに `relocate_issue` を唯一の移動経路として確定させておく方が、将来の group_by 対応時に分岐が増えずに済む。

再検討条件: renga が将来 mtime/inode に依存する機能（例: 「最終更新順」ソート）を追加する場合、そのときに軽量パスを検討する。
