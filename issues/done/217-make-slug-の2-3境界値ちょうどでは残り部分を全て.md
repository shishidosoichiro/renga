---
schema_version: 1
status: done
priority: low
area: core
labels: [found_in_impl]
---

# make_slug の2/3境界値ちょうどでは残り部分を全て捨てる挙動をテストで確認していない

## 背景

issue #214 の実装レビュー。`make_slug`（`src/issue.rs`）に「最後の `-` が予算の2/3(20/30文字)以上を保持できる場合のみ区切り文字カットする」というガードを追加したが、この閾値のちょうど境界（保持文字数が min_boundary と一致するケース）を検証するテストが無い。

## 再現

`make_slug("aaaaaaaaaaaaaaaaaaaa bbbbbbbbbbbbbbbbbbbb")` で確認（20文字の語 + スペース + 20文字の語）。

- 30文字にハードカットすると `"aaaaaaaaaaaaaaaaaaaa-bbbbbbbbb"`（30文字、2語目の頭9文字を含む）
- 区切り文字は境界（chars before dash == 20 == min_boundary）にちょうど乗るため `>=` 条件が成立し、区切り文字カットが発動
- 結果は `"aaaaaaaaaaaaaaaaaaaa"`（20文字）となり、2語目の情報が完全に失われる

実際に一時テストを追加して確認済み: `PROBE_SLUG="aaaaaaaaaaaaaaaaaaaa" LEN=20`

## 問題点

- 境界値ちょうど（`>=`の等号が効くケース）をカバーするテストが既存の2件（`make_slug_truncates_at_late_boundary_instead_of_mid_word`, `make_slug_keeps_hard_cut_when_boundary_is_too_far_back`）に無い。両方とも境界から離れた値（27文字保持 / 9文字保持）でしか検証していない。
- 境界ちょうどの場合、2語目の内容を一切残さず切り捨てる（ハードカットなら9文字だけでも残る）。「予算の2/3以上保持できるなら区切り文字を使う」というルールが、捨てる側の量ではなく残す側の量だけを見ているため、捨てる部分がまるごと1つの意味的まとまり（単語・フレーズ）である場合に、それを全部失っても「良し」と判定してしまう。実際の日本語タイトルで英数字プレフィックスがちょうど20文字前後になるケース（例: 短い issue 番号 + 固有名詞 + 長めの日本語フレーズ）で、日本語フレーズの冒頭が一切残らない結果になり得る。

## 対応案

- 境界ちょうどのケースを既存のテストスイートに追加する（`chars_before_dash == min_boundary` を明示的に検証）
- 加えて、「区切り文字カットによって失われる文字数」にも下限（例: 失われるのは最大でも1/3、かつ捨てる部分の量が全体の意味を左右しない程度に小さいこと）を設けるか、閾値の妥当性を再検討する。単純な比率だけでなく、既存の best practice（例: 最後の空白の位置が十分後方にある場合のみ使う、という一般的な "truncate at last space if it's not too far from the end" パターン）で許容される損失幅を確認し直す

## 撤回

本 issue が対象としていた「区切り文字カット + 予算2/3ガード」ロジックは #214 の設計見直しにより丸ごと撤回された。バイト長ハードカット方式（80バイト、`str::is_char_boundary` で丸める）に置き換えたため該当コードが存在しない。
