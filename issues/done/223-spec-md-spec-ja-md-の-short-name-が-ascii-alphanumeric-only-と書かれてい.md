---
schema_version: 1
status: done
priority: medium
area: docs
labels: [found_at:0.15.0, fixed_at:0.16.0]
---

# spec.md/spec.ja.md の short-name が ASCII alphanumeric only と書かれているが実装は Unicode 英数字を許容する

## 問題

`spec.md` / `spec.ja.md` は `short-name` の文字種を
「ASCII alphanumeric and hyphens only」（英数字・ハイフンのみ）と記述しているが、
実装（`make_slug`, `src/issue.rs`）は Unicode の英数字（日本語含む）を保持する。

```
spec.md:41:  - `short-name`: Kebab-case short description (ASCII alphanumeric and hyphens only, up to 30 characters).
```

```rust
// src/issue.rs (make_slug の doc comment)
/// Unicode alphanumeric characters are preserved, so Japanese and other
/// non-ASCII titles produce meaningful slugs.
```

## 経緯

`git blame` によると spec.md の該当行は 2a4a9f8a（2026-05-26）から存在するが、
Unicode 文字を保持する挙動は commit 876fa39「fix: preserve Unicode characters in
auto-generated slugs」で導入された。876fa39 は `v0.14.0` の後・`v0.15.0` の前に
取り込まれ、`v0.15.0` としてリリース済み。したがって現行リリース（v0.15.0,
`Cargo.toml` の version と一致）以降、spec と実装の記述が食い違ったままになっている。

## 対応

`spec.md` / `spec.ja.md` の該当箇所を Unicode 英数字を許容する旨に修正する。

