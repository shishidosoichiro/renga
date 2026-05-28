---
status: done
priority: medium
area: test
labels: []
---

# cargo fmt --check が set_frontmatter_field_ignores_hr_in_body テストで失敗する

## 概要

`cargo fmt --check` が `src/issue.rs` のテスト `set_frontmatter_field_ignores_hr_in_body` で失敗する。

## 問題箇所

`src/issue.rs` 479–480 行目:

```rust
assert!(updated.contains("status: done"), "frontmatter status should be updated");
assert!(updated.contains("status: see notes"), "body line must not be modified");
```

`rustfmt` は行長制限（デフォルト 100 文字）を超えているため、以下のように展開することを要求する:

```rust
assert!(
    updated.contains("status: done"),
    "frontmatter status should be updated"
);
assert!(
    updated.contains("status: see notes"),
    "body line must not be modified"
);
```

## 再現手順

```sh
cargo fmt --check
```

## 修正方法

`rustfmt` の出力に合わせて `assert!` マクロを複数行に展開する。
