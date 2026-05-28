# Renga — File-Based Issue Management

[![CI](https://github.com/shishidosoichiro/renga/actions/workflows/ci.yml/badge.svg)](https://github.com/shishidosoichiro/renga/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

renga is a CLI tool for managing issues as files. Plain Markdown, so your editor, grep, git, and AI agents can all read and write them. `renga create "title"` creates an issue; `renga done 1` closes it.

> Japanese version: [README.ja.md](README.ja.md)

## Quick start

```sh
# 1. Install
bash <(curl -fsSL https://raw.githubusercontent.com/shishidosoichiro/renga/main/install.sh)

# 2. Initialize (anywhere — no git repo required)
renga init

# 3. Create your first issue
renga create "My first task"
```

That's it. Issue files appear in `issues/`.

## Recommended workflow with AI agents

```text
1. Create an issue for the task
   renga create "Add input validation" --area core

2. Agent runs renga list before starting work

3. Agent appends decisions and constraints to the issue body under ## Notes

4. After implementation, run renga validate

5. Close the issue alongside the fix commit
   renga done 1
   git add issues/ src/
   git commit -m "feat: add input validation (#1)"
```

Issue files and code changes land in the same commit — the git history tells the full story.

## Claude Code integration

Claude can create issues as it works, close them when done, and keep the list current — all without interrupting the coding flow.

Install the skill to manage issues with `/renga` inside Claude Code:

```sh
# With Node.js
npx skills add shishidosoichiro/renga

# Without Node.js
mkdir -p ~/.claude/skills/renga
curl -fsSL https://raw.githubusercontent.com/shishidosoichiro/renga/main/skills/renga/SKILL.md \
  -o ~/.claude/skills/renga/SKILL.md
```

Then use it directly in any Claude Code session:

```
/renga create "Add input validation"
/renga list
/renga done 3
```

| Command | Description |
|---|---|
| `/renga [create] <title>` | Create an issue |
| `/renga done <N>` | Mark as done |
| `/renga pending <N>` | Put on hold |
| `/renga reopen <N>` | Reopen |
| `/renga list` | List open and pending issues |
| `/renga show <N>` | Show details |

## Before you reach for GitHub Issues

Renga is for **solo developers and small teams** who want to start tracking work immediately, without setting up an external service first.

- Using an AI coding tool (like Claude Code) and want issue management without leaving the terminal
- Starting a project before setting up GitHub Issues
- Working offline or on a private machine with no internet access
- Want issues to live in the same git history as the code that fixes them

If you need comments, assignments, notifications, or a web UI for non-engineers, reach for GitHub Issues or Linear instead. Renga is intentionally minimal.

## How it's different

- **AI-native**: Issue files are plain Markdown, so LLMs can read and write them directly. An agent can open an issue, make a fix, and close it — all in one session.
- **Works offline**: No network connection, no account, no API token. `renga init` is all the setup there is.
- **No config required**: Drop it into any directory. No project configuration or external service needed to get started.
- **Lives with your code**: Issue files are just files — open them in any editor, search with grep, and commit them to git alongside the code that fixes them.
- **Your data, your format**: Nothing to export. The files are readable plain text whether you keep using renga or not.

## Installation

Download and run the install script. It fetches the pre-built binary for your platform from GitHub Releases.

```sh
bash <(curl -fsSL https://raw.githubusercontent.com/shishidosoichiro/renga/main/install.sh)
```

Or build from source:

```sh
cargo install --path /path/to/renga
```

## Commands

| Command | Description |
|---|---|
| `renga init` | Initialize the issues directory |
| `renga create <title> [--id <N>] [--slug <slug>] [--priority high\|medium\|low] [--area <area>] [--body <text\|-\>] [--milestone <milestone>]` | Create an issue (`--body -` reads from stdin) |
| `renga done <N>` | Mark an issue as done |
| `renga pending <N>` | Put an issue on hold |
| `renga reopen <N>` | Reopen a closed issue |
| `renga list [--status open\|pending\|done\|unknown] [--area <area>] [--label <label>] [--milestone <milestone>] [--json]` | List issues |
| `renga show <N>` | Show issue details |
| `renga validate` | Check all issues for schema errors and duplicate IDs |
| `renga completions bash\|zsh\|fish` | Print shell completion script |
| `renga help [command]` | Show help |

```sh
# JSON output can be piped to jq
renga list --json | jq '.[] | select(.area == "auth")'
```

## How renga finds your issues

When you run any `renga` command, it walks up from the current directory toward the filesystem root, stopping at the first directory that matches either of these conditions:

1. A `.renga.yml` file is present — the `issues_dir` value in that file is used as the issues directory (default: `issues`)
2. An `issues/` subdirectory is present

This means you can run `renga` from any subdirectory of your project and it will find the right issues directory automatically. If nothing is found, `renga` falls back to `issues/` relative to the current directory.

## Shell completions

Enable tab completion for subcommands, flags, and issue IDs.

**bash** — add to `~/.bashrc`:

```sh
eval "$(renga completions bash)"
```

**zsh** — add to `~/.zshrc`:

```sh
source <(renga completions zsh)
```

**fish** — install once:

```sh
renga completions fish > ~/.config/fish/completions/renga.fish
```

## Customization

Place `.renga.yml` in the project root.

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
