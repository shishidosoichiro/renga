//! `renga pending` command handler.

use anyhow::{Context as _, Result};

use crate::{
    cli::PendingArgs,
    issue::{find_issue, set_frontmatter_field},
    readme, Context, FbimError,
};

/// Run the pending command.
pub fn run(args: PendingArgs, ctx: &Context) -> Result<()> {
    ctx.check_issues_dir()?;

    let path = find_issue(&ctx.issues_dir, &args.id, false)?
        .ok_or_else(|| FbimError::IssueNotFound(args.id.clone()))?;

    let pending_dir = ctx.status_dir("pending");
    std::fs::create_dir_all(&pending_dir)?;

    let file_name = path
        .file_name()
        .with_context(|| format!("invalid path: {}", path.display()))?;
    let dest = pending_dir.join(file_name);

    let content = std::fs::read_to_string(&path)?;
    let updated = set_frontmatter_field(&content, "status", "pending");

    let tmp = dest.with_extension("tmp");
    std::fs::write(&tmp, &updated)?;
    if let Err(e) = std::fs::rename(&tmp, &dest) {
        let _ = std::fs::remove_file(&tmp);
        return Err(e.into());
    }
    if path != dest {
        std::fs::remove_file(&path)?;
    }

    readme::write_readme(&ctx.issues_dir, &ctx.config)?;
    println!("{}", dest.display());

    Ok(())
}
