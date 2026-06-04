//! `renga reopen` command handler.

use anyhow::{Context as _, Result};

use crate::{
    cli::ReopenArgs,
    issue::{find_issue, set_frontmatter_field, Issue, Status},
    readme, Context, FbimError,
};

/// Run the reopen command.
pub fn run(args: ReopenArgs, ctx: &Context) -> Result<()> {
    ctx.check_issues_dir()?;

    let path = find_issue(&ctx.issues_dir, &args.id, true)?
        .ok_or_else(|| FbimError::IssueNotFound(args.id.clone()))?;

    let open_dir = ctx.status_dir("open");
    std::fs::create_dir_all(&open_dir)?;

    let file_name = path
        .file_name()
        .with_context(|| format!("invalid path: {}", path.display()))?;
    let dest = open_dir.join(file_name);

    let content = std::fs::read_to_string(&path)?;

    if path == dest {
        // Already in open/ — reject if already open.
        if let Ok(issue) = Issue::parse(&path, &content) {
            if issue.status == Status::Open {
                anyhow::bail!("issue {} already exists as an open issue", args.id);
            }
        }
    } else if dest.exists() {
        anyhow::bail!(
            "cannot reopen {}: {} already exists as an open issue",
            args.id,
            dest.display()
        );
    }

    let updated = set_frontmatter_field(&content, "status", "open");

    if path != dest {
        let tmp = dest.with_extension("tmp");
        std::fs::write(&tmp, &updated)?;
        if let Err(e) = std::fs::rename(&tmp, &dest) {
            let _ = std::fs::remove_file(&tmp);
            return Err(e.into());
        }
        std::fs::remove_file(&path)?;
    } else {
        std::fs::write(&dest, &updated)?;
    }

    readme::write_readme(&ctx.issues_dir, &ctx.config)?;
    println!("{}", dest.display());

    Ok(())
}
