//! `renga migrate` command handler.

use std::collections::HashSet;

use anyhow::{Context as _, Result};
use walkdir::WalkDir;

use crate::{
    issue::{
        canonical_status_dir, collect_issue_files, issue_root, relocate_issue,
        validate_area_for_group_by, Issue,
    },
    readme, Context,
};

/// Run the migrate command.
///
/// Two steps, both idempotent (re-running finds nothing left to do):
/// 1. Move issue files from the legacy flat layout (`issues/N-slug.md`) into
///    per-status directories (`issues/<status>/N-slug.md`).
/// 2. If `group_by` is configured, relocate any issue not already at its
///    canonical `<area>/<status>` directory — this covers both files just
///    moved by step 1 and files already under a status directory from before
///    `group_by` was enabled.
pub fn run(ctx: &Context) -> Result<()> {
    ctx.check_issues_dir()?;

    for status in ["open", "pending", "in-progress", "done", "unknown"] {
        std::fs::create_dir_all(ctx.status_dir(status))?;
    }

    let flat_files: Vec<_> = WalkDir::new(&ctx.issues_dir)
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

    if flat_files.is_empty() && ctx.config.group_by.is_empty() {
        println!("Nothing to migrate.");
        return Ok(());
    }

    // Keyed by entry name (stable across a relocate) rather than a raw
    // counter, so a file that hops twice — flat -> status in step 1, then
    // status -> area/status in step 2 — is only counted once.
    let mut moved: HashSet<String> = HashSet::new();

    for path in &flat_files {
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
        moved.insert(file_name.to_string_lossy().into_owned());
    }

    // Whether step 2 found any issue not already at its canonical location,
    // regardless of whether the relocation succeeded or was skipped — used
    // below to distinguish "found candidates but skipped them all" (still
    // reported as "Migrated 0 issue(s).") from "nothing needed migrating".
    let mut step2_candidates = 0usize;

    if !ctx.config.group_by.is_empty() {
        // Re-scan fresh so any moves from step 1 above are reflected.
        for path in collect_issue_files(&ctx.issues_dir) {
            let content = std::fs::read_to_string(&path)?;
            let (area, status_str) = match Issue::parse(&path, &content) {
                Ok(issue) => (issue.area, issue.status.to_string()),
                Err(_) => (String::new(), "unknown".to_string()),
            };
            if validate_area_for_group_by(&area, &ctx.config.group_by).is_err() {
                step2_candidates += 1;
                eprintln!(
                    "warning: skipping {} — area '{area}' collides with a reserved status directory name",
                    path.display()
                );
                continue;
            }
            let expected_dir =
                canonical_status_dir(&ctx.issues_dir, &ctx.config.group_by, &area, &status_str);
            if issue_root(&path).parent() == Some(expected_dir.as_path()) {
                continue;
            }
            step2_candidates += 1;
            let Some(entry_name) = issue_root(&path).file_name() else {
                continue;
            };
            let dest_entry = expected_dir.join(entry_name);
            if dest_entry.exists() {
                eprintln!(
                    "warning: skipping {} — {} already exists",
                    path.display(),
                    dest_entry.display()
                );
                continue;
            }
            let dest = relocate_issue(&path, &content, &expected_dir)?;
            println!("{} -> {}", path.display(), dest.display());
            moved.insert(entry_name.to_string_lossy().into_owned());
        }
    }

    if flat_files.is_empty() && step2_candidates == 0 {
        println!("Nothing to migrate.");
        return Ok(());
    }

    readme::write_readme(&ctx.issues_dir, &ctx.config)?;
    println!("Migrated {} issue(s).", moved.len());
    Ok(())
}
