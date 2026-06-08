---
schema_version: 1
status: open
priority: medium
area: test
labels: [found_at:0.2.0]
---

# project tests race on process current_dir

project::tests changes the process-wide current directory in multiple tests without synchronization. A normal parallel cargo test run can race and fail with os error 2 while restoring or reading the current directory. Reproduce observed during README link fix validation; the same test passes when run alone with --test-threads=1. Fix separately from issue 171 by serializing cwd-changing tests or avoiding process-wide cwd mutation.
