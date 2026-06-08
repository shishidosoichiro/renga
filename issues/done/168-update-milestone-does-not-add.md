---
schema_version: 1
status: done
priority: medium
area: cli
labels: [found_at:0.11.0, fixed_at:0.11.0]
---

# update --milestone does not add missing frontmatter field

The update command uses set_frontmatter_field for --milestone. That helper only updates an existing frontmatter line, so issues created without a milestone do not get a milestone when running update --milestone. This was observed while implementing issue 92 and is pre-existing behavior, not introduced by the JSON input feature.
