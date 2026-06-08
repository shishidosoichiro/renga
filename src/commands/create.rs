//! `renga create` command handler.

use std::io::Read;

use anyhow::{Context as _, Result};
use serde::Deserialize;

use crate::{
    cli::CreateArgs,
    issue::{find_issue, make_slug, next_id, validate_label},
    readme, Context,
};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CreateJsonInput {
    title: Option<String>,
    id: Option<String>,
    slug: Option<String>,
    #[serde(default = "default_priority")]
    priority: String,
    #[serde(default)]
    area: String,
    body: Option<String>,
    milestone: Option<String>,
    #[serde(default)]
    labels: Vec<String>,
}

struct CreateInput {
    title: String,
    id: Option<String>,
    slug: Option<String>,
    priority: String,
    area: String,
    body: Option<String>,
    milestone: Option<String>,
    labels: Vec<String>,
}

/// Run the create command.
pub fn run(args: CreateArgs, ctx: &Context) -> Result<()> {
    ctx.check_issues_dir()?;

    let input = read_input(args)?;

    validate_priority(&input.priority)?;

    let slug = input.slug.unwrap_or_else(|| make_slug(&input.title));
    let id = match input.id {
        Some(id) => {
            let n: u64 = id
                .parse()
                .with_context(|| format!("invalid id '{}': must be a positive integer", id))?;
            if n == 0 {
                anyhow::bail!("invalid id '{}': must be a positive integer", id);
            }
            if find_issue(&ctx.issues_dir, &id, true)?.is_some() {
                anyhow::bail!("issue {} already exists", id);
            }
            id
        }
        None => next_id(&ctx.issues_dir)?,
    };
    let open_dir = ctx.status_dir("open");
    std::fs::create_dir_all(&open_dir)?;
    let path = open_dir.join(format!("{id}-{slug}.md"));

    let body_section = match input.body.as_deref() {
        Some(b) if !b.is_empty() => format!("\n{b}\n"),
        _ => "\n".to_string(),
    };

    let milestone_line = match &input.milestone {
        Some(m) => format!("milestone: {m}\n"),
        None => String::new(),
    };

    for l in &input.labels {
        validate_label(l)?;
    }
    let labels_yaml = if input.labels.is_empty() {
        "[]".to_string()
    } else {
        format!("[{}]", input.labels.join(", "))
    };

    let content = format!(
        "---\nschema_version: 1\nstatus: open\npriority: {}\narea: {}\nlabels: {labels_yaml}\n{milestone_line}---\n\n# {}\n{}",
        input.priority, input.area, input.title, body_section
    );

    std::io::Write::write_all(
        &mut std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .with_context(|| format!("failed to create {}", path.display()))?,
        content.as_bytes(),
    )?;
    readme::write_readme(&ctx.issues_dir, &ctx.config)?;
    println!("{}", path.display());

    Ok(())
}

fn read_input(args: CreateArgs) -> Result<CreateInput> {
    if args.json {
        ensure_no_cli_fields_with_json(&args)?;
        let mut buf = String::new();
        std::io::stdin()
            .read_to_string(&mut buf)
            .context("failed to read JSON from stdin")?;
        let json: CreateJsonInput =
            serde_json::from_str(&buf).context("failed to parse JSON input")?;
        let title = json
            .title
            .filter(|title| !title.trim().is_empty())
            .context("JSON input field 'title' is required")?;
        return Ok(CreateInput {
            title,
            id: json.id,
            slug: json.slug,
            priority: json.priority,
            area: json.area,
            body: json.body,
            milestone: json.milestone,
            labels: json.labels,
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

    Ok(CreateInput {
        title: args.title.join(" "),
        id: args.id,
        slug: args.slug,
        priority: args.priority.unwrap_or_else(default_priority),
        area: args.area,
        body,
        milestone: args.milestone,
        labels: args.label,
    })
}

fn ensure_no_cli_fields_with_json(args: &CreateArgs) -> Result<()> {
    if !args.title.is_empty()
        || args.id.is_some()
        || args.slug.is_some()
        || args.priority.is_some()
        || !args.area.is_empty()
        || args.body.is_some()
        || args.milestone.is_some()
        || !args.label.is_empty()
    {
        anyhow::bail!("--json cannot be combined with create field arguments");
    }
    Ok(())
}

fn default_priority() -> String {
    "medium".to_string()
}

fn validate_priority(priority: &str) -> Result<()> {
    match priority {
        "high" | "medium" | "low" => Ok(()),
        _ => anyhow::bail!(
            "invalid priority '{}': must be high, medium, or low",
            priority
        ),
    }
}
