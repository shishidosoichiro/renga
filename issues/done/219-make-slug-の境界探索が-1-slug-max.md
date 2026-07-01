---
schema_version: 1
status: done
priority: medium
area: core
labels: [found_in_impl]
---

# make_slug の境界探索が (1..SLUG_MAX_CHARS) で chars[29]/chars[30] の境界を見落とし不要に短く切る

## 問題

`make_slug` の切り詰めロジック:

```rust
let cut = (1..SLUG_MAX_CHARS)
    .rev()
    .find(|&i| char_class(chars[i - 1]) != char_class(chars[i]))
    .unwrap_or(SLUG_MAX_CHARS);
```

`1..SLUG_MAX_CHARS`（= `1..30`）は `i` が 29 までしか回らないため、`chars[29]` と `chars[30]`（30文字目と31文字目、つまりハードカット境界そのもの）を比較するペアが一切検査されない。

その結果、「ちょうど30文字目でスクリプトが切り替わっており、30文字そのままハードカットしても単語境界を壊さない」という理想的なケースでも、30文字より手前にある別の（より短い）内部境界を優先して選んでしまい、本来落とす必要のないテキストまで不必要に切り捨てる。

## 再現

```rust
let title = "abcdefghijklmnopqrstuvwxy-wxyz漢字ですね"; // 35 chars
// chars[29] = 'z' (Word), chars[30] = '漢' (Han) → 30文字目で自然な境界がある
make_slug(title)
// => "abcdefghijklmnopqrstuvwxy" (25文字)
// 本来は "abcdefghijklmnopqrstuvwxy-wxyz" (30文字) を保持できるはず
// "-wxyz" (5文字) が不要に失われる
```

`cargo run --example` で実際に確認済み（一時ファイルで検証、コミットはしていない）。

## 対応案

範囲を `1..=SLUG_MAX_CHARS` に変更し、`chars[SLUG_MAX_CHARS - 1]` と `chars[SLUG_MAX_CHARS]` の境界も候補に含める（`chars.len() > SLUG_MAX_CHARS` が保証されているため `chars[SLUG_MAX_CHARS]` へのアクセスは安全）。

## 関連

#217 と同種（「境界に頼る截断が、本来より多くの情報を不必要に失う」）の別原因によるバグ。#217 の脅威モデルとの切り分けをレビューで指摘済み。

## 撤回

本 issue が対象としていた境界探索ロジックは #214 の設計見直しにより丸ごと撤回された。バイト長ハードカット方式（80バイト、`str::is_char_boundary` で丸める）に置き換えたため該当コードが存在しない。
