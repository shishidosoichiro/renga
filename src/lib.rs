#![deny(missing_docs)]

//! FBIM — File-Based Issue Management.

/// CLI argument definitions.
pub mod cli;
/// Command handler modules.
pub mod commands;
/// Configuration loading from `.fbim.yml`.
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

/// Domain errors for FBIM operations.
#[derive(Debug, thiserror::Error)]
pub enum FbimError {
    /// No issue file found for the given ID.
    #[error("issue {0} not found")]
    IssueNotFound(String),
    /// The issues directory could not be found.
    #[error("issues directory not found (run 'mkdir -p issues/done')")]
    IssuesDirNotFound,
}

/// Runtime context shared across all command handlers.
pub struct Context {
    /// The project root directory (where `.fbim.yml` or `issues/` was found).
    pub project_root: PathBuf,
    /// The issues directory.
    pub issues_dir: PathBuf,
    /// The `done/` subdirectory inside the issues directory.
    pub done_dir: PathBuf,
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
}

/// Parse CLI arguments and dispatch to the appropriate command handler.
pub fn run() -> Result<()> {
    let cli = cli::Cli::parse();
    let (project_root, issues_dir) = project::find_project_root();
    let done_dir = issues_dir.join("done");
    let config = config::Config::load(&project_root)?;

    let ctx = Context {
        project_root,
        issues_dir,
        done_dir,
        config,
    };

    match cli.command {
        cli::Command::Init => commands::init::run(&ctx),
        cli::Command::Create(args) => commands::create::run(args, &ctx),
        cli::Command::Done(args) => commands::done::run(args, &ctx),
        cli::Command::Pending(args) => commands::pending::run(args, &ctx),
        cli::Command::Reopen(args) => commands::reopen::run(args, &ctx),
        cli::Command::List(args) => commands::list::run(args, &ctx),
        cli::Command::Show(args) => commands::show::run(args, &ctx),
        cli::Command::Validate => commands::validate::run(&ctx),
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
