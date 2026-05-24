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

## help

```
fbim help [command]
```

Display the output as-is.
