---
schema_version: 1
status: done
priority: medium
area: test
labels: [found_in_impl]
---

# group_by 有効時の unparseable frontmatter 経路（done/pending/in-progress/reopen/migrate）のテストが不足している

## 問題

issue #232 の回帰（done/pending/in-progress が不正な frontmatter で失敗する）は、テストで検出できなかった。関連して、以下の経路にもテストが存在しない:

1. `reopen.rs` の unparseable frontmatter フォールバック（`Issue::parse(&path, &content).ok()` → area="" にフォールバックする分岐）。手動確認では正しく動作していたが、回帰を防ぐテストがない。
2. `migrate.rs` の group_by 移行ステップ（ステップ2）で `Issue::parse` が失敗した場合の分岐（area="", status="unknown" にフォールバックし `issues/unknown/` へ配置する経路）。
3. `migrate.rs` の group_by 移行ステップ（ステップ2）での移動先衝突検出（`dest_entry.exists()` で警告してスキップする分岐）。手動確認では正しく動作していたが、テストがない。

## 期待される対応

- #232 の修正（done/pending/in-progress のフォールバック追加）と合わせて、4コマンド共通の「unparseable frontmatter でも area="" にフォールバックして動作する」ことを保証する回帰テストを追加する
- migrate.rs のステップ2について、unparseable frontmatter フォールバックと衝突スキップの統合テストを追加する

`cargo llvm-cov --summary-only -- --test-threads=1` では migrate.rs 89.51%、reopen.rs 85.23% で、他ファイルより低い（issue.rs 93.18%、config.rs 99.30%）。上記の未テスト分岐が主因とみられる。
