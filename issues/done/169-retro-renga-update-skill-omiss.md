---
schema_version: 1
status: done
priority: medium
area: agent
labels: [retro]
---

# retro: renga-update skill omission

renga create/update JSON input support updated skills/renga and skills/renga-create, but the absence of a dedicated renga-update skill leaves the skill distribution inconsistent. The user pointed this out after implementation. Improve the workflow so paired CLI commands are checked for matching dedicated skills when updating distributed skills.
