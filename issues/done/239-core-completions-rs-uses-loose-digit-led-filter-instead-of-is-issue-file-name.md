---
schema_version: 1
status: done
priority: low
area: core
labels: []
---

# core: completions.rs uses loose digit-led filter instead of is_issue_file_name

While reviewing issue #236's fix for the `migrate` dedup regression (migrate's flat-file filter used to accept any top-level `.md` file starting with an ASCII digit, without requiring the `N-slug` hyphen that `id_prefix`/`is_issue_file_name` require — fixed by switching to `is_issue_file_name`), I found the same loose pattern still present in `src/commands/completions.rs::emit_issues_recursive`:

```rust
.filter(|e| {
    let name = e.file_name().to_string_lossy().into_owned();
    name.ends_with(".md")
        && name
            .chars()
            .next()
            .map(|c| c.is_ascii_digit())
            .unwrap_or(false)
})
```

and further down:

```rust
let stem = path.file_stem().unwrap_or_default().to_string_lossy();
let id = stem.split('-').next().unwrap_or("").to_string();
```

For a malformed file like `5foo.md` (digit-led, no hyphen — not a valid issue per `id_prefix`), this filter accepts it, and `stem.split('-').next()` yields the whole stem `"5foo"` as a bogus completion "ID" rather than `None`/skip.

This is lower severity than the migrate bug: `emit_issues_recursive` just writes `id\ttitle` lines for shell completion, with no `HashSet`-keyed dedup, so there's no silent undercount like the migrate regression. But it's the same class of inconsistency — deriving an issue ID from a filename without going through `is_issue_file_name`/`issue_file_id` (renga's actual ID grammar, used by `collect_issue_files` and now `migrate`'s `flat_files` filter) — and it will offer bogus completion candidates for any malformed top-level `.md` file.

## Suggested fix

Replace the ad-hoc digit/split-based filtering in `emit_issues_recursive` with `issue::is_issue_file_name` / `issue::issue_file_id`, mirroring how `collect_issue_files` and `migrate.rs` do it.

## Related

- `src/commands/completions.rs` (`emit_issues_recursive`, ~lines 274-287)
- `src/issue.rs::is_issue_file_name`, `issue_file_id`, `id_prefix`
- Not part of issue #236's diff — found via a broader sweep for the same bug class while confirming #236's fix was complete.
