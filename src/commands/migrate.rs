//! `renga migrate` command handler.

use anyhow::{Context as _, Result};
use walkdir::WalkDir;

use crate::{issue::Issue, readme, Context};

/// Run the migrate command: move all issue files from the flat layout to per-status directories.
pub fn run(ctx: &Context) -> Result<()> {
    ctx.check_issues_dir()?;

    for status in ["open", "pending", "in-progress", "done", "unknown"] {
        std::fs::create_dir_all(ctx.status_dir(status))?;
    }

    let files: Vec<_> = WalkDir::new(&ctx.issues_dir)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            let name = e.file_name().to_string_lossy();
            name.ends_with(".md")
                && name != "README.md"
                && name
                    .chars()
                    .next()
                    .map(|c| c.is_ascii_digit())
                    .unwrap_or(false)
        })
        .map(|e| e.path().to_path_buf())
        .collect();

    if files.is_empty() {
        println!("Nothing to migrate.");
        return Ok(());
    }

    let mut moved = 0usize;
    for path in &files {
        let content = std::fs::read_to_string(path)?;
        let status_str = match Issue::parse(path, &content) {
            Ok(issue) => issue.status.to_string(),
            Err(_) => "unknown".to_string(),
        };
        let dest_dir = ctx.status_dir(&status_str);
        let file_name = path
            .file_name()
            .with_context(|| format!("invalid path: {}", path.display()))?;
        let dest = dest_dir.join(file_name);
        if dest.exists() {
            eprintln!(
                "warning: skipping {} — {} already exists",
                path.display(),
                dest.display()
            );
            continue;
        }
        std::fs::rename(path, &dest)?;
        println!("{} -> {}", path.display(), dest.display());
        moved += 1;
    }

    readme::write_readme(&ctx.issues_dir, &ctx.config)?;
    println!("Migrated {moved} issue(s).");
    Ok(())
}
