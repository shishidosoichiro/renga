//! Shell completion script generation and dynamic candidate output.

use std::fs;
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
    let script = match shell {
        clap_complete::Shell::Zsh => ZSH_SCRIPT,
        clap_complete::Shell::Bash => BASH_SCRIPT,
        clap_complete::Shell::Fish => FISH_SCRIPT,
        _ => {
            let mut cmd = Cli::command();
            clap_complete::generate(shell, &mut cmd, "fbim", &mut std::io::stdout());
            return Ok(());
        }
    };
    print!("{script}");
    Ok(())
}

/// Run `fbim __complete <shell words>`: print completion candidates.
///
/// Called by the shell completion script at completion time. Receives the full
/// command-line token list from the shell (`$words` in zsh, `${COMP_WORDS[@]}` in bash).
///
/// Output: one candidate per line as `CANDIDATE\tDESCRIPTION` (tab-separated).
/// The shell filters by the current partial word using prefix matching.
pub fn complete(args: &[String], ctx: &Context) -> Result<()> {
    // args[0] = binary name, args[1] = subcommand (possibly partial), args[2+] = further args
    let subcmd = args.get(1).map(|s| s.as_str()).unwrap_or("");

    if args.len() <= 2 {
        print_subcommands();
        return Ok(());
    }

    let prev = args.get(args.len() - 2).map(|s| s.as_str()).unwrap_or("");

    match subcmd {
        "done" | "pending" | "show" => print_open_issues(ctx)?,
        "reopen" => print_done_issues(ctx)?,
        "completions" => {
            for shell in ["bash", "zsh", "fish", "powershell", "elvish"] {
                println!("{shell}");
            }
        }
        "list" => match prev {
            "--status" => {
                println!("open");
                println!("pending");
                println!("done");
            }
            _ => {
                println!("--status\tFilter by status");
                println!("--area\tFilter by area");
                println!("--label\tFilter by label");
                println!("--json\tOutput as JSON");
            }
        },
        "create" => match prev {
            "--priority" => {
                println!("high");
                println!("medium");
                println!("low");
            }
            _ => {
                println!("--slug\tCustom filename slug");
                println!("--priority\tPriority level");
                println!("--area\tArea");
                println!("--body\tBody text");
            }
        },
        _ => print_subcommands(),
    }

    Ok(())
}

fn print_subcommands() {
    println!("create\tCreate a new issue");
    println!("done\tMark an issue as done");
    println!("pending\tMark an issue as pending");
    println!("reopen\tReopen an issue");
    println!("list\tList issues");
    println!("show\tShow issue details");
    println!("completions\tGenerate shell completions");
    println!("help\tShow help");
}

fn print_open_issues(ctx: &Context) -> Result<()> {
    if ctx.issues_dir.exists() {
        print_issues_in_dir(&ctx.issues_dir)?;
    }
    Ok(())
}

fn print_done_issues(ctx: &Context) -> Result<()> {
    if ctx.done_dir.exists() {
        print_issues_in_dir(&ctx.done_dir)?;
    }
    Ok(())
}

fn print_issues_in_dir(dir: &Path) -> Result<()> {
    let mut entries: Vec<_> = fs::read_dir(dir)?
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
        println!("{id}\t{title}");
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
