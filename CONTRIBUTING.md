# Contributing to Renga

## Development environment

**Prerequisites**

- Rust 1.95.0 (pinned via `rust-toolchain.toml` — `rustup` picks it up automatically)
- `git-cliff` for changelog generation: `cargo install git-cliff`
- `cargo-llvm-cov` for coverage: `cargo install cargo-llvm-cov --locked`

**Build and test**

```sh
cargo build                      # debug build
cargo build --release            # release build
cargo test                       # unit + integration tests
cargo test -- --test-threads=1   # run tests sequentially (required when tests change CWD)
cargo test --doc                 # doctests
cargo clippy -- -D warnings      # lint (must be clean)
cargo fmt --check                # format check
cargo doc --no-deps              # verify doc generation
```

## Coverage

```sh
cargo llvm-cov --summary-only -- --test-threads=1   # summary (check before committing)
cargo llvm-cov --html -- --test-threads=1           # HTML report (target/llvm-cov/)
```

Check coverage both after implementation and after applying review feedback.

## Commit messages

Follow [Conventional Commits](https://www.conventionalcommits.org/). The type is used by git-cliff to group entries in the changelog.

```
<type>[(<scope>)]: <description>
```

| Type | Use when |
|---|---|
| `feat` | Adding or changing user-visible behaviour |
| `fix` | Bug fixes |
| `docs` | Documentation only |
| `test` | Tests only |
| `refactor` | Code restructuring with no behaviour change |
| `style` | Formatting, whitespace |
| `ci` | CI/CD pipeline changes |
| `chore` | Maintenance (deps, release prep, etc.) |

**Agent-internal changes** (`scope: agent`) — changes to `CLAUDE.md`, `.claude/agents/`, or Claude Code skills that have no effect on the CLI or library. Use `chore(agent):` so git-cliff excludes them from the changelog automatically. Do not use `feat(agent):` or `refactor(agent):` for these changes.

**Breaking changes** — add `!` after the type (`feat!:`) or add a `BREAKING CHANGE:` footer. Include migration steps in the commit body.

Use English for the description.

**Write for the reader of the changelog, not the implementer.** The `description` is the exact text that appears in `CHANGELOG.md` via git-cliff. Write it so a user upgrading between versions understands what changed and why it matters — not what internal work was done.

- `feat:` — what the user can now do that they couldn't before
- `fix:` — what specific misbehaviour was corrected

| | Example |
|---|---|
| Bad | `fix: clear release blockers` |
| Bad | `feat: refactor status enum` |
| Good | `fix: skip unparseable issue files instead of aborting` |
| Good | `feat: accept multiple IDs for done/pending/reopen` |

Internal work, issue numbers, and implementation details belong in the commit body, not the subject.

## Branching

- `main` — always releasable. Direct commits for small fixes are fine; use a feature branch for anything larger.
- Feature branches — name them `<type>/<short-description>`, e.g. `feat/plain-ids`.
- No long-lived branches. Merge or rebase promptly.

## Releasing

Releases follow [Semantic Versioning](https://semver.org/). Because Renga is pre-1.0:

- `0.x.0` — new features or breaking changes
- `0.x.y` — bug fixes only

**Release steps**

1. Make and commit all changes with appropriate Conventional Commit messages.
2. Generate the changelog for the new version and append the legacy history:
   ```sh
   git cliff v0.(x-1).0..HEAD --tag v0.x.0 -o CHANGELOG.md
   cat CHANGELOG-legacy.md >> CHANGELOG.md
   ```
   Review the output and edit for clarity before committing. To regenerate, repeat the same two commands.
3. Bump the version in `Cargo.toml` (and run `cargo build` to update `Cargo.lock`).
4. Commit the changelog and version bump:
   ```sh
   git add CHANGELOG.md Cargo.toml Cargo.lock
   git commit -m "chore(release): prepare for v0.x.0"
   ```
5. Tag the commit:
   ```sh
   git tag v0.x.0
   ```
6. Push with tags:
   ```sh
   git push && git push --tags
   ```

CI picks up the tag and publishes the binary automatically.

## Issues

This repository uses Renga itself for issue tracking.

```sh
renga create "タイトル" --area <area>   # create an issue
renga list                              # list open/pending issues
renga done <N>                          # close an issue
```

Or use the Claude Code skill: `/renga`.

## Pull requests

- Keep PRs focused — one logical change per PR.
- All CI checks must pass before merging.
- Code review looks for: correctness, test coverage, clippy/fmt compliance, doc comment completeness on public items.
