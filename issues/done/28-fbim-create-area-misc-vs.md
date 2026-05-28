---
status: done
priority: high
area: cli
labels: []
---

# fbim create --area のデフォルト値が仕様と不一致（'misc' vs 空文字）

## 問題

src/cli.rs 83行目の CreateArgs::area フィールドに `#[arg(long, default_value = "misc")]` が設定されている。

spec.md 44行目では「Omitting 'area' places the issue in no group (shown without a heading in issues/README.md)」と明記されており、--area を省略した場合のデフォルトは空文字（グループなし）であるべき。

## 再現

`fbim create "test"` を実行すると area が "misc" で作成される。

仕様では area を省略した場合はグループなし（空文字）になるべき。

## 関連箇所

- src/cli.rs:83: `#[arg(long, default_value = "misc")]`
- spec.md:44
- spec.ja.md:44
