//! `renga show` command handler.

use anyhow::Result;
use serde::Serialize;

use crate::{
    cli::ShowArgs,
    issue::{find_issue, Issue, Priority, Status},
    Context, FbimError,
};

/// Run the show command.
pub fn run(args: ShowArgs, ctx: &Context) -> Result<()> {
    ctx.check_issues_dir()?;

    let path = find_issue(&ctx.issues_dir, &args.id, true)?
        .ok_or_else(|| FbimError::IssueNotFound(args.id.clone()))?;

    if args.json {
        let issue = Issue::load(&path)?;

        #[derive(Serialize)]
        struct IssueJson<'a> {
            id: &'a str,
            file: String,
            status: &'a str,
            priority: &'a str,
            area: &'a str,
            labels: &'a [String],
            milestone: Option<&'a str>,
            assignee: Option<&'a str>,
            title: &'a str,
        }

        let root = ctx.issues_dir.parent().unwrap_or(&ctx.issues_dir);
        let row = IssueJson {
            id: &issue.id,
            file: issue
                .path
                .strip_prefix(root)
                .unwrap_or(&issue.path)
                .to_string_lossy()
                .to_string(),
            status: match issue.status {
                Status::Open => "open",
                Status::Pending => "pending",
                Status::InProgress => "in-progress",
                Status::Done => "done",
                Status::Unknown => "unknown",
            },
            priority: match issue.priority {
                Priority::High => "high",
                Priority::Medium => "medium",
                Priority::Low => "low",
                Priority::Unknown => "-",
            },
            area: &issue.area,
            labels: &issue.labels,
            milestone: issue.milestone.as_deref(),
            assignee: issue.assignee.as_deref(),
            title: &issue.title,
        };
        println!("{}", serde_json::to_string_pretty(&row)?);
    } else {
        let content = std::fs::read_to_string(&path)?;
        print!("{content}");
    }

    Ok(())
}
