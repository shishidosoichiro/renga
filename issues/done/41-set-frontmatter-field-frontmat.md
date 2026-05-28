---
status: done
priority: medium
area: core
labels: []
---

# set_frontmatter_field が frontmatter なしコンテンツの本文中 --- ブロックを誤って frontmatter として扱う

## 概要

`src/issue.rs` の `set_frontmatter_field` は、コンテンツが frontmatter（`---` で始まる YAML ブロック）を持たず、本文中に `---` が 2 回出現する場合、本文内のフィールドを誤って書き換える。

## 問題箇所

`src/issue.rs` 373–383 行目（`set_frontmatter_field` 関数）:

```rust
for line in content.lines() {
    if !fm_closed && line.trim() == "---" {
        if in_fm {
            in_fm = false;
            fm_closed = true;
        } else {
            in_fm = true;  // ← frontmatter なし本文の最初の "---" で in_fm = true になる
        }
        ...
    }
}
```

`fm_closed` は一度 `---` で閉じた後は再トグルしないが、**最初の `---` が frontmatter の開き `---` かどうかを確認していない**。コンテンツの先頭が `---` でない場合でも、本文中の最初の `---` で `in_fm = true` になり、次の `---` までの区間が frontmatter として扱われる。

## 再現シナリオ

```rust
let content = "# Title\n\n---\nstatus: open\n---\n";
let result = set_frontmatter_field(content, "status", "done");
// 期待: content が変更されない（frontmatter がないため）
// 実際: "# Title\n\n---\nstatus: done\n---\n"（本文が書き換えられる）
```

## 修正方法

frontmatter の開き `---` はコンテンツの**最初の行**でなければならない（YAML front matter の仕様）。`in_fm = true` にする条件を「最初の行でかつ先頭行が `---`」に限定する。

例: `in_fm` を true にするのは `out.is_empty()` の場合のみとする。

```rust
} else if out.is_empty() {
    in_fm = true;
}
```
