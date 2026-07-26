---
schema_version: 1
status: done
priority: low
area: cli
labels: []
---

# docs: create --dir help text doesn't mention defaults.dir override

`renga create --help`'s `--dir` description (doc comment on `CreateArgs::dir` in `src/cli.rs`) reads:

```
--dir <DIR>
    Store the issue as a directory (`true`) or a flat file (`false`, default).

    When `true`, creates `N-title/README.md` instead of `N-title.md`, allowing additional files to be placed inside the directory.

    [possible values: true, false]
```

As of issue #236, "flat file (`false`, default)" is only accurate when `.renga.yml` has no `defaults.dir: true` set. When `defaults.dir: true` is configured, *omitting* `--dir` now produces a directory-based issue, not flat. A user running `renga create --help` in a project with `defaults.dir: true` has no way to discover this from the CLI itself — they'd need to already know to check `.renga.yml` or `renga info`.

This is low severity: `renga info` already surfaces the effective `defaults.dir` value, and `spec.md`/`spec.ja.md` document the interaction fully. But the `--help` text is the first place users look, and it currently states the wrong effective default silently whenever project config overrides it.

## Suggested fix

Add a short note to the `--dir` doc comment, e.g.:

> Store the issue as a directory (`true`) or a flat file (`false`). Falls back to `defaults.dir` in `.renga.yml` when omitted (see `renga info`); flat if neither is set.

## Related

- `src/cli.rs` (`CreateArgs::dir` doc comment, ~line 119-125)
- Found during review of issue #236's uncommitted implementation.
