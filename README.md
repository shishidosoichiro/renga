# FBIM — File-Based Issue Management

A system for managing issues as plain files. Track issues entirely within your Git repository, without relying on external services like GitHub Issues or Redmine.

> Japanese version: [README.ja.md](README.ja.md)

## Why file-based?

- **Git-native**: Full history, authorship, and comments live in `git log`. No external service dependency.
- **Co-located with code**: Include issue files in the same PR as the code change they relate to. The connection between fix and issue is explicit.
- **Works offline**: Create, update, and browse issues without a network connection.
- **Easy to migrate**: All data is plain files. Moving to Redmine or GitHub Issues later means reading those files — no lock-in.
- **Tool-agnostic**: Use an editor, a shell script, or an AI tool. The interface is just files.

## Requirements

- Python 3.8+ (`bin/fbim`, `bin/gen-issues-readme`)
- bash (`bin/next-id`)
- PyYAML — only if using `.fbim.yml` for customization (`pip install pyyaml`)

## Actions

### Create an issue

Create `issues/NNNN-short-name.md`. `NNNN` is a zero-padded sequential number. Include `status`, `priority`, and `area` in the frontmatter.

```
issues/0042-api-auth-missing-scope.md
```

See [spec.md](spec.md) for naming conventions and the file template.

### Update an issue

Edit the file directly. Both the body and frontmatter can be changed freely.

### Close an issue

Move `issues/NNNN-*.md` to `issues/done/NNNN-*.md` and set `status` to `done`.

### Put an issue on hold

Set `status` to `pending`. Use this for issues that are blocked or deferred.

### Reopen an issue

Move `issues/done/NNNN-*.md` back to `issues/NNNN-*.md` and set `status` to `open`.

### Browse the issue list

`issues/README.md` contains a generated list of open and pending issues. Do not edit it by hand — run `bin/gen-issues-readme` to regenerate it.

## Tools

### bin/ CLI

Add `bin/` to your PATH or call scripts by full path.

```sh
export PATH="$PATH:/path/to/fbim/bin"
```

| Command | Description |
|---|---|
| `fbim create <title>` | Create an issue |
| `fbim done <NNNN>` | Close an issue |
| `fbim pending <NNNN>` | Put an issue on hold |
| `fbim reopen <NNNN>` | Reopen a closed issue |
| `fbim list [--json]` | List issues (use `--json` for structured output) |
| `fbim show <NNNN>` | Show issue details |
| `fbim help [command]` | Show help |

`fbim list --json` outputs JSON that can be piped to `jq` or `yq`.

```sh
fbim list --json | jq '.[] | select(.area == "auth")'
```

### Claude Code skill

A Claude Code skill is included in `skills/fbim/`. Install it with a symlink.

```sh
ln -s /path/to/fbim/skills/fbim ~/.claude/skills/fbim
```

Once installed, the following commands are available in Claude Code sessions.

| Command | Description |
|---|---|
| `/fbim <title>` | Create an issue |
| `/fbim done NNNN` | Close an issue |
| `/fbim pending NNNN` | Put an issue on hold |
| `/fbim reopen NNNN` | Reopen a closed issue |
| `/fbim help` | Show help |

## Shell completion

Completion scripts for bash and zsh are in `completions/`.

**zsh**

```zsh
# Add to ~/.zshrc
fpath=(/path/to/fbim/completions $fpath)
autoload -Uz compinit && compinit
```

**bash**

```bash
# Add to ~/.bashrc
source /path/to/fbim/completions/fbim.bash
```

Subcommands, options, and issue numbers (read from `issues/` in the current directory) are all completed.

## Customization

Place `.fbim.yml` in the project root to customize how areas are displayed in the issue list.

```yaml
area_order:
  - backend
  - frontend
  - infra
  - misc

area_labels:
  backend: "Backend"
  frontend: "Frontend"
  infra: "Infrastructure"
  misc: "Other"
```

Without `.fbim.yml`, areas are displayed as-is and sorted alphabetically.
