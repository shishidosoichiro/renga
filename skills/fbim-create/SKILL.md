---
description: Create a new FBIM issue
argument-hint: "<title> [--slug <slug>] [--priority high|medium|low] [--area <area>] [--body <text>]"
---

# /fbim-create

Create a new issue in `issues/`.

```
fbim create "<title>" --slug <slug> --area <area>
```

- `--slug`: Kebab-case English slug derived from the title (max 30 chars).
- `--area`: Infer from context. Use `misc` if unclear.
- `--priority`: Default is `medium`. Use `high` for correctness issues, `low` for suggestions.
- `--body`: Include if there is additional context to add.

Report the created file path to the user.
