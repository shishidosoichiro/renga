---
description: List FBIM issues
argument-hint: "[--status open|pending|done] [--area <area>] [--label <label>] [--json]"
---

# /fbim-list

List issues. Defaults to open and pending.

```
fbim list
```

With filters:

```
fbim list --status open --area <area>
```

Display the output to the user. Use `--json` if structured output is needed for further processing.
