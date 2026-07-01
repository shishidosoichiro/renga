---
schema_version: 1
status: done
priority: medium
area: core
labels: [found_in_impl]
---

# make_slug の char_class がイテレーション記号「々」(U+3005) を Word 扱いにし複合語を誤って分割する

## 問題

`src/issue.rs` の `char_class` は ひらがな(U+3040-309F)・カタカナ(U+30A0-30FF)・漢字(U+3400-4DBF/U+4E00-9FFF/U+F900-FAFF) をカバーするが、繰り返し記号「々」(U+3005 IDEOGRAPHIC ITERATION MARK) はいずれの range にも含まれず catch-all の `CharClass::Word` に分類される。

「々」は `char::is_alphanumeric()` が `true` を返すため `replace_non_alnum_runs` で除去されず、slug 中にそのまま残る（`make_slug("色々") == "色々"` で確認済み）。そのため「々」を含む語（色々・時々・我々・人々・山々・各々・近々・日々 など、実際の日本語で非常に頻出する複合語）がちょうど30文字カットオフ付近にあると、「々」の直前・直後を Han とは異なるクラスの境界とみなし、複合語を真っ二つに分割してしまう。

## 再現

```rust
let title = format!("{}色々問題です", "あ".repeat(28)); // 34 chars
make_slug(&title) // => "...色" 「々」が失われ「色」だけが残る
```

`cargo run --example` で実際に確認済み（一時ファイルで検証、コミットはしていない）。

## 対応案

`char_class` の Han 判定に U+3005（IDEOGRAPHIC ITERATION MARK）を追加する。余裕があれば U+3006（IDEOGRAPHIC CLOSING MARK, 〆）、U+303B（VERTICAL IDEOGRAPHIC ITERATION MARK）も検討する。

## 撤回

本 issue が対象としていた `char_class`/スクリプト境界検出ロジックは #214 の設計見直しにより丸ごと撤回された。バイト長ハードカット方式（80バイト、`str::is_char_boundary` で丸める）に置き換えたため該当コードが存在しない。
