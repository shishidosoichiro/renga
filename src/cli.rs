//! CLI argument definitions using clap derive.

use clap::{builder::PossibleValue, Args, Parser, Subcommand};

/// Renga — File-Based Issue Management.
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
    /// Mark an issue as in-progress (actively being worked on).
    #[command(name = "in-progress")]
    InProgress(InProgressArgs),
    /// Reopen a done or pending issue.
    Reopen(ReopenArgs),
    /// List issues (default: open, pending, and in-progress).
    List(ListArgs),
    /// Show the full content of an issue.
    Show(ShowArgs),
    /// Open an issue file in $EDITOR.
    Edit(EditArgs),
    /// Update fields of an issue.
    Update(UpdateArgs),
    /// Validate issue files for schema errors and duplicate IDs.
    Validate(ValidateArgs),
    /// Show project root, issues directory, config file location, and current settings.
    Info,
    /// Migrate issues from flat layout to per-status directories.
    Migrate,
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
    ///   eval "$(renga completions bash)"
    ///
    /// zsh — add to ~/.zshrc:
    ///
    ///   source <(renga completions zsh)
    ///
    /// fish — install once:
    ///
    ///   renga completions fish > ~/.config/fish/completions/renga.fish
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
    #[arg(required_unless_present = "json")]
    pub title: Vec<String>,
    /// Read issue fields as JSON from stdin.
    #[arg(long)]
    pub json: bool,
    /// Issue ID to use instead of auto-incrementing (e.g. `42`).
    #[arg(long)]
    pub id: Option<String>,
    /// Kebab-case slug for the filename (auto-generated from title if omitted).
    #[arg(long)]
    pub slug: Option<String>,
    /// Priority level.
    #[arg(long, value_parser = [
        PossibleValue::new("high").help("High priority"),
        PossibleValue::new("medium").help("Medium priority"),
        PossibleValue::new("low").help("Low priority"),
    ])]
    pub priority: Option<String>,
    /// Area for categorization (e.g. `core`, `cli`, `docs`).
    #[arg(long, default_value = "")]
    pub area: String,
    /// Body text to append to the issue file. Use `-` to read from stdin.
    #[arg(long)]
    pub body: Option<String>,
    /// Milestone to assign (e.g. `v1.0`, `2026-Q3`).
    #[arg(long)]
    pub milestone: Option<String>,
    /// Assignee responsible for the issue (e.g. `alice`, `app-implementer`).
    #[arg(long)]
    pub assignee: Option<String>,
    /// Labels to attach (repeatable: `--label bug --label urgent`).
    #[arg(long)]
    pub label: Vec<String>,
}

/// Arguments for `renga done`.
#[derive(Args)]
pub struct DoneArgs {
    /// Issue ID(s) to mark as done (e.g. `42` or `1 2 3`).
    #[arg(required = true, num_args(1..))]
    pub ids: Vec<String>,
}

/// Arguments for `renga pending`.
#[derive(Args)]
pub struct PendingArgs {
    /// Issue ID(s) to mark as pending (e.g. `42` or `1 2 3`).
    #[arg(required = true, num_args(1..))]
    pub ids: Vec<String>,
}

/// Arguments for `renga in-progress`.
#[derive(Args)]
pub struct InProgressArgs {
    /// Issue ID(s) to mark as in-progress (e.g. `42` or `1 2 3`).
    #[arg(required = true, num_args(1..))]
    pub ids: Vec<String>,
}

/// Arguments for `renga reopen`.
#[derive(Args)]
pub struct ReopenArgs {
    /// Issue ID(s) to reopen (e.g. `42` or `1 2 3`).
    #[arg(required = true, num_args(1..))]
    pub ids: Vec<String>,
}

/// Arguments for `renga list`.
#[derive(Args)]
pub struct ListArgs {
    /// Filter by status. Comma-separated: `open`, `pending`, `in-progress`, `done`, `unknown`.
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
    /// Filter by assignee.
    #[arg(long)]
    pub assignee: Option<String>,
    /// Output as JSON.
    #[arg(long)]
    pub json: bool,
}

/// Arguments for `renga show`.
#[derive(Args)]
pub struct ShowArgs {
    /// Issue ID (e.g. `42`).
    pub id: String,
    /// Output as JSON.
    #[arg(long)]
    pub json: bool,
}

/// Arguments for `renga edit`.
#[derive(Args)]
pub struct EditArgs {
    /// Issue ID (e.g. `42`).
    pub id: String,
}

/// Arguments for `renga update`.
#[derive(Args)]
pub struct UpdateArgs {
    /// Issue ID (e.g. `42`).
    pub id: String,
    /// New title (positional, multi-word). Updates the `# Heading` line in the body.
    #[arg(num_args(0..))]
    pub title: Vec<String>,
    /// Read fields to update as JSON from stdin.
    #[arg(long)]
    pub json: bool,
    /// New priority level.
    #[arg(long, value_parser = [
        PossibleValue::new("high").help("High priority"),
        PossibleValue::new("medium").help("Medium priority"),
        PossibleValue::new("low").help("Low priority"),
    ])]
    pub priority: Option<String>,
    /// New area.
    #[arg(long)]
    pub area: Option<String>,
    /// New status.
    #[arg(long, value_parser = [
        PossibleValue::new("open").help("Active issue"),
        PossibleValue::new("pending").help("Blocked or deferred"),
        PossibleValue::new("in-progress").help("Actively being worked on"),
    ])]
    pub status: Option<String>,
    /// New milestone. Pass an empty string (`--milestone ''`) to remove the field.
    #[arg(long)]
    pub milestone: Option<String>,
    /// New assignee. Pass an empty string (`--assignee ''`) to remove the field.
    #[arg(long)]
    pub assignee: Option<String>,
    /// Replace labels (repeatable). Use `--label foo --label bar`.
    #[arg(long)]
    pub label: Vec<String>,
    /// Add a label without removing others (repeatable).
    #[arg(long)]
    pub add_label: Vec<String>,
    /// Remove a label (repeatable).
    #[arg(long)]
    pub remove_label: Vec<String>,
    /// Replace body text. Use `-` to read from stdin.
    #[arg(long)]
    pub body: Option<String>,
}

/// Arguments for `renga validate`.
#[derive(Args)]
pub struct ValidateArgs {
    /// Issue ID(s) to validate. Omit to validate all issue files.
    #[arg(num_args(0..))]
    pub ids: Vec<String>,
    /// Move files to the status directory declared in frontmatter.
    #[arg(long)]
    pub auto_correct: bool,
}
