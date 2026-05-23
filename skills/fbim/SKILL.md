---
argument-hint: "[create] <title> | done <NNNN> | pending <NNNN> | reopen <NNNN> | help [command]"
---

# /fbim skill

A Claude Code skill for File-Based Issue Management (FBIM).
All operations are performed by calling `${CLAUDE_SKILL_DIR}/scripts/fbim`.

## Usage

```
/fbim [create] <title>     Create an issue
/fbim done <NNNN>          Close an issue
/fbim pending <NNNN>       Put an issue on hold
/fbim reopen <NNNN>        Reopen a closed issue
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

- `--slug`: Kebab-case English slug derived from the title (max 30 chars).
- `--area`: Infer from context. Use `misc` if unclear.
- `--body`: Include if there is additional context to add.
- Report the created file path to the user.

---

## done

```
${CLAUDE_SKILL_DIR}/scripts/fbim done <NNNN>
```

Report the destination file path to the user.

---

## pending

```
${CLAUDE_SKILL_DIR}/scripts/fbim pending <NNNN>
```

Report the updated file path to the user.

---

## reopen

```
${CLAUDE_SKILL_DIR}/scripts/fbim reopen <NNNN>
```

Report the destination file path to the user.

---

## help

```
${CLAUDE_SKILL_DIR}/scripts/fbim help [command]
```

Display the output as-is.
