---
description: Update fields of a renga issue
argument-hint: "<ID> [<title>] [--priority high|medium|low] [--area <area>] [--status open|pending|in-progress] [--milestone <milestone>] [--label <label>]... [--add-label <label>]... [--remove-label <label>]... [--body <text|->] | <ID> --json"
---

# /renga-update

Update issue fields without opening an editor. Designed for AI agents and scripts.

```
renga update <ID> [<title>] [--priority high|medium|low] [--area <area>] [--status open|pending|in-progress] [--milestone <milestone>] [--label <label>]... [--add-label <label>]... [--remove-label <label>]... [--body <text|->]
renga update <ID> --json
```

- `<title>`: Optional positional argument. Updates the `# Heading` line in the body.
- `--body`: Replaces the body. If the new body has no `# Heading`, the existing title is automatically preserved.
- `--body -`: Reads the new body from stdin.
- `--label`: Repeatable and replaces all existing labels.
- `--add-label`: Adds a label without removing others. Repeatable and deduplicates automatically.
- `--remove-label`: Removes a specific label. Repeatable.
- `--json`: Reads one JSON object from stdin. Supported fields are `title`, `priority`, `area`, `status`, `milestone`, `labels`, `add_labels`, `remove_labels`, and `body`. Do not combine with field arguments.

Report the updated file path to the user.
