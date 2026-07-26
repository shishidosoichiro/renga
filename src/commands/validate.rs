//! `renga validate` command handler.

use anyhow::{Context as _, Result};
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
};

use crate::{
    cli::ValidateArgs,
    issue::{
        canonical_status_dir, collect_issue_files, id_prefix, is_dir_based, issue_file_id,
        issue_root, relocate_issue, split_frontmatter, validate_area_for_group_by, Issue, Status,
    },
    readme, Context,
};

/// A single validation finding.
#[derive(Debug)]
pub struct Finding {
    /// Relative path to the issue file, for display.
    pub path: String,
    /// Human-readable description of the problem.
    pub message: &'static str,
    /// Whether this finding is an error (true) or a warning (false).
    pub is_error: bool,
}

struct Correction {
    from: String,
    to: String,
}

struct ValidateResult {
    findings: Vec<Finding>,
    corrections: Vec<Correction>,
}

/// Validate issue files and return findings.
///
/// Checks for:
/// - Unparseable frontmatter (YAML parse failure)
/// - Invalid `status` value (parsed as `Status::Unknown`)
/// - Status directory mismatch
/// - Duplicate IDs across the issues tree
/// - Missing `schema_version` field (warning only)
pub fn validate(ctx: &Context, ids: &[String]) -> Result<Vec<Finding>> {
    Ok(validate_inner(ctx, ids, false)?.findings)
}

fn validate_inner(ctx: &Context, ids: &[String], auto_correct: bool) -> Result<ValidateResult> {
    ctx.check_issues_dir()?;

    let selected_ids = parse_selected_ids(ids)?;

    let all_files = collect_issue_files(&ctx.issues_dir);
    let mut files: Vec<PathBuf> = all_files
        .into_iter()
        .filter(|path| match &selected_ids {
            Some(ids) => path_id_matches(path, ids),
            None => true,
        })
        .collect();
    files.sort();

    let mut findings: Vec<Finding> = Vec::new();
    let mut corrections: Vec<Correction> = Vec::new();
    let mut id_map: HashMap<u64, Vec<String>> = HashMap::new();
    let mut issues: Vec<(Issue, bool)> = Vec::new();
    let mut found_ids: HashSet<u64> = HashSet::new();

    for path in &files {
        let rel = rel_path(path, &ctx.issues_dir);
        if let Some(id) = path_id(path) {
            found_ids.insert(id);
            id_map.entry(id).or_default().push(rel.clone());
        }

        let content =
            std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
        let has_status = frontmatter_declares_status(&content);
        match Issue::parse(path, &content) {
            Ok(issue) => {
                issues.push((issue, has_status));
            }
            Err(_) => {
                findings.push(Finding {
                    path: rel,
                    message: "unparseable frontmatter",
                    is_error: true,
                });
            }
        }
    }

    if let Some(ids) = &selected_ids {
        for id in ids {
            if !found_ids.contains(id) {
                findings.push(Finding {
                    path: format!("id:{id}"),
                    message: "issue not found",
                    is_error: true,
                });
            }
        }
    }

    for (issue, has_status) in &issues {
        let rel = rel_path(&issue.path, &ctx.issues_dir);

        if issue.status == Status::Unknown {
            findings.push(Finding {
                path: rel.clone(),
                message: "invalid status value",
                is_error: true,
            });
        } else if !has_status {
            findings.push(Finding {
                path: rel.clone(),
                message: "missing status",
                is_error: true,
            });
        } else if let Some(status_str) = writable_status_dir(issue.status) {
            if validate_area_for_group_by(&issue.area, &ctx.config.group_by).is_err() {
                findings.push(Finding {
                    path: rel.clone(),
                    message: "area collides with a reserved status directory name",
                    is_error: true,
                });
            } else {
                let expected_dir = canonical_status_dir(
                    &ctx.issues_dir,
                    &ctx.config.group_by,
                    &issue.area,
                    status_str,
                );
                let actual_dir = issue_root(&issue.path).parent();
                if actual_dir != Some(expected_dir.as_path()) {
                    if auto_correct {
                        match correct_status_directory(issue, &expected_dir, ctx) {
                            Ok(correction) => corrections.push(correction),
                            Err(_) => findings.push(Finding {
                                path: rel.clone(),
                                message: "status directory mismatch (auto-correct failed)",
                                is_error: true,
                            }),
                        }
                    } else {
                        findings.push(Finding {
                            path: rel.clone(),
                            message: "status directory mismatch",
                            is_error: true,
                        });
                    }
                }
            }
        }

        if issue.schema_version.is_none() {
            findings.push(Finding {
                path: rel,
                message: "missing schema_version (file predates the field)",
                is_error: false,
            });
        }
    }

    for paths in id_map.values() {
        if paths.len() > 1 {
            for p in paths {
                findings.push(Finding {
                    path: p.clone(),
                    message: "duplicate ID",
                    is_error: true,
                });
            }
        }
    }

    findings.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(ValidateResult {
        findings,
        corrections,
    })
}

fn parse_selected_ids(ids: &[String]) -> Result<Option<HashSet<u64>>> {
    if ids.is_empty() {
        return Ok(None);
    }
    let mut selected = HashSet::new();
    for id in ids {
        let n: u64 = id
            .parse()
            .with_context(|| format!("invalid issue ID: {id}"))?;
        if n == 0 {
            anyhow::bail!("invalid issue ID: {id}");
        }
        selected.insert(n);
    }
    Ok(Some(selected))
}

fn path_id_matches(path: &Path, ids: &HashSet<u64>) -> bool {
    match path_id(path) {
        Some(id) => ids.contains(&id),
        None => false,
    }
}

fn path_id(path: &Path) -> Option<u64> {
    if is_dir_based(path) {
        path.parent()
            .and_then(|p| p.file_name())
            .and_then(|s| s.to_str())
            .and_then(id_prefix)
            .and_then(|id| id.parse::<u64>().ok())
    } else {
        path.file_name()
            .and_then(|s| s.to_str())
            .and_then(issue_file_id)
            .and_then(|id| id.parse::<u64>().ok())
    }
}

fn frontmatter_declares_status(content: &str) -> bool {
    match split_frontmatter(content) {
        Some((frontmatter, _)) => frontmatter
            .lines()
            .any(|line| line.trim_start().starts_with("status:")),
        None => false,
    }
}

fn rel_path(path: &Path, issues_dir: &Path) -> String {
    path.strip_prefix(issues_dir)
        .unwrap_or(path)
        .to_string_lossy()
        .to_string()
}

fn writable_status_dir(status: Status) -> Option<&'static str> {
    match status {
        Status::Open => Some("open"),
        Status::Pending => Some("pending"),
        Status::InProgress => Some("in-progress"),
        Status::Done => Some("done"),
        Status::Unknown => None,
    }
}

fn correct_status_directory(issue: &Issue, dest_dir: &Path, ctx: &Context) -> Result<Correction> {
    std::fs::create_dir_all(dest_dir)?;

    let entry_name = issue_root(&issue.path)
        .file_name()
        .with_context(|| format!("invalid path: {}", issue.path.display()))?;
    let dest_entry = dest_dir.join(entry_name);
    if dest_entry.exists() {
        anyhow::bail!("{} already exists", dest_entry.display());
    }

    let from = rel_path(&issue.path, &ctx.issues_dir);
    let dest_path = relocate_issue(&issue.path, &issue.raw_content, dest_dir)?;
    let to = rel_path(&dest_path, &ctx.issues_dir);
    Ok(Correction { from, to })
}

/// Run the validate command.
pub fn run(args: ValidateArgs, ctx: &Context) -> Result<()> {
    let result = validate_inner(ctx, &args.ids, args.auto_correct)?;
    if args.auto_correct && !result.corrections.is_empty() {
        readme::write_readme(&ctx.issues_dir, &ctx.config)?;
    }

    let errors: Vec<_> = result.findings.iter().filter(|f| f.is_error).collect();
    let warnings: Vec<_> = result.findings.iter().filter(|f| !f.is_error).collect();

    for correction in &result.corrections {
        println!("corrected: {} -> {}", correction.from, correction.to);
    }
    for f in &result.findings {
        let level = if f.is_error { "error" } else { "warning" };
        println!("{level}: {}: {}", f.path, f.message);
    }

    if result.findings.is_empty() && result.corrections.is_empty() {
        println!("ok");
    } else if !result.corrections.is_empty() {
        println!("{} correction(s)", result.corrections.len());
    } else {
        println!("{} error(s), {} warning(s)", errors.len(), warnings.len());
    }

    if !errors.is_empty() {
        std::process::exit(1);
    }

    Ok(())
}
