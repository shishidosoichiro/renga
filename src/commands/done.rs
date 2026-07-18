//! `renga done` command handler.

use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::{
    cli::DoneArgs,
    issue::{find_active_issue, relocate_issue, set_frontmatter_field},
    readme, Context, FbimError,
};

/// Run the done command.
pub fn run(args: DoneArgs, ctx: &Context) -> Result<()> {
    ctx.check_issues_dir()?;

    let done_dir = ctx.status_dir("done");
    std::fs::create_dir_all(&done_dir)?;

    let mut had_error = false;
    for id in &args.ids {
        match move_one(id, &done_dir, ctx) {
            Ok(dest) => println!("{}", dest.display()),
            Err(e) => {
                eprintln!("error: {e}");
                had_error = true;
            }
        }
    }

    readme::write_readme(&ctx.issues_dir, &ctx.config)?;

    if had_error {
        std::process::exit(1);
    }
    Ok(())
}

fn move_one(id: &str, done_dir: &Path, ctx: &Context) -> Result<PathBuf> {
    let active = find_active_issue(&ctx.issues_dir, id)?
        .ok_or_else(|| FbimError::IssueNotFound(id.to_owned()))?;
    if let Some(warning) = &active.warning {
        warning.warn();
    }
    let path = active.path;

    let content = std::fs::read_to_string(&path)?;
    let updated = set_frontmatter_field(&content, "status", "done");

    relocate_issue(&path, &updated, done_dir)
}
