# FBIM Specification

> Japanese version: [spec.ja.md](spec.ja.md)

---

## Directory structure

```
issues/
  README.md           Issue list (auto-generated — do not edit manually)
  NNNNN-name.md       Open or pending issues
  done/
    NNNNN-name.md     Closed issues
```

## File naming

```
NNNNN-short-name.md
```

- `NNNNN`: Zero-padded 5-digit sequential number. Get the next number with `bin/next-id issues/`.
- `short-name`: Kebab-case short description (ASCII alphanumeric and hyphens only).
- Before creating, check for duplicates. Do not create a new issue if an equivalent one already exists.

**Backward compatibility**: 4-digit IDs (`NNNN-*.md`) from earlier versions are supported. All tools read both formats; new issues are always created with 5-digit IDs.

## Frontmatter

```markdown
---
status: open
priority: high|medium|low
area: area-name (e.g. auth, api, docs)
labels: []
---
```

**status values**

| Value | Meaning |
|---|---|
| `open` | Needs action. Work target. |
| `pending` | Blocked or deferred. Do not work on it. |
| `done` | Closed (moved to `done/`). |

**priority values**

| Value | Meaning |
|---|---|
| `high` | Fix immediately. Correctness or consistency is broken. |
| `medium` | Needs discussion. Design decisions required. |
| `low` | Suggestion. Improvement opportunity, not urgent. |

## Issue file template

```markdown
---
status: open
priority: medium
area: area-name
labels: []
---

# Title

Brief description of what needs to be done.

## Background (optional)

Additional context and related information.

## Related (optional)

- Links to related issues, ADRs, or documentation
```

## Managing `issues/README.md`

- Do not edit manually. Run `bin/gen-issues-readme` to regenerate.
- Run after every create, close, or status change.

## Actions

### Create an issue

1. Get the next number: `bin/next-id issues/`
2. Generate a kebab-case `short-name` from the title.
3. Create `issues/NNNNN-short-name.md` using the template above.
4. Run `bin/gen-issues-readme`.

### Close an issue

1. Move `issues/NNNNN-*.md` to `issues/done/NNNNN-*.md`.
2. Set frontmatter `status` to `done`.
3. Run `bin/gen-issues-readme`.

### Put an issue on hold

1. Set frontmatter `status` to `pending`.
2. Run `bin/gen-issues-readme`.

### Reopen an issue

1. Move `issues/done/NNNNN-*.md` to `issues/NNNNN-*.md`.
2. Set frontmatter `status` to `open`.
3. Run `bin/gen-issues-readme`.

### Update an issue

Edit the file directly.

---

## `bin/next-id`

`bin/next-id <dir>` scans the given directory (and its `done/` subdirectory) for issue files, and prints the next number as a zero-padded 5-digit string. Returns `00001` if no files exist. 4-digit files from earlier versions are also detected.

## `bin/gen-issues-readme`

Scans `issues/` and `issues/done/`, reads `status`, `priority`, and `area` from each file's frontmatter, and regenerates `issues/README.md`. Open and pending issues appear first; done issues appear after.

Area display order and labels are read from `.fbim.yml` in the project root. If not present, areas are sorted alphabetically and displayed as-is.

## `.fbim.yml`

Optional configuration file placed in the project root. Requires PyYAML (`pip install pyyaml`).

```yaml
area_order:       # Display order of areas in the issue list (alphabetical if omitted)
  - backend
  - frontend
  - misc

area_labels:      # Display names for areas (area name used as-is if omitted)
  backend: "Backend"
  frontend: "Frontend"
  misc: "Other"
```
