//! `renga update` command handler.

use std::io::Read;

use anyhow::{Context as _, Result};

use crate::{
    cli::UpdateArgs,
    issue::{find_issue, set_frontmatter_field, split_frontmatter, validate_label},
    readme, Context, FbimError,
};

/// Run the update command.
pub fn run(args: UpdateArgs, ctx: &Context) -> Result<()> {
    ctx.check_issues_dir()?;

    let path = find_issue(&ctx.issues_dir, &args.id, false)?
        .ok_or_else(|| FbimError::IssueNotFound(args.id.clone()))?;

    let mut content = std::fs::read_to_string(&path)?;

    if let Some(priority) = &args.priority {
        content = set_frontmatter_field(&content, "priority", priority);
    }
    if let Some(area) = &args.area {
        content = set_frontmatter_field(&content, "area", area);
    }
    if let Some(status) = &args.status {
        content = set_frontmatter_field(&content, "status", status);
    }
    if let Some(milestone) = &args.milestone {
        content = set_frontmatter_field(&content, "milestone", milestone);
    }
    if !args.label.is_empty() {
        for l in &args.label {
            validate_label(l)?;
        }
        let labels_yaml = format!(
            "[{}]",
            args.label
                .iter()
                .map(|l| l.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        );
        content = set_frontmatter_field(&content, "labels", &labels_yaml);
    }
    if let Some(body_arg) = &args.body {
        let body_text = if body_arg == "-" {
            let mut buf = String::new();
            std::io::stdin()
                .read_to_string(&mut buf)
                .context("failed to read body from stdin")?;
            buf
        } else {
            body_arg.clone()
        };
        let fm_str = split_frontmatter(&content)
            .map(|(fm, _)| fm)
            .with_context(|| format!("no frontmatter in {}", path.display()))?;
        content = format!("---\n{fm_str}\n---\n\n{body_text}\n");
    }

    std::fs::write(&path, &content)?;
    readme::write_readme(&ctx.issues_dir, &ctx.config)?;
    println!("{}", path.display());

    Ok(())
}
