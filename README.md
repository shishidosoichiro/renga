# FBIM — File-Based Issue Management

[![pipeline status](https://gitlab.home/kiwi/ifbm/badges/main/pipeline.svg)](https://gitlab.home/kiwi/ifbm/-/pipelines)
[![coverage](https://gitlab.home/kiwi/ifbm/badges/main/coverage.svg)](https://gitlab.home/kiwi/ifbm/-/pipelines)

Issue tracking that lives in your repo. Start in seconds — no account, no configuration, no external service required.

> Japanese version: [README.ja.md](README.ja.md)

## Quick start

```sh
# 1. Install
bash <(curl -fsSL https://gitlab.home/kiwi/ifbm/-/raw/main/install.sh)

# 2. Initialize (anywhere — no git repo required)
fbim init

# 3. Create your first issue
fbim create "My first task"
```

That's it. Issue files appear in `issues/` alongside your code.

## Who is it for?

FBIM is for **solo developers and small teams** who want to start tracking work immediately, without setting up an external service first.

- Starting a new project and don't want to configure GitHub Issues yet
- Working offline or on a private machine with no internet access
- Using an AI coding tool (like Claude Code) and want issue management without leaving the terminal
- Want issues to live in the same git history as the code that fixes them

If you need comments, assignments, notifications, or a web UI for non-engineers, reach for GitHub Issues or Linear instead. FBIM is intentionally minimal.

## Why file-based?

- **Zero friction**: `fbim init` and you're done. No accounts, tokens, or config files required.
- **Git-native**: History, authorship, and diffs live in `git log`. Close an issue in the same commit as the fix.
- **Works offline**: Create, update, and browse issues without a network connection.
- **AI-friendly**: Plain Markdown files are easy for LLMs to read, create, and update. Pair with Claude Code to manage issues without leaving your editor.
- **No lock-in**: All data is plain files. Migrate to another tracker later by reading those files — no export step needed.

## Installation

Download and run the install script. It fetches the pre-built binary for your platform from the package registry.

```sh
bash <(curl -fsSL https://gitlab.home/kiwi/ifbm/-/raw/main/install.sh)
```

Or build from source:

```sh
cargo install --path /path/to/fbim
```

## Commands

| Command | Description |
|---|---|
| `fbim init` | Initialize the issues directory |
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

## How fbim finds your issues

When you run any `fbim` command, it walks up from the current directory toward the filesystem root, stopping at the first directory that matches either of these conditions:

1. A `.fbim.yml` file is present — the `issues_dir` value in that file is used as the issues directory (default: `issues`)
2. An `issues/` subdirectory is present

This means you can run `fbim` from any subdirectory of your project and it will find the right issues directory automatically. If nothing is found, `fbim` falls back to `issues/` relative to the current directory.

## Shell completions

Enable tab completion for subcommands, flags, and issue IDs.

**bash** — add to `~/.bashrc`:

```sh
eval "$(fbim completions bash)"
```

**zsh** — add to `~/.zshrc`:

```sh
source <(fbim completions zsh)
```

**fish** — install once:

```sh
fbim completions fish > ~/.config/fish/completions/fbim.fish
```

## Claude Code skill

FBIM pairs especially well with Claude Code. Install the skill to create and manage issues with `/fbim` without leaving your coding session.

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

Claude can create issues as it works, close them when done, and keep the list current — all without interrupting the coding flow.

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
