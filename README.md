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

## Getting started

1. Clone or download this repository.

   ```sh
   git clone https://gitlab.home/kiwi/ifbm.git /path/to/fbim
   ```

2. Add `bin/` to your PATH.

   ```sh
   export PATH="$PATH:/path/to/fbim/bin"
   ```

3. In your project root, create the issues directory.

   ```sh
   mkdir -p issues/done
   ```

4. Create your first issue.

   ```sh
   fbim create "My first issue" --area docs
   ```

That's it. Issue files live alongside your code in Git.

## Actions

### Create an issue

Create `issues/NNNNN-short-name.md`. `NNNNN` is a zero-padded 5-digit sequential number. Include `status`, `priority`, and `area` in the frontmatter.

```
issues/00042-api-auth-missing-scope.md
```

See [spec.md](spec.md) for naming conventions and the file template.

### Update an issue

Edit the file directly. Both the body and frontmatter can be changed freely.

### Close an issue

Move `issues/NNNNN-*.md` to `issues/done/NNNNN-*.md` and set `status` to `done`.

### Put an issue on hold

Set `status` to `pending`. Use this for issues that are blocked or deferred.

### Reopen an issue

Move `issues/done/NNNNN-*.md` back to `issues/NNNNN-*.md` and set `status` to `open`.

### Browse the issue list

`issues/README.md` contains a generated list of open and pending issues. Do not edit it by hand — run `bin/gen-issues-readme` to regenerate it.

## Tools

### bin/ CLI

| Command | Description |
|---|---|
| `fbim create <title>` | Create an issue (`--slug` is auto-generated from title if omitted) |
| `fbim done <NNNNN>` | Close an issue |
| `fbim pending <NNNNN>` | Put an issue on hold |
| `fbim reopen <NNNNN>` | Reopen a closed issue |
| `fbim list [--json]` | List issues (use `--json` for structured output) |
| `fbim show <NNNNN>` | Show issue details |
| `fbim help [command]` | Show help |

`fbim list --json` outputs JSON that can be piped to `jq` or `yq`.

```sh
fbim list --json | jq '.[] | select(.area == "auth")'
```

### Claude Code skills

Skills are included in `skills/`. Install with symlinks.

```sh
# Single entry point (all subcommands)
ln -s /path/to/fbim/skills/fbim ~/.claude/skills/fbim

# Individual skills (discoverable via /fbim- prefix)
ln -s /path/to/fbim/skills/fbim-create  ~/.claude/skills/fbim-create
ln -s /path/to/fbim/skills/fbim-done    ~/.claude/skills/fbim-done
ln -s /path/to/fbim/skills/fbim-pending ~/.claude/skills/fbim-pending
ln -s /path/to/fbim/skills/fbim-reopen  ~/.claude/skills/fbim-reopen
ln -s /path/to/fbim/skills/fbim-list    ~/.claude/skills/fbim-list
ln -s /path/to/fbim/skills/fbim-show    ~/.claude/skills/fbim-show
```

| Command | Description |
|---|---|
| `/fbim [create] <title>` | Create an issue |
| `/fbim-create <title>` | Create an issue |
| `/fbim done <NNNNN>` | Close an issue |
| `/fbim-done <NNNNN>` | Close an issue |
| `/fbim pending <NNNNN>` | Put an issue on hold |
| `/fbim-pending <NNNNN>` | Put an issue on hold |
| `/fbim reopen <NNNNN>` | Reopen a closed issue |
| `/fbim-reopen <NNNNN>` | Reopen a closed issue |
| `/fbim-list` | List issues |
| `/fbim-show <NNNNN>` | Show issue details |

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

## Development

```sh
pip install -r requirements-dev.txt
python3 -m pytest tests/ -v
```

[spec.md](spec.md) is the authoritative specification for file format, naming rules, and tool behavior. README covers usage; spec.md covers the rules behind it.

## Customization

Place `.fbim.yml` in the project root to configure FBIM behavior.

```yaml
issues_dir: issues    # Where to store issues, relative to this file (default: issues)

area_order:           # Display order in the issue list (alphabetical if omitted)
  - backend
  - frontend
  - infra
  - misc

area_labels:          # Display names for areas (area name used as-is if omitted)
  backend: "Backend"
  frontend: "Frontend"
  infra: "Infrastructure"
  misc: "Other"
```

`issues_dir` lets you store issues in a non-default location (e.g. `docs/issues`). FBIM tools walk up from the current directory to find `.fbim.yml`, so they work from any subdirectory of the project.
