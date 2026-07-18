//! `renga reopen` command handler.

use std::path::{Path, PathBuf};

use anyhow::{Context as _, Result};

use crate::{
    cli::ReopenArgs,
    issue::{find_issue, issue_root, relocate_issue, set_frontmatter_field, Issue, Status},
    readme, Context, FbimError,
};

/// Run the reopen command.
pub fn run(args: ReopenArgs, ctx: &Context) -> Result<()> {
    ctx.check_issues_dir()?;

    let open_dir = ctx.status_dir("open");
    std::fs::create_dir_all(&open_dir)?;

    let mut had_error = false;
    for id in &args.ids {
        match reopen_one(id, &open_dir, ctx) {
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

fn reopen_one(id: &str, open_dir: &Path, ctx: &Context) -> Result<PathBuf> {
    let path = find_issue(&ctx.issues_dir, id, true)?
        .ok_or_else(|| FbimError::IssueNotFound(id.to_owned()))?;

    let content = std::fs::read_to_string(&path)?;
    let updated = set_frontmatter_field(&content, "status", "open");

    let src_root = issue_root(&path);
    let entry_name = src_root
        .file_name()
        .with_context(|| format!("invalid path: {}", path.display()))?;
    let dest_root = open_dir.join(entry_name);

    if src_root == dest_root {
        if let Ok(issue) = Issue::parse(&path, &content) {
            if issue.status == Status::Open {
                anyhow::bail!("issue {} already exists as an open issue", id);
            }
        }
    } else if dest_root.exists() {
        anyhow::bail!(
            "cannot reopen {}: {} already exists as an open issue",
            id,
            dest_root.display()
        );
    }

    relocate_issue(&path, &updated, open_dir)
}
