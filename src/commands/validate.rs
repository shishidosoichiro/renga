//! `renga validate` command handler.

use anyhow::{Context as _, Result};
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

use crate::{
    cli::ValidateArgs,
    issue::{
        id_prefix, is_dir_based, is_issue_file_name, issue_file_id, issue_root, split_frontmatter,
        Issue, Status,
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

        let content = std::fs::read_to_string(path)?;
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
        } else if let Some(expected_dir) = writable_status_dir(issue.status) {
            if status_dir_name(&issue.path, &ctx.issues_dir).as_deref() != Some(expected_dir) {
                if auto_correct {
                    match correct_status_directory(issue, expected_dir, ctx) {
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

fn collect_issue_files(issues_dir: &Path) -> Vec<PathBuf> {
    WalkDir::new(issues_dir)
        .min_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let name = e.file_name().to_string_lossy().into_owned();
            if e.file_type().is_file() && is_issue_file_name(&name) {
                Some(e.path().to_path_buf())
            } else if e.file_type().is_dir() && id_prefix(&name).is_some() {
                let readme = e.path().join("README.md");
                if readme.exists() {
                    Some(readme)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
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

fn status_dir_name(path: &Path, issues_dir: &Path) -> Option<String> {
    path.strip_prefix(issues_dir)
        .ok()?
        .components()
        .next()?
        .as_os_str()
        .to_str()
        .map(str::to_string)
}

fn correct_status_directory(
    issue: &Issue,
    expected_dir: &str,
    ctx: &Context,
) -> Result<Correction> {
    let dest_dir = ctx.status_dir(expected_dir);
    std::fs::create_dir_all(&dest_dir)?;

    if is_dir_based(&issue.path) {
        let src_root = issue_root(&issue.path);
        let entry_name = src_root
            .file_name()
            .with_context(|| format!("invalid path: {}", issue.path.display()))?;
        let dest_root = dest_dir.join(entry_name);
        if dest_root.exists() {
            anyhow::bail!("{} already exists", dest_root.display());
        }
        let from = rel_path(&issue.path, &ctx.issues_dir);
        std::fs::rename(src_root, &dest_root)?;
        let dest_readme = dest_root.join("README.md");
        let to = rel_path(&dest_readme, &ctx.issues_dir);
        Ok(Correction { from, to })
    } else {
        let file_name = issue
            .path
            .file_name()
            .with_context(|| format!("invalid path: {}", issue.path.display()))?;
        let dest = dest_dir.join(file_name);
        if dest.exists() {
            anyhow::bail!("{} already exists", dest.display());
        }
        let from = rel_path(&issue.path, &ctx.issues_dir);
        std::fs::rename(&issue.path, &dest)?;
        let to = rel_path(&dest, &ctx.issues_dir);
        Ok(Correction { from, to })
    }
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
