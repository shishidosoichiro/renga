//! `renga done` command handler.

use anyhow::{Context as _, Result};

use crate::{
    cli::DoneArgs,
    issue::{find_issue, set_frontmatter_field},
    readme, Context, FbimError,
};

/// Run the done command.
pub fn run(args: DoneArgs, ctx: &Context) -> Result<()> {
    ctx.check_issues_dir()?;

    let path = find_issue(&ctx.issues_dir, &args.id, false)?
        .ok_or_else(|| FbimError::IssueNotFound(args.id.clone()))?;

    let done_dir = ctx.status_dir("done");
    std::fs::create_dir_all(&done_dir)?;

    let file_name = path
        .file_name()
        .with_context(|| format!("invalid path: {}", path.display()))?;
    let dest = done_dir.join(file_name);

    let content = std::fs::read_to_string(&path)?;
    let updated = set_frontmatter_field(&content, "status", "done");

    // Write to a temp file then rename so a crash between write and remove
    // does not leave both copies with inconsistent status.
    // dest.exists() is not checked: re-closing an already-done issue is
    // idempotent and overwrites the stale copy intentionally.
    let tmp = dest.with_extension("tmp");
    std::fs::write(&tmp, &updated)?;
    if let Err(e) = std::fs::rename(&tmp, &dest) {
        let _ = std::fs::remove_file(&tmp);
        return Err(e.into());
    }
    std::fs::remove_file(&path)?;

    readme::write_readme(&ctx.issues_dir, &ctx.config)?;
    println!("{}", dest.display());

    Ok(())
}
