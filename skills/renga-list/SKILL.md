---
description: List FBIM issues
argument-hint: "[--status open|pending|done] [--area <area>] [--label <label>] [--json]"
---

# /renga-list

List issues. Defaults to open and pending.

```
renga list
```

With filters:

```
renga list --status open --area <area>
```

Display the output to the user. Use `--json` if structured output is needed for further processing.
