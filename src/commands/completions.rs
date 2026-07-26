//! Shell completion script generation and dynamic candidate output.

use std::io::{self, Write};

use anyhow::Result;
use clap::CommandFactory;

use crate::{
    cli::Cli,
    issue::{all_issues, status_dir_name, Issue, Status},
    Context,
};

// Thin wrapper scripts: all logic lives in `renga __complete`.
// Tab-separated `CANDIDATE\tDESCRIPTION` output is handled by each shell appropriately.

// #compdef is only honoured when the file is autoloaded via $fpath.
// compdef _renga renga is required when sourcing directly.
const ZSH_SCRIPT: &str = r#"#compdef renga

_renga() {
    local -a candidates
    local line
    while IFS= read -r line; do
        [[ -n "$line" ]] && candidates+=("${line/$'\t'/:}")
    done < <(renga __complete "${words[@]}" 2>/dev/null)
    _describe 'candidates' candidates
}

compdef _renga renga
"#;

const BASH_SCRIPT: &str = r#"# bash completion for renga
_renga_completion() {
    local cur="${COMP_WORDS[COMP_CWORD]}"
    local candidates
    candidates=$(renga __complete "${COMP_WORDS[@]}" 2>/dev/null | cut -f1)
    COMPREPLY=($(compgen -W "$candidates" -- "$cur"))
}
complete -F _renga_completion renga
"#;

// Fish natively supports tab-separated `value\tdescription` output.
const FISH_SCRIPT: &str = r#"# fish completion for renga
complete -c renga -f
complete -c renga -f -a '(
    set -l tokens (commandline -opc) (commandline -ct)
    renga __complete $tokens 2>/dev/null
)'
"#;

/// Run `renga completions <shell>`: print the shell completion script.
pub fn run(shell: clap_complete::Shell) -> Result<()> {
    ignore_broken_pipe(write_script(shell))
}

/// Run `renga __complete <shell words>`: print completion candidates.
///
/// Called by the shell completion script at completion time. Receives the full
/// command-line token list from the shell (`$words` in zsh, `${COMP_WORDS[@]}` in bash).
///
/// Output: one candidate per line as `CANDIDATE\tDESCRIPTION` (tab-separated).
/// The shell filters by the current partial word using prefix matching.
pub fn complete(args: &[String], ctx: &Context) -> Result<()> {
    ignore_broken_pipe(write_candidates(args, ctx))
}

/// Treat `BrokenPipe` as success; propagate all other errors.
fn ignore_broken_pipe(result: io::Result<()>) -> Result<()> {
    match result {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == io::ErrorKind::BrokenPipe => Ok(()),
        Err(e) => Err(e.into()),
    }
}

fn write_script(shell: clap_complete::Shell) -> io::Result<()> {
    let script = match shell {
        clap_complete::Shell::Zsh => ZSH_SCRIPT,
        clap_complete::Shell::Bash => BASH_SCRIPT,
        clap_complete::Shell::Fish => FISH_SCRIPT,
        _ => {
            let mut cmd = Cli::command();
            clap_complete::generate(shell, &mut cmd, "renga", &mut io::stdout());
            return Ok(());
        }
    };
    io::stdout().write_all(script.as_bytes())
}

fn write_candidates(args: &[String], ctx: &Context) -> io::Result<()> {
    let out = io::stdout();
    let mut out = out.lock();

    // args[0] = binary name, args[1] = subcommand (possibly partial), args[2+] = further args
    let subcmd = args.get(1).map(|s| s.as_str()).unwrap_or("");

    if args.len() <= 2 {
        if subcmd.starts_with('-') {
            writeln!(out, "--version\tPrint version")?;
            writeln!(out, "--help\tPrint help")?;
            return Ok(());
        }
        return emit_subcommands(&mut out);
    }

    let prev = args.get(args.len() - 2).map(|s| s.as_str()).unwrap_or("");

    match subcmd {
        "done" | "pending" | "in-progress" | "edit" => emit_open_issues(&mut out, ctx)?,
        "show" => {
            emit_open_issues(&mut out, ctx)?;
            emit_done_issues(&mut out, ctx)?;
        }
        // `reopen` rejects an issue that is not done, so do not offer one.
        "reopen" => emit_done_issues(&mut out, ctx)?,
        "update" => {
            let emitted = prev
                .strip_prefix("--")
                .map(|flag| emit_flag_values(&mut out, "update", flag))
                .transpose()?
                .unwrap_or(false);
            if !emitted {
                emit_open_issues(&mut out, ctx)?;
                emit_subcmd_flags(&mut out, "update")?;
            }
        }
        "validate" => {
            emit_all_issues(&mut out, ctx)?;
            emit_subcmd_flags(&mut out, "validate")?;
        }
        "completions" => {
            for (shell, desc) in [
                ("bash", "Bash"),
                ("zsh", "Zsh"),
                ("fish", "Fish"),
                ("powershell", "PowerShell"),
                ("elvish", "Elvish"),
            ] {
                writeln!(out, "{shell}\t{desc}")?;
            }
        }
        "list" => {
            if prev == "--status" {
                for (s, desc) in [
                    ("open", "Active issue"),
                    ("pending", "Blocked or deferred"),
                    ("in-progress", "Actively being worked on"),
                    ("done", "The issue is complete"),
                    ("unknown", "Status could not be determined"),
                ] {
                    writeln!(out, "{s}\t{desc}")?;
                }
            } else {
                emit_subcmd_flags(&mut out, "list")?;
            }
        }
        "create" => {
            let emitted = prev
                .strip_prefix("--")
                .map(|flag| emit_flag_values(&mut out, "create", flag))
                .transpose()?
                .unwrap_or(false);
            if !emitted {
                emit_subcmd_flags(&mut out, "create")?;
            }
        }
        _ => emit_subcommands(&mut out)?,
    }

    Ok(())
}

/// Emit `--flag\tHelp text` for all long flags of the given subcommand.
fn emit_subcmd_flags<W: Write>(out: &mut W, subcmd: &str) -> io::Result<()> {
    let cmd = Cli::command();
    let Some(sub) = cmd.get_subcommands().find(|s| s.get_name() == subcmd) else {
        return Ok(());
    };
    for arg in sub.get_arguments() {
        let Some(long) = arg.get_long() else { continue };
        if long == "help" {
            continue;
        }
        let help = arg.get_help().map(|h| h.to_string()).unwrap_or_default();
        if help.is_empty() {
            writeln!(out, "--{long}")?;
        } else {
            writeln!(out, "--{long}\t{help}")?;
        }
    }
    Ok(())
}

/// Emit the `value_parser` candidates for `--flag` in the given subcommand.
/// Returns `true` if any values were emitted, `false` if the flag has no
/// defined possible values (e.g. free-text flags like `--area`).
fn emit_flag_values<W: Write>(out: &mut W, subcmd: &str, flag: &str) -> io::Result<bool> {
    let cmd = Cli::command();
    let Some(sub) = cmd.get_subcommands().find(|s| s.get_name() == subcmd) else {
        return Ok(false);
    };
    let Some(arg) = sub.get_arguments().find(|a| a.get_long() == Some(flag)) else {
        return Ok(false);
    };
    let values = arg.get_possible_values();
    for val in &values {
        let name = val.get_name();
        let help = val.get_help().map(|h| h.to_string()).unwrap_or_default();
        if help.is_empty() {
            writeln!(out, "{name}")?;
        } else {
            writeln!(out, "{name}\t{help}")?;
        }
    }
    Ok(!values.is_empty())
}

fn emit_subcommands<W: Write>(out: &mut W) -> io::Result<()> {
    writeln!(out, "info\tShow project and configuration info")?;
    writeln!(out, "init\tInitialize the issues directory")?;
    writeln!(out, "migrate\tMigrate issues to per-status directories")?;
    writeln!(out, "create\tCreate a new issue")?;
    writeln!(out, "done\tMark an issue as done")?;
    writeln!(out, "pending\tMark an issue as pending")?;
    writeln!(out, "in-progress\tMark an issue as in-progress")?;
    writeln!(out, "reopen\tReopen an issue")?;
    writeln!(out, "list\tList issues")?;
    writeln!(out, "show\tShow issue details")?;
    writeln!(out, "edit\tOpen an issue in $EDITOR")?;
    writeln!(out, "update\tUpdate issue fields")?;
    writeln!(out, "validate\tValidate all issues")?;
    writeln!(out, "completions\tGenerate shell completions")?;
    writeln!(out, "help\tShow help")?;
    Ok(())
}

/// Whether an issue is already done, and so a candidate for `reopen` rather
/// than for `done`/`pending`/`in-progress`.
///
/// Frontmatter status is authoritative, so a file parked in `open/` but marked
/// `done` counts as done. A file whose frontmatter is missing or unparseable
/// has no status to read; there the directory decides, which is what the
/// commands themselves fall back on.
fn is_done(issue: &Issue) -> bool {
    match issue.status {
        Status::Done => true,
        Status::Unknown => status_dir_name(&issue.path).as_deref() == Some("done"),
        _ => false,
    }
}

fn emit_open_issues<W: Write>(out: &mut W, ctx: &Context) -> io::Result<()> {
    emit_issues(out, ctx, |issue| !is_done(issue))
}

fn emit_done_issues<W: Write>(out: &mut W, ctx: &Context) -> io::Result<()> {
    emit_issues(out, ctx, is_done)
}

fn emit_all_issues<W: Write>(out: &mut W, ctx: &Context) -> io::Result<()> {
    emit_issues(out, ctx, |_| true)
}

/// Write `ID\tTITLE` for every issue matching `keep`.
///
/// Candidates come from [`all_issues`] rather than a private directory walk, so
/// the completion list agrees with what `renga list` reports and with what the
/// command being completed will actually accept. `emit_open_issues` and
/// `emit_done_issues` partition the issues exactly, so `show` can concatenate
/// both without duplicates.
fn emit_issues<W: Write>(
    out: &mut W,
    ctx: &Context,
    keep: impl Fn(&Issue) -> bool,
) -> io::Result<()> {
    if !ctx.issues_dir.exists() {
        return Ok(());
    }
    // Never fail a TAB press over bad data: emit what could be read. Note that
    // `all_issues` writes its own warning to stderr for a file it cannot read;
    // the shell wrappers redirect stderr to /dev/null.
    let issues = all_issues(&ctx.issues_dir, None, None, None, None, None).unwrap_or_default();
    for issue in issues.iter().filter(|issue| keep(issue)) {
        writeln!(out, "{}\t{}", issue.id, issue.title)?;
    }
    Ok(())
}
