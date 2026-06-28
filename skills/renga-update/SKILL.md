---
description: Update fields of a renga issue
argument-hint: "<ID> [<title>] [--priority high|medium|low] [--area <area>] [--status open|pending|in-progress] [--milestone <milestone>] [--assignee <assignee>] [--label <label>]... [--add-label <label>]... [--remove-label <label>]... [--body <text|->] [--dir=true|false] | <ID> --json"
---

# /renga-update

Update issue fields without opening an editor. Designed for AI agents and scripts.

```
renga update <ID> [<title>] [--priority high|medium|low] [--area <area>] [--status open|pending|in-progress] [--milestone <milestone>] [--assignee <assignee>] [--label <label>]... [--add-label <label>]... [--remove-label <label>]... [--body <text|->]
renga update <ID> --dir=true|false
renga update <ID> --json
```

- `<title>`: Optional positional argument. Updates the `# Heading` line in the body.
- `--body`: Replaces the body. If the new body has no `# Heading`, the existing title is automatically preserved.
- `--body -`: Reads the new body from stdin.
- `--milestone`: Optional. Set the milestone. Pass `--milestone ''` to remove the field.
- `--assignee`: Optional. Set the person or agent responsible for the issue. Pass `--assignee ''` to remove the field.
- `--label`: Repeatable and replaces all existing labels.
- `--add-label`: Adds a label without removing others. Repeatable and deduplicates automatically.
- `--remove-label`: Removes a specific label. Repeatable.
- `--dir`: Convert the issue between layouts. `--dir=true` expands a flat `N-slug.md` file into a `N-slug/README.md` directory; `--dir=false` collapses it back (fails if the directory holds files other than `README.md`).
- `--json`: Reads one JSON object from stdin. Supported fields are `title`, `priority`, `area`, `status`, `milestone`, `assignee`, `labels`, `add_labels`, `remove_labels`, and `body`. Do not combine with field arguments.

If the issue is stored under `done/` but its frontmatter status is active, the
command still updates it and prints a warning recommending
`renga validate <ID> --auto-correct`. Normal `status: done` issues must be
reopened before update.

Report the updated file path to the user.
