# Changelog

All notable changes to this project will be documented in this file.
## [0.17.0] - 2026-07-27

### 🚀 Features

- [`6fe52d7`] *(core)* Nest issues under an area directory via group_by
- [`edf7865`] *(core)* Add defaults.dir config and apply it in migrate

### 🐛 Bug Fixes

- [`9ac3d7d`] Stop offering malformed filenames as completion candidates
- [`b95c8a4`] Keep attachments inside a directory-based issue from being read as issues
- [`052da22`] Offer directory-based issues as shell completion candidates
- [`33f6113`] Stop offering open issues as `renga reopen` completion candidates
- [`1eff237`] Stop a custom issues_dir name from reserving an issue ID

### 🚜 Refactor

- [`563e482`] *(core)* Consolidate issue relocation into a shared helper
## [0.16.0] - 2026-07-02

### 🚀 Features

- [`8df5b9f`] Allow update/edit to modify done issues without reopening

### 🐛 Bug Fixes

- [`3a67de1`] Truncate slugs by byte length instead of character count
## [0.15.0] - 2026-06-29

### 🚀 Features

- [`3450c3e`] Support directory layout for issues (--dir=true|false)

### 🐛 Bug Fixes

- [`876fa39`] Preserve Unicode characters in auto-generated slugs
## [0.14.0] - 2026-06-22

### 🚀 Features

- [`1be2289`] *(cli)* Validate selected issues and auto-correct status dirs

### 🐛 Bug Fixes

- [`84b37e9`] *(cli)* Operate on misplaced active issues
- [`2abf200`] *(cli)* Require explicit active status for misplaced issues
## [0.13.0] - 2026-06-12

### Features

- *(core)* Add `assignee` field to issue front matter
- *(core)* Clear `milestone` and `assignee` by passing an empty string

### Bug Fixes

- *(docs)* Add missing `--milestone` to skill argument-hints

## [0.12.0] - 2026-06-10

### Features

- *(cli)* Accept JSON input for `create` and `update` commands

### Bug Fixes

- *(cli)* Add missing `milestone` field on `update`
- *(core)* Generate README links relative to the issues directory

## [0.11.0] - 2026-06-06

### Features

- *(cli)* Accept multiple IDs for `done`, `pending`, `in-progress`, and `reopen`

### Bug Fixes

- *(cli)* Add descriptions to `--status` values and shell name candidates in completions

## [0.10.0] - 2026-06-05

### Features

- *(cli)* Add descriptions to completion candidates for `--priority` and `--status`

## [0.9.1] - 2026-06-05

### Bug Fixes

- *(cli)* Add `--add-label`, `--remove-label` to `update` completion and `--milestone` to `list` completion

## [0.9.0] - 2026-06-05

### Features

- *(cli)* `renga update`: add `--title`, `--add-label`/`--remove-label` flags; protect body from accidental truncation
- *(core)* **Breaking:** Reorganize issues into per-status subdirectories (`issues/open/`, `issues/done/`, etc.)

  **Migration:** Run `renga migrate` to move existing issue files into the new layout.

## [0.8.0] - 2026-06-03

### Features

- *(core)* Add `in-progress` status
- *(cli)* Add `renga info` command

## [0.7.0] - 2026-05-31

### Features

- *(cli)* Add `--label` to `create` and `--json` to `show`

## [0.6.0] - 2026-05-29

### Features

- *(cli)* Add `renga edit` and `renga update` commands

## [0.5.1] - 2026-05-29

### Features

- Publish to crates.io — install with `cargo install renga`

## [0.5.0] - 2026-05-29

### Features

- **Breaking:** Rename tool from `fbim` to `renga`; update all command names accordingly

  **Migration:** Replace `fbim` with `renga` in scripts and shell configurations. The `.fbim.yml` config file is now `.renga.yml`.

- *(cli)* Add `--body -` to read issue body from stdin
- *(cli)* Add `--id` to specify an issue number explicitly on `create`
- *(core)* Add `schema_version` field to issue front matter
- *(cli)* Add `validate` command to check issue file integrity
- Support installation via `npx skills add`

### Bug Fixes

- *(core)* Fix `set_frontmatter_field` incorrectly re-parsing front matter when body contained `---`
- *(core)* Improve file operation safety for `reopen`, `done`, and `create`
- *(config)* Propagate `.renga.yml` YAML parse errors instead of silently ignoring them
- *(core,cli)* Remove trailing dash from slugs; default `--area` to empty string
- *(cli)* Reject invalid `--status` values with an error; fix completions and help text
- *(core)* Fall back to filename stem when issue title cannot be extracted
- *(core)* Validate that `--id 0` and non-numeric IDs are rejected
- *(core)* Fix `reopen` overwriting an already-open issue

## [0.4.0] - 2026-05-26

### Features

- *(core)* Add `milestone` field to issue front matter

## [0.3.0] - 2026-05-26

### Features

- *(core)* Assign `unknown` status to files without front matter
- *(core)* Search `issues/` recursively; add `Priority::Unknown` variant

### Bug Fixes

- *(core)* Skip unparseable issue files with a warning instead of aborting
- *(core)* Show Markdown files without front matter using default field values

## [0.2.1] - 2026-05-26

### Bug Fixes

- *(cli)* Show `--version` and `--help` in tab completion for `renga --`
- *(cli)* Fix issue list sort order to be numeric

## [0.2.0] - 2026-05-25

### Features

- *(core)* Use plain integer IDs instead of zero-padded IDs (e.g. `42` instead of `00042`)

## [0.1.0] - 2026-05-25

Initial release. Rust rewrite of the original Python/bash implementation.

### Features

- `renga init` — initialize a new issues directory
- Shell completions for bash and zsh (including issue title display)
- Individual skills: `renga-create`, `renga-done`, `renga-pending`, `renga-reopen`, `renga-list`, `renga-show`
- Parent directory search for issues directory; `issues_dir` config support
- Binary distribution via package registry

### Bug Fixes

- Fix broken-pipe panic in `completions`
- Fix CI coverage collection on Kubernetes
