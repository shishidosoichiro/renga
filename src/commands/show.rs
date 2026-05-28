//! `renga show` command handler.

use anyhow::Result;

use crate::{cli::ShowArgs, issue::find_issue, Context, FbimError};

/// Run the show command.
pub fn run(args: ShowArgs, ctx: &Context) -> Result<()> {
    ctx.check_issues_dir()?;

    let path = find_issue(&ctx.issues_dir, &args.id, true)?
        .ok_or_else(|| FbimError::IssueNotFound(args.id.clone()))?;

    let content = std::fs::read_to_string(&path)?;
    print!("{content}");

    Ok(())
}
