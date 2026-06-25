//! `renga done` command handler.

use std::path::{Path, PathBuf};

use anyhow::{Context as _, Result};

use crate::{
    cli::DoneArgs,
    issue::{find_active_issue, is_dir_based, issue_root, set_frontmatter_field},
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

    if is_dir_based(&path) {
        let entry_name = issue_root(&path)
            .file_name()
            .with_context(|| format!("invalid path: {}", path.display()))?;
        let dest_dir = done_dir.join(entry_name);
        std::fs::write(&path, &updated)?;
        std::fs::rename(issue_root(&path), &dest_dir)?;
        Ok(dest_dir.join("README.md"))
    } else {
        let file_name = path
            .file_name()
            .with_context(|| format!("invalid path: {}", path.display()))?;
        let dest = done_dir.join(file_name);
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
}
