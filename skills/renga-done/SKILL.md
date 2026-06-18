---
description: Close a renga issue
argument-hint: "<ID>..."
---

# /renga-done

Close an issue by moving it to `issues/done/` and setting `status: done`.

```
renga done <ID>...
```

If the issue is stored under `done/` but its frontmatter status is active, the
command still sets `status: done` and prints a warning recommending
`renga validate <ID> --auto-correct`.

Report the destination file path to the user.
