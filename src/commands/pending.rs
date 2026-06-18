//! `renga pending` command handler.

use std::path::{Path, PathBuf};

use anyhow::{Context as _, Result};

use crate::{
    cli::PendingArgs,
    issue::{find_active_issue, set_frontmatter_field},
    readme, Context, FbimError,
};

/// Run the pending command.
pub fn run(args: PendingArgs, ctx: &Context) -> Result<()> {
    ctx.check_issues_dir()?;

    let pending_dir = ctx.status_dir("pending");
    std::fs::create_dir_all(&pending_dir)?;

    let mut had_error = false;
    for id in &args.ids {
        match move_one(id, &pending_dir, ctx) {
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

fn move_one(id: &str, pending_dir: &Path, ctx: &Context) -> Result<PathBuf> {
    let active = find_active_issue(&ctx.issues_dir, id)?
        .ok_or_else(|| FbimError::IssueNotFound(id.to_owned()))?;
    if let Some(warning) = &active.warning {
        warning.warn();
    }
    let path = active.path;

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

    Ok(dest)
}
