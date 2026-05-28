//! `fbim validate` command handler.

use std::{collections::HashMap, sync::LazyLock};

use anyhow::Result;
use regex::Regex;
use walkdir::WalkDir;

use crate::{
    issue::{Issue, Status},
    Context,
};

static RE_ISSUE_FILE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d+-.*\.md$").unwrap());

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

/// Validate all issue files and return findings.
///
/// Checks for:
/// - Unparseable frontmatter (YAML parse failure)
/// - Invalid `status` value (parsed as `Status::Unknown`)
/// - Duplicate IDs across the issues tree
/// - Missing `schema_version` field (warning only)
pub fn validate(ctx: &Context) -> Result<Vec<Finding>> {
    ctx.check_issues_dir()?;

    let mut files: Vec<std::path::PathBuf> = WalkDir::new(&ctx.issues_dir)
        .min_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| RE_ISSUE_FILE.is_match(&e.file_name().to_string_lossy()))
        .map(|e| e.path().to_path_buf())
        .collect();
    files.sort();

    let mut findings: Vec<Finding> = Vec::new();
    let mut id_map: HashMap<u64, Vec<String>> = HashMap::new();
    let mut issues: Vec<Issue> = Vec::new();

    for path in &files {
        let rel = path
            .strip_prefix(&ctx.issues_dir)
            .unwrap_or(path)
            .to_string_lossy()
            .to_string();

        let content = std::fs::read_to_string(path)?;
        match Issue::parse(path, &content) {
            Ok(issue) => {
                if let Ok(n) = issue.id.parse::<u64>() {
                    id_map.entry(n).or_default().push(rel.clone());
                }
                issues.push(issue);
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

    for issue in &issues {
        let rel = issue
            .path
            .strip_prefix(&ctx.issues_dir)
            .unwrap_or(&issue.path)
            .to_string_lossy()
            .to_string();

        if issue.status == Status::Unknown {
            findings.push(Finding {
                path: rel.clone(),
                message: "invalid status value",
                is_error: true,
            });
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
    Ok(findings)
}

/// Run the validate command.
pub fn run(ctx: &Context) -> Result<()> {
    let findings = validate(ctx)?;
    let errors: Vec<_> = findings.iter().filter(|f| f.is_error).collect();
    let warnings: Vec<_> = findings.iter().filter(|f| !f.is_error).collect();

    for f in &findings {
        let level = if f.is_error { "error" } else { "warning" };
        println!("{level}: {}: {}", f.path, f.message);
    }

    if findings.is_empty() {
        println!("ok");
    } else {
        println!("{} error(s), {} warning(s)", errors.len(), warnings.len());
    }

    if !errors.is_empty() {
        std::process::exit(1);
    }

    Ok(())
}
