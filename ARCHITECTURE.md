# Architecture

This document describes the high-level architecture of renga: where things live and which rules the code relies on. It only covers things that rarely change. If you make a change that contradicts something written here, update this document in the same commit.

## Overview

Renga is a command-line tool that keeps each issue as a Markdown file with YAML frontmatter under an issues directory, and its commands list, show, create, edit, and move those files.

Renga talks to two things only: the filesystem and the terminal. It reads the issues directory and an optional `.renga.yml` at the project root. It writes issue files, the status directories that hold them, and a generated index, `issues/README.md`. It has no database, no server, and no network access.

An issue is either a single file (`issues/open/12-fix-login.md`) or a directory-based issue (`issues/open/15-add-export/README.md`) whose directory can also hold attachments. Issue files sit in one directory per status. The full layout, including the optional area level, is described in [spec.md](spec.md).

Other documents:

- What each command does, and what each frontmatter key means: [spec.md](spec.md)
- How to install and use renga: [README.md](README.md)
- How to build, test, and release: [CONTRIBUTING.md](CONTRIBUTING.md)

## Code map

Each subcommand lives in its own file under `src/commands/`, and the code that finds, parses, rewrites, and places issue files lives in `src/issue.rs`. A command runs through these calls:

```
main.rs  main                  prints any returned error and exits with code 1
  → lib.rs  run                finds the project root, loads .renga.yml, builds Context
    → commands/<name>.rs  run  does the work through functions in issue.rs
```

### `src/cli.rs`

The clap definitions: `Cli`, the `Command` enum, and one `*Args` struct per subcommand. The hidden `__complete` subcommand serves dynamic shell completion; `commands/completions.rs` implements it.

### `src/lib.rs`

`run` dispatches to the command handlers. `Context` carries the project root, the issues directory, and the loaded `Config` to the handlers that work on issues. `FbimError` holds the errors specific to renga, such as an unknown issue ID; its name comes from the project's former name, File-Based Issue Management.

### `src/project.rs` and `src/config.rs`

`find_project_root` walks up from the current directory. It stops at the first directory that contains `.renga.yml` or `issues/`, and checks `.renga.yml` first in each directory. `Config` is the parsed `.renga.yml`; its `Defaults` hold the values used when a flag is omitted.

### `src/issue.rs`

The core of renga. The entry points are the `Issue` type, which a file parses into, and the lookup functions `find_issue`, `find_active_issue`, and `find_editable_issue`. The rest of the module lists issue files, assigns IDs, edits frontmatter, and decides where a file belongs.

**Architecture invariant:** frontmatter `status` is meant to be the source of truth, and the status directory mirrors it. Lookup by ID is the one place that looks at the directory first: whether an issue counts as done is decided by whether it sits under `done/`. A file under `done/` whose frontmatter says it is active is the only mismatch that lookup resolves in favor of the frontmatter.

**Architecture invariant:** frontmatter is edited as YAML through a lossless syntax tree (the `yaml-edit` crate), never as lines of text. Only the values an edit touches change; comments, key order, keys that renga does not know, and the formatting of other keys survive. A value is replaced whole whatever its YAML style, and an edit whose result would not parse back is refused. Frontmatter that is not valid YAML is not edited.

**Architecture invariant:** the ID lives only in the file name, and the title lives only in the body's first H1 heading. Neither is stored in frontmatter.

**Architecture invariant:** code that walks the issues tree never descends into a directory-based issue. Files inside it are attachments, even when their names look like issue files.

**Architecture invariant:** the commands `done`, `pending`, and `in-progress` look issues up with `find_active_issue`, so they never act on a done issue. `reopen` must find done issues, so it uses `find_issue` with done issues included. Field edits (`update`, `edit`) use `find_editable_issue`, which accepts both active and done issues.

**Architecture invariant:** renga never builds a directory layout that its own `create` would reject. `canonical_status_dir` enforces this: when an area cannot serve as a directory name, the issue is placed without the area level.

### `src/readme.rs`

Generates `issues/README.md` from the parsed issues.

**Architecture invariant:** `issues/README.md` is output only; renga never reads it back. Every command that writes issue files regenerates it.

### `src/commands/`

One file per subcommand, each with a `run` function that `lib.rs` calls.

### `tests/integration.rs`

End-to-end tests that run the compiled binary. See Testability below.

### Boundaries

Renga deliberately has no network access, no locking, no git integration, and no re-serialization of frontmatter.

- **No network.** All state is in the working tree. Sharing issues is the job of git or whatever syncs the files.
- **No locking.** Two renga processes writing to the same issues directory at the same time can race. For example, two `create` calls can pick the same next ID. Renga assumes one writer at a time.
- **No git integration.** Renga moves and edits files but never stages or commits them. The user decides when and how to commit.
- **No re-serialization of frontmatter.** Renga never rebuilds frontmatter from parsed data; it edits the syntax tree in place. A key that renga rewrites may change style (a block list of labels becomes `[a, b]`), but nothing else in the frontmatter is reformatted.

## Cross-cutting concerns

Four concerns apply across modules: testability, error handling, backward compatibility, and configuration.

### Testability

**Tests use the real filesystem and the real binary; there are no mocks.**

- **What runs for real.** `tests/integration.rs` runs the compiled `renga` binary with `assert_cmd` against a fresh temporary directory per test. Each command gets its working directory explicitly, so tests do not depend on the test runner's current directory.
- **How the code is split.** Functions in `src/issue.rs` and `src/project.rs` have unit tests next to them; pure string functions such as frontmatter edits and slug generation are tested on strings, and the rest on temporary directories. Command handlers have no unit tests and are covered by the integration tests.
- **How rare states are produced.** Tests write the files they need with `fs::write` before running renga: broken frontmatter, a file in the wrong status directory, two files with the same ID. They do not try to reach these states through renga commands.

### Error handling

Errors are `anyhow` errors. Handlers return them to `main`, which prints the whole chain of context as `error: …` and exits with code 1. When adding a fallible step, attach context (usually the file path) with `with_context`.

The exception is commands that take several IDs, such as `done`, `pending`, `in-progress`, and `reopen`. They print each failure in the handler, finish the remaining IDs, and exit with code 1 at the end, so their messages do not reach `main`.

### Backward compatibility

Renga keeps reading every format it has ever written. When a new format replaces an old one, reading code accepts both. Moving all old files to the current format at once is the job of `renga migrate`. Commands that rewrite a single issue, such as `update`, may also move that one issue to its current location. The formats that are still accepted are listed in [spec.md](spec.md).

### Configuration

`.renga.yml` is read once at startup into `Config` and reaches the handlers through `Context`. The only other reader is `project.rs`, which takes just `issues_dir` from it to locate the issues directory. The keys and their meanings are listed in [spec.md](spec.md).
