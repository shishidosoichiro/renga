//! `renga list` command handler.

use anyhow::{Context as _, Result};
use serde::Serialize;

use crate::{
    cli::ListArgs,
    issue::{all_issues, Priority, Status},
    Context,
};

/// Run the list command.
pub fn run(args: ListArgs, ctx: &Context) -> Result<()> {
    ctx.check_issues_dir()?;

    let status_filter: Vec<Status> = match args.status.as_deref() {
        Some(s) => s
            .split(',')
            .map(|p| {
                p.trim()
                    .parse::<Status>()
                    .with_context(|| format!("unknown status: '{}'", p.trim()))
            })
            .collect::<Result<Vec<_>>>()?,
        None => vec![Status::Open, Status::Pending, Status::InProgress],
    };

    let issues = all_issues(
        &ctx.issues_dir,
        Some(&status_filter),
        args.area.as_deref(),
        args.label.as_deref(),
        args.milestone.as_deref(),
    )?;

    if args.json {
        #[derive(Serialize)]
        struct IssueJson<'a> {
            id: &'a str,
            file: String,
            status: &'a str,
            priority: &'a str,
            area: &'a str,
            labels: &'a [String],
            milestone: Option<&'a str>,
            title: &'a str,
        }

        let root = ctx.issues_dir.parent().unwrap_or(&ctx.issues_dir);
        let rows: Vec<IssueJson> = issues
            .iter()
            .map(|i| IssueJson {
                id: &i.id,
                file: i
                    .path
                    .strip_prefix(root)
                    .unwrap_or(&i.path)
                    .to_string_lossy()
                    .to_string(),
                status: match i.status {
                    Status::Open => "open",
                    Status::Pending => "pending",
                    Status::InProgress => "in-progress",
                    Status::Done => "done",
                    Status::Unknown => "unknown",
                },
                priority: match i.priority {
                    Priority::High => "high",
                    Priority::Medium => "medium",
                    Priority::Low => "low",
                    Priority::Unknown => "-",
                },
                area: &i.area,
                labels: &i.labels,
                milestone: i.milestone.as_deref(),
                title: &i.title,
            })
            .collect();

        println!("{}", serde_json::to_string_pretty(&rows)?);
    } else {
        for i in &issues {
            println!(
                "[{}] {:8} {:6} {:16} {}",
                i.id, i.status, i.priority, i.area, i.title
            );
        }
    }

    Ok(())
}
