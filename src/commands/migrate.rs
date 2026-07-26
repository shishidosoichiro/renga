//! `renga migrate` command handler.

use std::collections::HashSet;

use anyhow::{Context as _, Result};
use walkdir::WalkDir;

use crate::{
    issue::{
        canonical_status_dir, collect_issue_files, convert_flat_to_dir, extract_id, is_dir_based,
        is_issue_file_name, issue_root, relocate_issue, validate_area_for_group_by, Issue,
    },
    readme, Context,
};

/// Run the migrate command.
///
/// Three steps, all idempotent (re-running finds nothing left to do):
/// 1. Move issue files from the legacy flat layout (`issues/N-slug.md`) into
///    per-status directories (`issues/<status>/N-slug.md`).
/// 2. If `defaults.dir` is `true`, convert any still-flat issue to
///    directory-based (`issues/<status>/N-slug/README.md`).
/// 3. If `group_by` is configured, relocate any issue not already at its
///    canonical `<area>/<status>` directory — this covers files moved or
///    converted by steps 1-2 and files already in place from before
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
        .filter(|e| is_issue_file_name(&e.file_name().to_string_lossy()))
        .map(|e| e.path().to_path_buf())
        .collect();

    if flat_files.is_empty()
        && ctx.config.group_by.is_empty()
        && ctx.config.defaults.dir != Some(true)
    {
        println!("Nothing to migrate.");
        return Ok(());
    }

    // Keyed by the issue's numeric ID (stable across any number of hops —
    // flat -> status in step 1, flat -> dir-based in step 2, status ->
    // area/status in step 3) rather than entry name, so a file that moves
    // through multiple steps in one run is only counted once.
    let mut moved: HashSet<String> = HashSet::new();

    for path in &flat_files {
        let content =
            std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
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
        moved.insert(extract_id(&dest));
    }

    // Whether steps 2/3 found any issue not already at its canonical shape
    // or location, regardless of whether the change succeeded or was
    // skipped — used below to distinguish "found candidates but skipped
    // them all" (still reported as "Migrated 0 issue(s).") from "nothing
    // needed migrating".
    let mut dir_candidates = 0usize;

    if ctx.config.defaults.dir == Some(true) {
        for path in collect_issue_files(&ctx.issues_dir) {
            if is_dir_based(&path) {
                continue;
            }
            dir_candidates += 1;
            let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
                continue;
            };
            let Some(parent) = path.parent() else {
                continue;
            };
            let dest_dir = parent.join(stem);
            if dest_dir.exists() {
                eprintln!(
                    "warning: skipping {} — {} already exists",
                    path.display(),
                    dest_dir.display()
                );
                continue;
            }
            let readme = convert_flat_to_dir(&path)?;
            println!("{} -> {}", path.display(), readme.display());
            moved.insert(extract_id(&readme));
        }
    }

    let mut area_candidates = 0usize;

    if !ctx.config.group_by.is_empty() {
        // Re-scan fresh so any moves/conversions from steps 1-2 above are
        // reflected.
        for path in collect_issue_files(&ctx.issues_dir) {
            let content = std::fs::read_to_string(&path)
                .with_context(|| format!("reading {}", path.display()))?;
            let (area, status_str) = match Issue::parse(&path, &content) {
                Ok(issue) => (issue.area, issue.status.to_string()),
                Err(_) => (String::new(), "unknown".to_string()),
            };
            if let Err(e) = validate_area_for_group_by(&area, &ctx.config.group_by) {
                area_candidates += 1;
                eprintln!("warning: skipping {} — {e}", path.display());
                continue;
            }
            let expected_dir =
                canonical_status_dir(&ctx.issues_dir, &ctx.config.group_by, &area, &status_str);
            if issue_root(&path).parent() == Some(expected_dir.as_path()) {
                continue;
            }
            area_candidates += 1;
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
            moved.insert(extract_id(&dest));
        }
    }

    if flat_files.is_empty() && dir_candidates == 0 && area_candidates == 0 {
        println!("Nothing to migrate.");
        return Ok(());
    }

    readme::write_readme(&ctx.issues_dir, &ctx.config)?;
    println!("Migrated {} issue(s).", moved.len());
    Ok(())
}
