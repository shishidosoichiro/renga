//! `fbim create` command handler.

use anyhow::Result;

use crate::{
    cli::CreateArgs,
    issue::{make_slug, next_id},
    readme, Context,
};

/// Run the create command.
pub fn run(args: CreateArgs, ctx: &Context) -> Result<()> {
    ctx.check_issues_dir()?;

    let title = args.title.join(" ");
    let slug = args.slug.unwrap_or_else(|| make_slug(&title));
    let id = next_id(&ctx.issues_dir)?;
    let path = ctx.issues_dir.join(format!("{id}-{slug}.md"));

    let body_section = match args.body.as_deref() {
        Some(b) if !b.is_empty() => format!("\n{b}\n"),
        _ => "\n".to_string(),
    };

    let content = format!(
        "---\nstatus: open\npriority: {}\narea: {}\nlabels: []\n---\n\n# {}\n{}",
        args.priority, args.area, title, body_section
    );

    std::fs::write(&path, &content)?;
    readme::write_readme(&ctx.issues_dir, &ctx.config)?;
    println!("{}", path.display());

    Ok(())
}
