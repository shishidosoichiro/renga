//! Shell completion script generation and dynamic candidate output.

use std::fs;
use std::io::{self, Write};
use std::path::Path;

use anyhow::Result;
use clap::CommandFactory;

use crate::{cli::Cli, Context};

// Thin wrapper scripts: all logic lives in `fbim __complete`.
// Tab-separated `CANDIDATE\tDESCRIPTION` output is handled by each shell appropriately.

const ZSH_SCRIPT: &str = r#"#compdef fbim

_fbim() {
    local -a candidates
    local line
    while IFS= read -r line; do
        [[ -n "$line" ]] && candidates+=("${line/$'\t'/:}")
    done < <(fbim __complete "${words[@]}" 2>/dev/null)
    _describe 'candidates' candidates
}
"#;

const BASH_SCRIPT: &str = r#"# bash completion for fbim
_fbim_completion() {
    local cur="${COMP_WORDS[COMP_CWORD]}"
    local candidates
    candidates=$(fbim __complete "${COMP_WORDS[@]}" 2>/dev/null | cut -f1)
    COMPREPLY=($(compgen -W "$candidates" -- "$cur"))
}
complete -F _fbim_completion fbim
"#;

// Fish natively supports tab-separated `value\tdescription` output.
const FISH_SCRIPT: &str = r#"# fish completion for fbim
complete -c fbim -f
complete -c fbim -f -a '(
    set -l tokens (commandline -opc) (commandline -ct)
    fbim __complete $tokens 2>/dev/null
)'
"#;

/// Run `fbim completions <shell>`: print the shell completion script.
pub fn run(shell: clap_complete::Shell) -> Result<()> {
    ignore_broken_pipe(write_script(shell))
}

/// Run `fbim __complete <shell words>`: print completion candidates.
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
            clap_complete::generate(shell, &mut cmd, "fbim", &mut io::stdout());
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
        return emit_subcommands(&mut out);
    }

    let prev = args.get(args.len() - 2).map(|s| s.as_str()).unwrap_or("");

    match subcmd {
        "done" | "pending" | "show" => emit_open_issues(&mut out, ctx)?,
        "reopen" => emit_done_issues(&mut out, ctx)?,
        "completions" => {
            for shell in ["bash", "zsh", "fish", "powershell", "elvish"] {
                writeln!(out, "{shell}")?;
            }
        }
        "list" => match prev {
            "--status" => {
                writeln!(out, "open")?;
                writeln!(out, "pending")?;
                writeln!(out, "done")?;
            }
            _ => {
                writeln!(out, "--status\tFilter by status")?;
                writeln!(out, "--area\tFilter by area")?;
                writeln!(out, "--label\tFilter by label")?;
                writeln!(out, "--json\tOutput as JSON")?;
            }
        },
        "create" => match prev {
            "--priority" => {
                writeln!(out, "high")?;
                writeln!(out, "medium")?;
                writeln!(out, "low")?;
            }
            _ => {
                writeln!(out, "--slug\tCustom filename slug")?;
                writeln!(out, "--priority\tPriority level")?;
                writeln!(out, "--area\tArea")?;
                writeln!(out, "--body\tBody text")?;
            }
        },
        _ => emit_subcommands(&mut out)?,
    }

    Ok(())
}

fn emit_subcommands<W: Write>(out: &mut W) -> io::Result<()> {
    writeln!(out, "init\tInitialize the issues directory")?;
    writeln!(out, "create\tCreate a new issue")?;
    writeln!(out, "done\tMark an issue as done")?;
    writeln!(out, "pending\tMark an issue as pending")?;
    writeln!(out, "reopen\tReopen an issue")?;
    writeln!(out, "list\tList issues")?;
    writeln!(out, "show\tShow issue details")?;
    writeln!(out, "completions\tGenerate shell completions")?;
    writeln!(out, "help\tShow help")?;
    Ok(())
}

fn emit_open_issues<W: Write>(out: &mut W, ctx: &Context) -> io::Result<()> {
    if ctx.issues_dir.exists() {
        emit_issues_in_dir(out, &ctx.issues_dir)?;
    }
    Ok(())
}

fn emit_done_issues<W: Write>(out: &mut W, ctx: &Context) -> io::Result<()> {
    if ctx.done_dir.exists() {
        emit_issues_in_dir(out, &ctx.done_dir)?;
    }
    Ok(())
}

fn emit_issues_in_dir<W: Write>(out: &mut W, dir: &Path) -> io::Result<()> {
    let mut entries: Vec<_> = fs::read_dir(dir)
        .map_err(io::Error::other)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            let name = e.file_name();
            let s = name.to_string_lossy();
            s.ends_with(".md")
                && s.chars()
                    .next()
                    .map(|c| c.is_ascii_digit())
                    .unwrap_or(false)
        })
        .collect();
    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let path = entry.path();
        let stem = path.file_stem().unwrap_or_default().to_string_lossy();
        let id = stem.split('-').next().unwrap_or("").to_string();
        let title = read_title(&path).unwrap_or_else(|| stem.to_string());
        writeln!(out, "{id}\t{title}")?;
    }
    Ok(())
}

fn read_title(path: &Path) -> Option<String> {
    let content = fs::read_to_string(path).ok()?;
    content
        .lines()
        .find(|l| l.starts_with("# "))
        .map(|l| l.trim_start_matches("# ").to_string())
}
