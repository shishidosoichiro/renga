//! `renga in-progress` command handler.

use std::path::PathBuf;

use anyhow::Result;

use crate::{
    cli::InProgressArgs,
    issue::{find_active_issue, relocate_issue, set_frontmatter_field, Issue},
    readme, Context, FbimError,
};

/// Run the in-progress command.
pub fn run(args: InProgressArgs, ctx: &Context) -> Result<()> {
    ctx.check_issues_dir()?;

    let mut had_error = false;
    for id in &args.ids {
        match move_one(id, ctx) {
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

fn move_one(id: &str, ctx: &Context) -> Result<PathBuf> {
    let active = find_active_issue(&ctx.issues_dir, id)?
        .ok_or_else(|| FbimError::IssueNotFound(id.to_owned()))?;
    if let Some(warning) = &active.warning {
        warning.warn();
    }
    let path = active.path;

    let content = std::fs::read_to_string(&path)?;
    // Tolerate unparseable frontmatter (pre-group_by behavior): fall back to
    // no area, which places the issue at the flat `issues/in-progress/`
    // directory regardless of group_by.
    let area = Issue::parse(&path, &content).map_or_else(|_| String::new(), |issue| issue.area);
    let updated = set_frontmatter_field(&content, "status", "in-progress");

    let dest_dir = ctx.canonical_dir(&area, "in-progress");
    relocate_issue(&path, &updated, &dest_dir)
}
