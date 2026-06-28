---
description: Create a new renga issue
argument-hint: "<title> [--slug <slug>] [--priority high|medium|low] [--area <area>] [--body <text>] [--milestone <milestone>] [--assignee <assignee>] [--dir=true|false] | --json"
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
- `--assignee`: Optional. Name of the person or agent responsible for the issue.
- `--dir`: Pass `--dir=true` to create the issue as a directory (`N-slug/README.md`) so attachments or notes can live alongside it. Defaults to the flat `N-slug.md` file.
- `--json`: Read one JSON object from stdin. Supported fields are `title`, `id`, `slug`, `priority`, `area`, `body`, `milestone`, `assignee`, and `labels`. Do not combine with field arguments.

Report the created file path to the user.
