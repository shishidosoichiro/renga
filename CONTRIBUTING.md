# Contributing to FBIM

## Development environment

**Prerequisites**

- Rust 1.95.0 (pinned via `rust-toolchain.toml` — `rustup` picks it up automatically)
- `git-cliff` for changelog generation: `cargo install git-cliff`

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

**Breaking changes** — add `!` after the type (`feat!:`) or add a `BREAKING CHANGE:` footer.

Use Japanese for the description to match the existing history.

## Branching

- `main` — always releasable. Direct commits for small fixes are fine; use a feature branch for anything larger.
- Feature branches — name them `<type>/<short-description>`, e.g. `feat/plain-ids`.
- No long-lived branches. Merge or rebase promptly.

## Releasing

Releases follow [Semantic Versioning](https://semver.org/). Because FBIM is pre-1.0:

- `0.x.0` — new features or breaking changes
- `0.x.y` — bug fixes only

**Release steps**

1. Make and commit all changes with appropriate Conventional Commit messages.
2. Regenerate the changelog with the upcoming tag:
   ```sh
   git cliff --tag v0.x.0 -o CHANGELOG.md
   ```
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

This repository uses FBIM itself for issue tracking.

```sh
fbim create "タイトル" --area <area>   # create an issue
fbim list                              # list open/pending issues
fbim done <N>                          # close an issue
```

Or use the Claude Code skill: `/fbim`.

## Pull requests

- Keep PRs focused — one logical change per PR.
- All CI checks must pass before merging.
- Code review looks for: correctness, test coverage, clippy/fmt compliance, doc comment completeness on public items.
