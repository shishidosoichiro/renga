//! CLI argument definitions using clap derive.

use clap::{Args, Parser, Subcommand};

/// FBIM — File-Based Issue Management.
#[derive(Parser)]
#[command(
    name = "renga",
    about = "File-Based Issue Management",
    version,
    disable_help_subcommand = true
)]
pub struct Cli {
    /// Subcommand to run.
    #[command(subcommand)]
    pub command: Command,
}

/// Available subcommands.
#[derive(Subcommand)]
pub enum Command {
    /// Initialize the issues directory in the current project.
    Init,
    /// Create a new issue.
    Create(CreateArgs),
    /// Mark an issue as done and move it to `done/`.
    Done(DoneArgs),
    /// Mark an issue as pending (blocked or deferred).
    Pending(PendingArgs),
    /// Reopen a done or pending issue.
    Reopen(ReopenArgs),
    /// List issues (default: open and pending).
    List(ListArgs),
    /// Show the full content of an issue.
    Show(ShowArgs),
    /// Validate all issue files for schema errors and duplicate IDs.
    Validate,
    /// Show help for a command.
    Help {
        /// Command to show help for (omit for overall help).
        command: Option<String>,
    },
    /// Generate shell completion scripts.
    ///
    /// Prints a completion script to stdout. Source it in your shell to enable
    /// tab completion for subcommands, flags, and issue IDs.
    ///
    /// bash — add to ~/.bashrc:
    ///
    ///   eval "$(fbim completions bash)"
    ///
    /// zsh — add to ~/.zshrc:
    ///
    ///   source <(fbim completions zsh)
    ///
    /// fish — install once:
    ///
    ///   fbim completions fish > ~/.config/fish/completions/fbim.fish
    Completions {
        /// Shell to generate completions for.
        shell: clap_complete::Shell,
    },
    /// Internal: output completion candidates for dynamic shell completion.
    #[command(name = "__complete", hide = true)]
    Complete {
        /// Shell words from the completion context (the full command line token list).
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
}

/// Arguments for `renga create`.
#[derive(Args)]
pub struct CreateArgs {
    /// Issue title (multiple words, no quotes required).
    #[arg(required = true)]
    pub title: Vec<String>,
    /// Issue ID to use instead of auto-incrementing (e.g. `42`).
    #[arg(long)]
    pub id: Option<String>,
    /// Kebab-case slug for the filename (auto-generated from title if omitted).
    #[arg(long)]
    pub slug: Option<String>,
    /// Priority level.
    #[arg(long, default_value = "medium", value_parser = ["high", "medium", "low"])]
    pub priority: String,
    /// Area for categorization (e.g. `core`, `cli`, `docs`).
    #[arg(long, default_value = "")]
    pub area: String,
    /// Body text to append to the issue file. Use `-` to read from stdin.
    #[arg(long)]
    pub body: Option<String>,
    /// Milestone to assign (e.g. `v1.0`, `2026-Q3`).
    #[arg(long)]
    pub milestone: Option<String>,
}

/// Arguments for `renga done`.
#[derive(Args)]
pub struct DoneArgs {
    /// Issue ID (e.g. `00042` or `42`).
    pub id: String,
}

/// Arguments for `renga pending`.
#[derive(Args)]
pub struct PendingArgs {
    /// Issue ID (e.g. `00042` or `42`).
    pub id: String,
}

/// Arguments for `renga reopen`.
#[derive(Args)]
pub struct ReopenArgs {
    /// Issue ID (e.g. `00042` or `42`).
    pub id: String,
}

/// Arguments for `renga list`.
#[derive(Args)]
pub struct ListArgs {
    /// Filter by status. Comma-separated: `open`, `pending`, `done`, `unknown`.
    #[arg(long)]
    pub status: Option<String>,
    /// Filter by area.
    #[arg(long)]
    pub area: Option<String>,
    /// Filter by label.
    #[arg(long)]
    pub label: Option<String>,
    /// Filter by milestone.
    #[arg(long)]
    pub milestone: Option<String>,
    /// Output as JSON.
    #[arg(long)]
    pub json: bool,
}

/// Arguments for `renga show`.
#[derive(Args)]
pub struct ShowArgs {
    /// Issue ID (e.g. `00042` or `42`).
    pub id: String,
}
