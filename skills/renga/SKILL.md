---
name: renga
description: File-Based Issue Management — track issues as plain Markdown files in your repo
argument-hint: "[create] <title> | done <ID> | pending <ID> | reopen <ID> | list | show <ID> | help"
---

# /renga skill

A Claude Code skill for File-Based Issue Management .
All operations are performed by calling the `renga` binary.

## Usage

```
/renga [create] <title>     Create an issue
/renga done <ID>            Close an issue
/renga pending <ID>         Put an issue on hold
/renga in-progress <ID>     Mark an issue as in-progress
/renga reopen <ID>          Reopen an issue
/renga list                 List open, pending, and in-progress issues
/renga show <ID>         Show issue details
/renga help [command]       Show help
```

`create` is optional. `/renga <title>` creates an issue directly.

---

## Agent workflow conventions

These rules apply whenever an AI agent works with renga issues.

**Before starting work**
- Run `renga list` to confirm which issues are open or pending.
- Run `renga show <ID>` on the relevant issue before beginning work.

**During work**
- When you make a decision or discover a constraint that is not obvious from the issue title, append it to the issue body under a `## Notes` section using `--body` on create or by editing the file directly.
- Do not delete issue files. Use `renga pending` or `renga done` instead.

**When blocked**
- If you cannot proceed, run `renga pending <ID>` and append the reason under `## Notes` in the issue body.
- Do not leave the issue as `open` when you are unable to complete it.

**After completing work**
- Run `renga validate` before closing any issue. Exit code 1 means errors exist; fix them first.
- Close the issue with `renga done <ID>`. Do not change `status` to `done` by editing the file directly — always use the command.
- Never skip `renga done` and leave the issue as `open` after work is complete.

---

## Rules (all commands)

- If `$ARGUMENTS` is empty, run `renga help` and display the output.
- Pass script error output to the user as-is.

---

## create

If `$ARGUMENTS` starts with `create`, strip it and use the rest as the title. Otherwise use `$ARGUMENTS` as-is.

```
renga create "<title>" --slug <slug> --area <area> --body "<description>"
```

- `--slug`: Kebab-case English slug derived from the title (max 30 chars). Auto-generated from title if omitted.
- `--area`: Infer from context. Use `misc` if unclear.
- `--body`: **Always include.** Write a brief description of what needs to be done and why. The title alone is not sufficient.
- Report the created file path to the user.

---

## done

Moves the file to `issues/done/`.

```
renga done <ID>
```

Report the destination file path to the user.

---

## pending

Moves the file to `issues/pending/`.

```
renga pending <ID>
```

Report the destination file path to the user.

---

## in-progress

Moves the file to `issues/in-progress/`.

```
renga in-progress <ID>
```

Report the destination file path to the user.

---

## reopen

Moves the file to `issues/open/`. Works from any status directory.

```
renga reopen <ID>
```

Report the destination file path to the user.

---

## list

Use `--json` when processing the result programmatically. Use plain output only when displaying directly to the user with no further processing.

```
renga list --json
```

With filters:

```
renga list --json --area <area>
```

---

## show

Use `--json` when accessing structured fields programmatically. Use plain output only when displaying directly to the user.

```
renga show <ID> --json
```

---

## edit

```
renga edit <ID>
```

Opens the issue file in `$EDITOR`. For human use only — AI agents cannot interact with interactive editors.

---

## update

```
renga update <ID> [<title>] [--priority high|medium|low] [--area <area>] [--status open|pending|in-progress] [--milestone <milestone>] [--label <label>]... [--add-label <label>]... [--remove-label <label>]... [--body <text|->]
```

Updates issue fields without opening an editor. Designed for AI agents and scripts.

- `<title>`: Optional positional argument. Updates the `# Heading` line in the body.
- `--body`: Replaces the body. If the new body has no `# Heading`, the existing title is automatically preserved.
- `--body -` reads the new body from stdin, allowing the agent to pipe modified content
- `--label` is repeatable and **replaces** all existing labels
- `--add-label` adds a label without removing others (repeatable, deduplicates automatically)
- `--remove-label` removes a specific label (repeatable)
- Multiple flags can be combined in one call

---

## migrate

Migrate issues from flat layout (`issues/N-name.md`) to per-status directories (`issues/open/`, `issues/pending/`, etc.). Run once on existing repos.

```
renga migrate
```

---

## validate

```
renga validate
```

Run after making bulk changes to issue files. Display the output to the user.
Exit code 1 means errors were found (unparseable frontmatter, invalid status,
duplicate IDs). Warnings (missing schema_version) exit with code 0.

---

## help

```
renga help [command]
```

Display the output as-is.
