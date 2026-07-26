---
schema_version: 1
status: done
priority: medium
area: docs
labels: []
---

# docs: renga migrate help/README description is stale after 3-step migrate

`renga migrate --help` (via the doc comment on the `Migrate` variant in `src/cli.rs`) and the command table in `README.md`/`README.ja.md` both still describe `migrate` as:

> Migrate issues from flat layout to per-status directories

This was already incomplete after `group_by` (#229) added a second relocation step (area/status), but issue #236 adds a third responsibility — converting flat issues to directory-based when `defaults.dir: true` — without updating this text. Verified against the built binary:

```
$ cargo run -q -- help | grep migrate
  migrate      Migrate issues from flat layout to per-status directories

$ cargo run -q -- migrate --help
Migrate issues from flat layout to per-status directories

Usage: renga migrate
```

`spec.md`/`spec.ja.md`'s prose (the `group_by`/`defaults` config sections) does describe all three steps correctly, and `migrate.rs`'s own top-of-function doc comment was updated in #236 to describe all three steps — only the clap-facing `--help` text and the README command table are stale.

## Fix

Update the short description on the `Migrate` clap variant (`src/cli.rs`) and the corresponding row in `README.md`/`README.ja.md`'s command table to mention all three steps (status move; directory conversion when `defaults.dir: true`; area/status relocation when `group_by` is set), or point to `spec.md`/`spec.ja.md` for the full behavior.

## Related

- `src/cli.rs` (`Migrate` variant doc comment, ~line 74)
- `README.md:136`, `README.ja.md:136`
- Found during review of issue #236's uncommitted implementation.
