//! `renga in-progress` command handler.

use anyhow::Result;

use crate::{
    cli::InProgressArgs,
    issue::{find_issue, set_frontmatter_field},
    readme, Context, FbimError,
};

/// Run the in-progress command.
pub fn run(args: InProgressArgs, ctx: &Context) -> Result<()> {
    ctx.check_issues_dir()?;

    let path = find_issue(&ctx.issues_dir, &args.id, false)?
        .ok_or_else(|| FbimError::IssueNotFound(args.id.clone()))?;

    let content = std::fs::read_to_string(&path)?;
    let updated = set_frontmatter_field(&content, "status", "in-progress");
    std::fs::write(&path, &updated)?;

    readme::write_readme(&ctx.issues_dir, &ctx.config)?;
    println!("{}", path.display());

    Ok(())
}
