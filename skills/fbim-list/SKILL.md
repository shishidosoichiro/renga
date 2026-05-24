---
description: List FBIM issues
argument-hint: "[--status open|pending|done] [--area <area>] [--label <label>] [--json]"
---

# /fbim-list

List issues. Defaults to open and pending.

```
${CLAUDE_SKILL_DIR}/../../bin/fbim list
```

With filters:

```
${CLAUDE_SKILL_DIR}/../../bin/fbim list --status open --area <area>
```

Display the output to the user. Use `--json` if structured output is needed for further processing.
