---
schema_version: 1
status: done
priority: high
area: core
labels: [found_in_impl]
---

# done/pending/in-progress は不正な YAML frontmatter の issue で失敗するようになった（group_by 実装での回帰）

## 問題

`src/commands/done.rs`/`pending.rs`/`in_progress.rs` の `move_one` に `let issue = Issue::parse(&path, &content)?;` が追加された（group_by 対応で area を取得するため）。

これにより、以前は `set_frontmatter_field`（文字列ベースの置換、YAML パース不要）だけで移動できていた、frontmatter が不正な YAML の issue に対して `renga done`/`pending`/`in-progress` が `error: invalid frontmatter in ...` で失敗するようになった。

`src/commands/reopen.rs` は同じ問題に対して明示的に対処済み（`Issue::parse(&path, &content).ok()` で area="" にフォールバックし、コメントで「mirrors the pre-group_by behavior」と明記）。しかし done/pending/in-progress には同じフォールバックが入っていない。

## 再現手順（手元で確認済み）

```sh
mkdir -p issues/open
printf -- '---\nnot: valid: yaml: [\n---\n\n# Bad\n' > issues/open/1-bad.md
renga done 1
```

- HEAD（このブランチの変更前）: exit 0, `issues/done/1-bad.md` に移動して成功
- このブランチ（未コミット差分）: exit 1, `error: invalid frontmatter in .../issues/open/1-bad.md`

## 期待される修正

done.rs/pending.rs/in_progress.rs でも reopen.rs と同様に `Issue::parse(...).ok()` でフォールバックし、area="" として canonical_dir を計算する（フラット配置になる）。

## 根拠

- `.renga.yml` 未設定（group_by オフ）時は「既存ユーザーへの影響はない」が今回の設計目標（issues/open/229-...md 本文、spec.md/spec.ja.md の group_by 節）。この回帰はその前提に反する。
- `tests/integration.rs` に done/pending/in-progress を不正な frontmatter に対して実行するテストが存在しない（reopen 側にも同様のテストがない — 別 issue で追跡）。

found_in_impl: 今回の group_by 実装サイクルで混入した回帰。コミット前に修正しfeatureコミットに含めること。
