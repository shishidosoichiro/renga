//! `renga update` command handler.

use std::{io::Read, path::Path};

use anyhow::{Context as _, Result};
use serde::Deserialize;

use crate::{
    cli::UpdateArgs,
    issue::{
        convert_flat_to_dir, find_editable_issue, is_dir_based, issue_root, relocate_issue,
        remove_frontmatter_field, replace_or_prepend_heading, set_frontmatter_field,
        split_frontmatter, validate_area_for_group_by, validate_label, Issue,
    },
    readme, Context, FbimError,
};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct UpdateJsonInput {
    title: Option<String>,
    priority: Option<String>,
    area: Option<String>,
    status: Option<String>,
    milestone: Option<String>,
    assignee: Option<String>,
    labels: Option<Vec<String>>,
    #[serde(default)]
    add_labels: Vec<String>,
    #[serde(default)]
    remove_labels: Vec<String>,
    body: Option<String>,
}

struct UpdateInput {
    id: String,
    title: Option<String>,
    priority: Option<String>,
    area: Option<String>,
    status: Option<String>,
    milestone: Option<String>,
    assignee: Option<String>,
    labels: Option<Vec<String>>,
    add_labels: Vec<String>,
    remove_labels: Vec<String>,
    body: Option<String>,
    dir: Option<bool>,
}

/// Run the update command.
pub fn run(args: UpdateArgs, ctx: &Context) -> Result<()> {
    ctx.check_issues_dir()?;

    let input = read_input(args)?;
    validate_input(&input, &ctx.config.group_by)?;

    let active = find_editable_issue(&ctx.issues_dir, &input.id)?
        .ok_or_else(|| FbimError::IssueNotFound(input.id.clone()))?;
    if let Some(warning) = &active.warning {
        warning.warn();
    }
    let path = active.path;

    if let Some(use_dir) = input.dir {
        ensure_no_other_fields_with_dir(&input)?;
        return if use_dir {
            convert_to_dir(&path, ctx)
        } else {
            convert_to_file(&path, ctx)
        };
    }

    let mut content = std::fs::read_to_string(&path)?;
    Issue::parse(&path, &content)?;

    if let Some(priority) = &input.priority {
        content = set_frontmatter_field(&content, "priority", priority);
    }
    if let Some(area) = &input.area {
        content = set_frontmatter_field(&content, "area", area);
    }
    if let Some(status) = &input.status {
        content = set_frontmatter_field(&content, "status", status);
    }
    if let Some(milestone) = &input.milestone {
        content = if milestone.is_empty() {
            remove_frontmatter_field(&content, "milestone")
        } else {
            set_frontmatter_field(&content, "milestone", milestone)
        };
    }
    if let Some(assignee) = &input.assignee {
        content = if assignee.is_empty() {
            remove_frontmatter_field(&content, "assignee")
        } else {
            set_frontmatter_field(&content, "assignee", assignee)
        };
    }
    if let Some(labels) = &input.labels {
        for l in labels {
            validate_label(l)?;
        }
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
    if !input.add_labels.is_empty() || !input.remove_labels.is_empty() {
        for l in &input.add_labels {
            validate_label(l)?;
        }
        for l in &input.remove_labels {
            validate_label(l)?;
        }
        let issue = Issue::parse(&path, &content)?;
        let mut labels = issue.labels.clone();
        for l in &input.add_labels {
            if !labels.contains(l) {
                labels.push(l.clone());
            }
        }
        labels.retain(|l| !input.remove_labels.contains(l));
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
    if let Some(new_title) = &input.title {
        let (fm_str, body) = split_frontmatter(&content)
            .with_context(|| format!("no frontmatter in {}", path.display()))?;
        let new_body = replace_or_prepend_heading(body, new_title);
        content = format!("---\n{fm_str}\n---\n\n{new_body}");
    }
    if let Some(body_text) = &input.body {
        let (fm_str, existing_body) = split_frontmatter(&content)
            .with_context(|| format!("no frontmatter in {}", path.display()))?;
        let body_with_title = if body_text.lines().any(|l| l.starts_with("# ")) {
            body_text.clone()
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

    // Always relocate to the canonical (area, status) directory. This is a
    // no-op write-in-place when the issue is already there, and also
    // self-heals a misplaced active issue found via find_editable_issue's
    // recoverable-mismatch path.
    let issue = Issue::parse(&path, &content)?;
    let dest_dir = ctx.canonical_dir(&issue.area, &issue.status.to_string());
    let dest = relocate_issue(&path, &content, &dest_dir)?;
    readme::write_readme(&ctx.issues_dir, &ctx.config)?;
    println!("{}", dest.display());

    Ok(())
}

fn read_input(args: UpdateArgs) -> Result<UpdateInput> {
    if args.json {
        ensure_no_cli_fields_with_json(&args)?;
        let mut buf = String::new();
        std::io::stdin()
            .read_to_string(&mut buf)
            .context("failed to read JSON from stdin")?;
        let json: UpdateJsonInput =
            serde_json::from_str(&buf).context("failed to parse JSON input")?;
        return Ok(UpdateInput {
            id: args.id,
            title: json.title,
            priority: json.priority,
            area: json.area,
            status: json.status,
            milestone: json.milestone,
            assignee: json.assignee,
            labels: json.labels,
            add_labels: json.add_labels,
            remove_labels: json.remove_labels,
            body: json.body,
            dir: None,
        });
    }

    let body = match args.body.as_deref() {
        Some("-") => {
            let mut buf = String::new();
            std::io::stdin()
                .read_to_string(&mut buf)
                .context("failed to read body from stdin")?;
            Some(buf)
        }
        other => other.map(|s| s.to_string()),
    };

    Ok(UpdateInput {
        id: args.id,
        title: if args.title.is_empty() {
            None
        } else {
            Some(args.title.join(" "))
        },
        priority: args.priority,
        area: args.area,
        status: args.status,
        milestone: args.milestone,
        assignee: args.assignee,
        labels: if args.label.is_empty() {
            None
        } else {
            Some(args.label)
        },
        add_labels: args.add_label,
        remove_labels: args.remove_label,
        body,
        dir: args.dir,
    })
}

fn ensure_no_cli_fields_with_json(args: &UpdateArgs) -> Result<()> {
    if !args.title.is_empty()
        || args.priority.is_some()
        || args.area.is_some()
        || args.status.is_some()
        || args.milestone.is_some()
        || args.assignee.is_some()
        || !args.label.is_empty()
        || !args.add_label.is_empty()
        || !args.remove_label.is_empty()
        || args.body.is_some()
        || args.dir.is_some()
    {
        anyhow::bail!("--json cannot be combined with update field arguments");
    }
    Ok(())
}

fn ensure_no_other_fields_with_dir(input: &UpdateInput) -> Result<()> {
    if input.title.is_some()
        || input.priority.is_some()
        || input.area.is_some()
        || input.status.is_some()
        || input.milestone.is_some()
        || input.assignee.is_some()
        || input.labels.is_some()
        || !input.add_labels.is_empty()
        || !input.remove_labels.is_empty()
        || input.body.is_some()
    {
        anyhow::bail!("--dir cannot be combined with other update fields");
    }
    Ok(())
}

fn convert_to_dir(path: &Path, ctx: &Context) -> Result<()> {
    let readme = convert_flat_to_dir(path)?;
    readme::write_readme(&ctx.issues_dir, &ctx.config)?;
    println!("{}", readme.display());
    Ok(())
}

fn convert_to_file(path: &Path, ctx: &Context) -> Result<()> {
    if !is_dir_based(path) {
        anyhow::bail!("issue is already a flat file");
    }
    let issue_dir = issue_root(path);
    let entries: Vec<_> = std::fs::read_dir(issue_dir)
        .with_context(|| format!("failed to read {}", issue_dir.display()))?
        .collect::<std::io::Result<_>>()?;
    let only_readme =
        entries.len() == 1 && entries[0].file_name() == std::ffi::OsStr::new("README.md");
    if !only_readme {
        anyhow::bail!(
            "cannot collapse: {} contains files other than README.md",
            issue_dir.display()
        );
    }
    let dir_name = issue_dir
        .file_name()
        .and_then(|n| n.to_str())
        .with_context(|| format!("invalid path: {}", issue_dir.display()))?;
    let parent = issue_dir
        .parent()
        .with_context(|| format!("invalid path: {}", issue_dir.display()))?;
    let dest = parent.join(format!("{dir_name}.md"));
    std::fs::rename(path, &dest)
        .with_context(|| format!("failed to move {} to {}", path.display(), dest.display()))?;
    std::fs::remove_dir(issue_dir)
        .with_context(|| format!("failed to remove {}", issue_dir.display()))?;
    readme::write_readme(&ctx.issues_dir, &ctx.config)?;
    println!("{}", dest.display());
    Ok(())
}

fn validate_input(input: &UpdateInput, group_by: &[String]) -> Result<()> {
    if let Some(priority) = &input.priority {
        match priority.as_str() {
            "high" | "medium" | "low" => {}
            _ => anyhow::bail!(
                "invalid priority '{}': must be high, medium, or low",
                priority
            ),
        }
    }
    if let Some(status) = &input.status {
        match status.as_str() {
            "open" | "pending" | "in-progress" => {}
            _ => anyhow::bail!(
                "invalid status '{}': must be open, pending, or in-progress",
                status
            ),
        }
    }
    if let Some(area) = &input.area {
        validate_area_for_group_by(area, group_by)?;
    }
    Ok(())
}
