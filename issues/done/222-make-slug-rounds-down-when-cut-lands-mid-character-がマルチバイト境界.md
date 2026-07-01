---
schema_version: 1
status: done
priority: medium
area: test
labels: [found_in_impl]
---

# make_slug_rounds_down_when_cut_lands_mid_character がマルチバイト境界の丸め込みを実際には検証していない

## 問題

issue #214 の実装レビュー。`src/issue.rs` の新規テスト
`make_slug_rounds_down_when_cut_lands_mid_character` は、名前とコメントで
「カットオフがマルチバイト文字の途中に来た場合に丸め込む」ことを検証すると
主張しているが、実際にはそのシナリオを検証できていない。

## 再現

テストの入力: `format!("{} あ", "a".repeat(79))`

`replace_non_alnum_runs` によりスペースは単一の `-` に変換されるため、
カット前の slug は `"a"*79 + "-" + "あ"`（83バイト）になる。
`SLUG_MAX_BYTES = 80` により `cut = 80` となるが、この位置は
ダッシュの直後・「あ」の直前という **文字境界そのもの** であり、
`slug.is_char_boundary(80)` は `true` を返す。

```
title bytes len = 83
boundary at 79 = true   (dash の直後)
boundary at 80 = true   ← cut はここに一致し、while ループは1回も減算しない
boundary at 81 = false
boundary at 82 = false
boundary at 83 = true
```

つまり `while cut > 0 && !slug.is_char_boundary(cut) { cut -= 1; }` の
減算本体は一度も実行されず、テストが実際に検証しているのは
「カット位置に露出したダッシュを `trim_end_matches('-')` で除去する」
という別の挙動（`make_slug_no_trailing_dash_after_truncation` と重複）である。

一方、`make_slug_truncates_japanese_by_byte_length_not_char_count`
（全角ひらがな40文字）は cut が 80→79→78 と2回減算されるため、
丸め込みロジック自体はテストスイート全体としては実行されている。
問題は「このテストの名前・コメントが実際の検証内容と一致していない」こと。

## 対応

- テストを名前通りの内容（ASCII 接頭辞 + 区切り文字なしで直接マルチバイト文字が
  cut 位置をまたぐケース、例: `"a".repeat(79) + "あ"` のようにスペースを挟まない入力）
  に組み直す、または
- テスト名・コメントを実際に検証している内容（切り詰め後に露出したダッシュの除去）
  に合わせて修正する

