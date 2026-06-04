//! `renga update` command handler.

use std::io::Read;

use anyhow::{Context as _, Result};

use crate::{
    cli::UpdateArgs,
    issue::{
        find_issue, replace_or_prepend_heading, set_frontmatter_field, split_frontmatter,
        validate_label, Issue,
    },
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
    if !args.add_label.is_empty() || !args.remove_label.is_empty() {
        for l in &args.add_label {
            validate_label(l)?;
        }
        for l in &args.remove_label {
            validate_label(l)?;
        }
        let issue = Issue::parse(&path, &content)?;
        let mut labels = issue.labels.clone();
        for l in &args.add_label {
            if !labels.contains(l) {
                labels.push(l.clone());
            }
        }
        labels.retain(|l| !args.remove_label.contains(l));
        let labels_yaml = format!(
            "[{}]",
            labels
                .iter()
                .map(|l| l.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        );
        content = set_frontmatter_field(&content, "labels", &labels_yaml);
    }
    if !args.title.is_empty() {
        let new_title = args.title.join(" ");
        let (fm_str, body) = split_frontmatter(&content)
            .with_context(|| format!("no frontmatter in {}", path.display()))?;
        let new_body = replace_or_prepend_heading(body, &new_title);
        content = format!("---\n{fm_str}\n---\n\n{new_body}");
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
        let (fm_str, existing_body) = split_frontmatter(&content)
            .with_context(|| format!("no frontmatter in {}", path.display()))?;
        let body_with_title = if body_text.lines().any(|l| l.starts_with("# ")) {
            body_text
        } else {
            let existing_heading = existing_body
                .lines()
                .find(|l| l.starts_with("# "))
                .map(|l| l.to_string())
                .unwrap_or_else(|| {
                    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("issue");
                    format!("# {stem}")
                });
            format!("{existing_heading}\n\n{body_text}")
        };
        content = format!("---\n{fm_str}\n---\n\n{body_with_title}\n");
    }

    // If --status was given, move the file to the matching status directory.
    let dest = if let Some(new_status) = &args.status {
        let dest_dir = ctx.status_dir(new_status);
        std::fs::create_dir_all(&dest_dir)?;
        let file_name = path
            .file_name()
            .with_context(|| format!("invalid path: {}", path.display()))?;
        dest_dir.join(file_name)
    } else {
        path.clone()
    };

    if dest != path {
        let tmp = dest.with_extension("tmp");
        std::fs::write(&tmp, &content)?;
        if let Err(e) = std::fs::rename(&tmp, &dest) {
            let _ = std::fs::remove_file(&tmp);
            return Err(e.into());
        }
        std::fs::remove_file(&path)?;
    } else {
        std::fs::write(&dest, &content)?;
    }

    readme::write_readme(&ctx.issues_dir, &ctx.config)?;
    println!("{}", dest.display());

    Ok(())
}
