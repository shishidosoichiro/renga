---
description: List renga issues
argument-hint: "[--status open|pending|in-progress|done] [--area <area>] [--label <label>] [--milestone <milestone>] [--assignee <assignee>] [--json]"
---

# /renga-list

List issues. Defaults to open, pending, and in-progress.

Use `--json` when processing the result programmatically. Use plain output only when displaying directly to the user with no further processing.

```
renga list --json
```

With filters:

```
renga list --json --status open --area <area>
```
