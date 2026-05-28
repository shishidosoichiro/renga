---
argument-hint: "[create] <title> | done <NNNNN> | pending <NNNNN> | reopen <NNNNN> | list | show <NNNNN> | help"
---

# /fbim skill

A Claude Code skill for File-Based Issue Management (FBIM).
All operations are performed by calling the `fbim` binary.

## Usage

```
/fbim [create] <title>     Create an issue
/fbim done <NNNNN>         Close an issue
/fbim pending <NNNNN>      Put an issue on hold
/fbim reopen <NNNNN>       Reopen a closed issue
/fbim list                 List open and pending issues
/fbim show <NNNNN>         Show issue details
/fbim help [command]       Show help
```

`create` is optional. `/fbim <title>` creates an issue directly.

---

## Agent workflow conventions

These rules apply whenever an AI agent works with FBIM issues.

**Before starting work**
- Run `fbim list` to confirm which issues are open or pending.
- Run `fbim show <N>` on the relevant issue before beginning work.

**During work**
- When you make a decision or discover a constraint that is not obvious from the issue title, append it to the issue body under a `## Notes` section using `--body` on create or by editing the file directly.
- Do not delete issue files. Use `fbim pending` or `fbim done` instead.

**When blocked**
- If you cannot proceed, run `fbim pending <N>` and append the reason under `## Notes` in the issue body.
- Do not leave the issue as `open` when you are unable to complete it.

**After completing work**
- Run `fbim validate` before closing any issue. Exit code 1 means errors exist; fix them first.
- Close the issue with `fbim done <N>`. Do not change `status` to `done` by editing the file directly — always use the command.
- Never skip `fbim done` and leave the issue as `open` after work is complete.

---

## Rules (all commands)

- If `$ARGUMENTS` is empty, run `fbim help` and display the output.
- Pass script error output to the user as-is.

---

## create

If `$ARGUMENTS` starts with `create`, strip it and use the rest as the title. Otherwise use `$ARGUMENTS` as-is.

```
fbim create "<title>" --slug <slug> --area <area>
```

- `--slug`: Kebab-case English slug derived from the title (max 30 chars). Auto-generated from title if omitted.
- `--area`: Infer from context. Use `misc` if unclear.
- `--body`: Include if there is additional context to add.
- Report the created file path to the user.

---

## done

```
fbim done <NNNNN>
```

Report the destination file path to the user.

---

## pending

```
fbim pending <NNNNN>
```

Report the updated file path to the user.

---

## reopen

```
fbim reopen <NNNNN>
```

Report the destination file path to the user.

---

## list

```
fbim list
```

With filters:

```
fbim list --area <area>
```

Display the output to the user.

---

## show

```
fbim show <NNNNN>
```

Display the output to the user.

---

## validate

```
fbim validate
```

Run after making bulk changes to issue files. Display the output to the user.
Exit code 1 means errors were found (unparseable frontmatter, invalid status,
duplicate IDs). Warnings (missing schema_version) exit with code 0.

---

## help

```
fbim help [command]
```

Display the output as-is.
