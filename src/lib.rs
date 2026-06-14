#![deny(missing_docs)]

//! Renga — File-Based Issue Management.

/// CLI argument definitions.
pub mod cli;
/// Command handler modules.
pub mod commands;
/// Configuration loading from `.renga.yml`.
pub mod config;
/// Issue file parsing and manipulation.
pub mod issue;
/// Project root discovery.
pub mod project;
/// `issues/README.md` generation.
pub mod readme;

use std::path::PathBuf;

use anyhow::Result;
use clap::{CommandFactory, Parser};

/// Domain errors for Renga operations.
#[derive(Debug, thiserror::Error)]
pub enum FbimError {
    /// No issue file found for the given ID.
    #[error("issue {0} not found")]
    IssueNotFound(String),
    /// The issues directory could not be found.
    #[error("issues directory not found (run 'renga init')")]
    IssuesDirNotFound,
}

/// Runtime context shared across all command handlers.
pub struct Context {
    /// The project root directory (where `.renga.yml` or `issues/` was found).
    pub project_root: PathBuf,
    /// The issues directory.
    pub issues_dir: PathBuf,
    /// The project configuration.
    pub config: config::Config,
}

impl Context {
    /// Return an error if the issues directory does not exist.
    pub fn check_issues_dir(&self) -> Result<()> {
        if !self.issues_dir.exists() {
            return Err(FbimError::IssuesDirNotFound.into());
        }
        Ok(())
    }

    /// Return the subdirectory for a given status name (e.g. `"open"`, `"done"`).
    pub fn status_dir(&self, status: &str) -> PathBuf {
        self.issues_dir.join(status)
    }
}

/// Parse CLI arguments and dispatch to the appropriate command handler.
pub fn run() -> Result<()> {
    let cli = cli::Cli::parse();
    let (project_root, issues_dir) = project::find_project_root();
    let config = config::Config::load(&project_root)?;

    let ctx = Context {
        project_root,
        issues_dir,
        config,
    };

    match cli.command {
        cli::Command::Info => commands::info::run(&ctx),
        cli::Command::Init => commands::init::run(&ctx),
        cli::Command::Migrate => commands::migrate::run(&ctx),
        cli::Command::Create(args) => commands::create::run(args, &ctx),
        cli::Command::Done(args) => commands::done::run(args, &ctx),
        cli::Command::Pending(args) => commands::pending::run(args, &ctx),
        cli::Command::InProgress(args) => commands::in_progress::run(args, &ctx),
        cli::Command::Reopen(args) => commands::reopen::run(args, &ctx),
        cli::Command::List(args) => commands::list::run(args, &ctx),
        cli::Command::Show(args) => commands::show::run(args, &ctx),
        cli::Command::Edit(args) => commands::edit::run(args, &ctx),
        cli::Command::Update(args) => commands::update::run(args, &ctx),
        cli::Command::Validate(args) => commands::validate::run(args, &ctx),
        cli::Command::Help { command } => {
            let mut cmd = cli::Cli::command();
            match command.as_deref() {
                None => cmd.print_help()?,
                Some(name) => {
                    let sub = cmd
                        .get_subcommands()
                        .find(|s| s.get_name() == name)
                        .cloned();
                    match sub {
                        Some(mut s) => s.print_help()?,
                        None => {
                            eprintln!("error: unknown command: {name}");
                            std::process::exit(1);
                        }
                    }
                }
            }
            println!();
            Ok(())
        }
        cli::Command::Completions { shell } => commands::completions::run(shell),
        cli::Command::Complete { args } => commands::completions::complete(&args, &ctx),
    }
}
