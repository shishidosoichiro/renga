//! `renga edit` command handler.

use anyhow::{Context as _, Result};

use crate::{cli::EditArgs, issue::find_active_issue, readme, Context, FbimError};

/// Run the edit command.
pub fn run(args: EditArgs, ctx: &Context) -> Result<()> {
    ctx.check_issues_dir()?;

    let active = find_active_issue(&ctx.issues_dir, &args.id)?
        .ok_or_else(|| FbimError::IssueNotFound(args.id.clone()))?;
    if let Some(warning) = &active.warning {
        warning.warn();
    }
    let path = active.path;

    let editor = std::env::var("EDITOR")
        .or_else(|_| std::env::var("VISUAL"))
        .unwrap_or_else(|_| "vi".to_string());

    std::process::Command::new(&editor)
        .arg(&path)
        .status()
        .with_context(|| format!("failed to launch editor: {editor}"))?;

    readme::write_readme(&ctx.issues_dir, &ctx.config)?;
    Ok(())
}
