//! `renga in-progress` command handler.

use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::{
    cli::InProgressArgs,
    issue::{find_active_issue, relocate_issue, set_frontmatter_field},
    readme, Context, FbimError,
};

/// Run the in-progress command.
pub fn run(args: InProgressArgs, ctx: &Context) -> Result<()> {
    ctx.check_issues_dir()?;

    let in_progress_dir = ctx.status_dir("in-progress");
    std::fs::create_dir_all(&in_progress_dir)?;

    let mut had_error = false;
    for id in &args.ids {
        match move_one(id, &in_progress_dir, ctx) {
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

fn move_one(id: &str, in_progress_dir: &Path, ctx: &Context) -> Result<PathBuf> {
    let active = find_active_issue(&ctx.issues_dir, id)?
        .ok_or_else(|| FbimError::IssueNotFound(id.to_owned()))?;
    if let Some(warning) = &active.warning {
        warning.warn();
    }
    let path = active.path;

    let content = std::fs::read_to_string(&path)?;
    let updated = set_frontmatter_field(&content, "status", "in-progress");

    relocate_issue(&path, &updated, in_progress_dir)
}
