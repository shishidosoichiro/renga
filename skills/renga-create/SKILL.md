---
description: Create a new renga issue
argument-hint: "<title> [--slug <slug>] [--priority high|medium|low] [--area <area>] [--body <text>]"
---

# /renga-create

Create a new issue in `issues/`.

```
renga create "<title>" --slug <slug> --area <area> --body "<description>"
```

- `--slug`: Kebab-case English slug derived from the title (max 30 chars).
- `--area`: Infer from context. Use `misc` if unclear.
- `--priority`: Default is `medium`. Use `high` for correctness issues, `low` for suggestions.
- `--body`: **Always include.** Write a brief description of what needs to be done and why. The title alone is not sufficient.

Report the created file path to the user.
