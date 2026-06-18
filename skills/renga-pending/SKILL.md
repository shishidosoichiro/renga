---
description: Put a renga issue on hold
argument-hint: "<ID>..."
---

# /renga-pending

Move an issue to `issues/pending/` and set `status: pending` (blocked or deferred).

```
renga pending <ID>...
```

If the issue is stored under `done/` but its frontmatter status is active, the
command still moves it to `issues/pending/` and prints a warning recommending
`renga validate <ID> --auto-correct`.

Report the updated file path to the user.
