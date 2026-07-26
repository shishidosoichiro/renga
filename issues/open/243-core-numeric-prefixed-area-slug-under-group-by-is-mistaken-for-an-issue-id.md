---
schema_version: 1
status: open
priority: medium
area: core
labels: []
---

# core: numeric-prefixed area slug under group_by is mistaken for an issue ID

`group_by: [area]` のとき、area 名の slug が `N-slug` 形式（数字＋ハイフン始まり）になると、その area ディレクトリが issue ID として解釈される。

## 再現

```sh
renga init
printf 'group_by: [area]\n' > .renga.yml
renga create "Q1 task" --area "2024 Q1"   # -> issues/2024-q1/open/1-q1-task.md
renga create "Next"
# -> issues/open/2025-next.md   ← ID が 2 ではなく 2025 に飛ぶ
renga show 2024
# -> area ディレクトリを issue として掴もうとする
```

## 原因

`validate_area_for_group_by`（`src/issue.rs`）が検査しているのは予約ステータス名（`open`/`pending`/`in-progress`/`done`/`unknown`）との衝突のみで、数字プレフィックスを弾かない。

```rust
let slug = make_slug(area);
if Status::all_values().iter().any(|s| s.to_string() == slug) {
    anyhow::bail!("area '{area}' is not allowed: ...");
}
```

`make_slug` は非英数字を `-` に潰すだけなので `2024 Q1` → `2024-q1` となり、`id_prefix("2024-q1")` が `Some("2024")` を返す。`next_id`（ディレクトリ名からも採番する）と `find_issue`（dir 形式 issue を探す）が両方これを拾う。

なお `3d-rendering` は `3d` が全数字ではないので `id_prefix` が `None` になり安全。危険なのは `2024-roadmap`・`2024 Q1` のような「先頭セグメントが全部数字」のケース。

## 修正案と論点

`validate_area_for_group_by` に `id_prefix(&slug).is_some()` の拒否を足すのが根本対処。ただし:

- **breaking change** になる。既に `2024 Q1` のような area を使っているプロジェクトで `create`/`update` がエラーになる
- `migrate` の扱いを決める必要がある（予約名衝突と同様に「警告してスキップ」か、`validate` でエラー報告のみか）
- 決めたら `spec.md:193` / `spec.ja.md:182` の予約語の段落に併記する

## コミット種別

`group_by` を導入した 6fe52d7 は最新タグ v0.16.0 より後＝**未リリース**。したがってこの修正は `fix:` ではなく `git commit --fixup 6fe52d7` で積む（`fix:` にすると世に出ていない機能の「修正」が CHANGELOG に載る）。

## 関連

- #241（dir 形式 issue の中身を走査しない修正）の設計中に発見。本 issue はそれとは独立した、#241 の修正前から存在する採番・検索の問題
- #241 の走査ガードは「`N-slug` ディレクトリのうち、canonical status サブディレクトリを持たないものを issue とみなす」という判定なので、area ディレクトリが誤って issue 扱いされて配下が隠される事故は起きない（`README.md` の有無では判別していない — 当初その案で実装したが、area に README を置くと area 全体が消える穴があったためレビューで差し替えた）
- `src/issue.rs::validate_area_for_group_by`、`next_id`、`find_issue`

