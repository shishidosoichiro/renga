# FBIM — File-Based Issue Management

[![pipeline status](https://gitlab.home/kiwi/ifbm/badges/main/pipeline.svg)](https://gitlab.home/kiwi/ifbm/-/pipelines)
[![coverage](https://gitlab.home/kiwi/ifbm/badges/main/coverage.svg)](https://gitlab.home/kiwi/ifbm/-/pipelines)

A system for managing issues as plain files. Track issues entirely within your Git repository, without relying on external services like GitHub Issues or Redmine.

> Japanese version: [README.ja.md](README.ja.md)

## Why file-based?

- **Git-native**: Full history, authorship, and comments live in `git log`. No external service dependency.
- **Co-located with code**: Include issue files in the same PR as the code change they relate to. The connection between fix and issue is explicit.
- **Works offline**: Create, update, and browse issues without a network connection.
- **Easy to migrate**: All data is plain files. Moving to Redmine or GitHub Issues later means reading those files — no lock-in.
- **Tool-agnostic**: Use an editor, a shell script, or an AI tool. The interface is just files.

## Installation

Download and run the install script. It fetches the pre-built binary for your platform from the package registry.

```sh
bash <(curl -fsSL https://gitlab.home/kiwi/ifbm/-/raw/main/install.sh)
```

Or clone the repo and build from source:

```sh
cargo install --path /path/to/fbim
```

## Getting started

1. In your project root, create the issues directory.

   ```sh
   mkdir -p issues/done
   ```

2. Create your first issue.

   ```sh
   fbim create "My first issue" --area docs
   ```

That's it. Issue files live alongside your code in Git.

## Commands

| Command | Description |
|---|---|
| `fbim create <title>` | Create an issue |
| `fbim done <NNNNN>` | Mark an issue as done |
| `fbim pending <NNNNN>` | Put an issue on hold |
| `fbim reopen <NNNNN>` | Reopen a closed issue |
| `fbim list [--status open\|pending\|done] [--area <area>] [--json]` | List issues |
| `fbim show <NNNNN>` | Show issue details |
| `fbim help [command]` | Show help |

```sh
# JSON output can be piped to jq
fbim list --json | jq '.[] | select(.area == "auth")'
```

## Claude Code skill

```sh
ln -sf /path/to/fbim/skills/fbim ~/.claude/skills/fbim
```

| Command | Description |
|---|---|
| `/fbim [create] <title>` | Create an issue |
| `/fbim done <NNNNN>` | Mark as done |
| `/fbim pending <NNNNN>` | Put on hold |
| `/fbim reopen <NNNNN>` | Reopen |
| `/fbim list` | List open and pending issues |
| `/fbim show <NNNNN>` | Show details |

## Customization

Place `.fbim.yml` in the project root.

```yaml
issues_dir: issues    # default: issues

area_order:           # display order in the list (alphabetical if omitted)
  - backend
  - frontend
  - infra
  - misc

area_labels:          # display names for areas
  backend: "Backend"
  frontend: "Frontend"
  infra: "Infrastructure"
  misc: "Other"
```

## Development

```sh
cargo test            # run tests
cargo test --doc      # run doctests
cargo clippy -- -D warnings
cargo fmt --check
cargo doc --no-deps --open
```

[spec.md](spec.md) is the authoritative specification for file format and naming rules.
