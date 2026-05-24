---
argument-hint: "[create] <title> | done <NNNNN> | pending <NNNNN> | reopen <NNNNN> | list | show <NNNNN> | help"
---

# /fbim skill

A Claude Code skill for File-Based Issue Management (FBIM).
All operations are performed by calling `${CLAUDE_SKILL_DIR}/scripts/fbim`.

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

## Rules (all commands)

- If `$ARGUMENTS` is empty, run `${CLAUDE_SKILL_DIR}/scripts/fbim help` and display the output.
- Pass script error output to the user as-is.

---

## create

If `$ARGUMENTS` starts with `create`, strip it and use the rest as the title. Otherwise use `$ARGUMENTS` as-is.

```
${CLAUDE_SKILL_DIR}/scripts/fbim create "<title>" --slug <slug> --area <area>
```

- `--slug`: Kebab-case English slug derived from the title (max 30 chars). Auto-generated from title if omitted.
- `--area`: Infer from context. Use `misc` if unclear.
- `--body`: Include if there is additional context to add.
- Report the created file path to the user.

---

## done

```
${CLAUDE_SKILL_DIR}/scripts/fbim done <NNNNN>
```

Report the destination file path to the user.

---

## pending

```
${CLAUDE_SKILL_DIR}/scripts/fbim pending <NNNNN>
```

Report the updated file path to the user.

---

## reopen

```
${CLAUDE_SKILL_DIR}/scripts/fbim reopen <NNNNN>
```

Report the destination file path to the user.

---

## list

```
${CLAUDE_SKILL_DIR}/scripts/fbim list
```

With filters:

```
${CLAUDE_SKILL_DIR}/scripts/fbim list --area <area>
```

Display the output to the user.

---

## show

```
${CLAUDE_SKILL_DIR}/scripts/fbim show <NNNNN>
```

Display the output to the user.

---

## help

```
${CLAUDE_SKILL_DIR}/scripts/fbim help [command]
```

Display the output as-is.
