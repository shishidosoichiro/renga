//! CLI argument definitions using clap derive.

use clap::{Args, Parser, Subcommand};

/// FBIM — File-Based Issue Management.
#[derive(Parser)]
#[command(name = "fbim", about = "File-Based Issue Management", version, disable_help_subcommand = true)]
pub struct Cli {
    /// Subcommand to run.
    #[command(subcommand)]
    pub command: Command,
}

/// Available subcommands.
#[derive(Subcommand)]
pub enum Command {
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
    /// Show help for a command.
    Help {
        /// Command to show help for (omit for overall help).
        command: Option<String>,
    },
}

/// Arguments for `fbim create`.
#[derive(Args)]
pub struct CreateArgs {
    /// Issue title (multiple words, no quotes required).
    #[arg(required = true)]
    pub title: Vec<String>,
    /// Kebab-case slug for the filename (auto-generated from title if omitted).
    #[arg(long)]
    pub slug: Option<String>,
    /// Priority level.
    #[arg(long, default_value = "medium", value_parser = ["high", "medium", "low"])]
    pub priority: String,
    /// Area for categorization (e.g. `core`, `cli`, `docs`).
    #[arg(long, default_value = "misc")]
    pub area: String,
    /// Body text to append to the issue file.
    #[arg(long)]
    pub body: Option<String>,
}

/// Arguments for `fbim done`.
#[derive(Args)]
pub struct DoneArgs {
    /// Issue ID (e.g. `00042` or `42`).
    pub id: String,
}

/// Arguments for `fbim pending`.
#[derive(Args)]
pub struct PendingArgs {
    /// Issue ID (e.g. `00042` or `42`).
    pub id: String,
}

/// Arguments for `fbim reopen`.
#[derive(Args)]
pub struct ReopenArgs {
    /// Issue ID (e.g. `00042` or `42`).
    pub id: String,
}

/// Arguments for `fbim list`.
#[derive(Args)]
pub struct ListArgs {
    /// Filter by status. Comma-separated: `open`, `pending`, `done`.
    #[arg(long)]
    pub status: Option<String>,
    /// Filter by area.
    #[arg(long)]
    pub area: Option<String>,
    /// Filter by label.
    #[arg(long)]
    pub label: Option<String>,
    /// Output as JSON.
    #[arg(long)]
    pub json: bool,
}

/// Arguments for `fbim show`.
#[derive(Args)]
pub struct ShowArgs {
    /// Issue ID (e.g. `00042` or `42`).
    pub id: String,
}
