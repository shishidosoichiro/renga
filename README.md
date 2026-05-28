# FBIM — File-Based Issue Management

[![CI](https://github.com/shishidosoichiro/fbim/actions/workflows/ci.yml/badge.svg)](https://github.com/shishidosoichiro/fbim/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

A CLI tool for managing issues as files. `fbim create "title"` creates a Markdown file, and `fbim done 1` closes it.

> Japanese version: [README.ja.md](README.ja.md)

## Quick start

```sh
# 1. Install
bash <(curl -fsSL https://raw.githubusercontent.com/shishidosoichiro/fbim/main/install.sh)

# 2. Initialize (anywhere — no git repo required)
fbim init

# 3. Create your first issue
fbim create "My first task"
```

That's it. Issue files appear in `issues/`.

## Claude Code integration

Claude can create issues as it works, close them when done, and keep the list current — all without interrupting the coding flow.

Install the skill to manage issues with `/fbim` inside Claude Code:

```sh
ln -sf /path/to/fbim/skills/fbim ~/.claude/skills/fbim
```

Then use it directly in any Claude Code session:

```
/fbim create "Add input validation"
/fbim list
/fbim done 3
```

| Command | Description |
|---|---|
| `/fbim [create] <title>` | Create an issue |
| `/fbim done <N>` | Mark as done |
| `/fbim pending <N>` | Put on hold |
| `/fbim reopen <N>` | Reopen |
| `/fbim list` | List open and pending issues |
| `/fbim show <N>` | Show details |

## Who is it for?

FBIM is for **solo developers and small teams** who want to start tracking work immediately, without setting up an external service first.

- Starting a new project and don't want to configure GitHub Issues yet
- Working offline or on a private machine with no internet access
- Using an AI coding tool (like Claude Code) and want issue management without leaving the terminal
- Want issues to live in the same git history as the code that fixes them

If you need comments, assignments, notifications, or a web UI for non-engineers, reach for GitHub Issues or Linear instead. FBIM is intentionally minimal.

## How it's different

- **AI-native**: Issue files are plain Markdown, so LLMs can read and write them directly. An agent can open an issue, make a fix, and close it — all in one session.
- **Works offline**: No network connection, no account, no API token. `fbim init` is all the setup there is.
- **No config required**: Drop it into any directory. No project configuration or external service needed to get started.
- **Lives with your code**: Issue files are just files — open them in any editor, search with grep, and commit them to git alongside the code that fixes them.
- **Your data, your format**: Nothing to export. The files are readable plain text whether you keep using fbim or not.

## Installation

Download and run the install script. It fetches the pre-built binary for your platform from the package registry.

```sh
bash <(curl -fsSL https://raw.githubusercontent.com/shishidosoichiro/fbim/main/install.sh)
```

Or build from source:

```sh
cargo install --path /path/to/fbim
```

## Commands

| Command | Description |
|---|---|
| `fbim init` | Initialize the issues directory |
| `fbim create <title> [--id <N>] [--slug <slug>] [--priority high\|medium\|low] [--area <area>] [--body <text\|-\>] [--milestone <milestone>]` | Create an issue (`--body -` reads from stdin) |
| `fbim done <N>` | Mark an issue as done |
| `fbim pending <N>` | Put an issue on hold |
| `fbim reopen <N>` | Reopen a closed issue |
| `fbim list [--status open\|pending\|done\|unknown] [--area <area>] [--label <label>] [--milestone <milestone>] [--json]` | List issues |
| `fbim show <N>` | Show issue details |
| `fbim validate` | Check all issues for schema errors and duplicate IDs |
| `fbim completions bash\|zsh\|fish` | Print shell completion script |
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
