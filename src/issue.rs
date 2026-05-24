//! Issue file parsing, searching, and manipulation.

use std::{
    fmt,
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::{Context as _, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

/// Status of an issue.
///
/// # Examples
///
/// ```
/// use fbim::issue::Status;
/// assert_eq!(Status::Open.to_string(), "open");
/// assert_eq!("pending".parse::<Status>().unwrap(), Status::Pending);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    /// The issue is open and needs action.
    Open,
    /// The issue is blocked or deferred.
    Pending,
    /// The issue is complete.
    Done,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Open => write!(f, "open"),
            Status::Pending => write!(f, "pending"),
            Status::Done => write!(f, "done"),
        }
    }
}

impl FromStr for Status {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "open" => Ok(Status::Open),
            "pending" => Ok(Status::Pending),
            "done" => Ok(Status::Done),
            other => Err(anyhow::anyhow!("unknown status: {other}")),
        }
    }
}

/// Priority of an issue.
///
/// # Examples
///
/// ```
/// use fbim::issue::Priority;
/// assert_eq!(Priority::High.to_string(), "high");
/// assert_eq!("low".parse::<Priority>().unwrap(), Priority::Low);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    /// High priority.
    High,
    /// Medium priority.
    Medium,
    /// Low priority.
    Low,
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Priority::High => write!(f, "high"),
            Priority::Medium => write!(f, "medium"),
            Priority::Low => write!(f, "low"),
        }
    }
}

impl FromStr for Priority {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "high" => Ok(Priority::High),
            "medium" => Ok(Priority::Medium),
            "low" => Ok(Priority::Low),
            other => Err(anyhow::anyhow!("unknown priority: {other}")),
        }
    }
}

#[derive(Deserialize)]
struct FrontmatterRaw {
    status: Option<Status>,
    priority: Option<Priority>,
    area: Option<String>,
    #[serde(default)]
    labels: Vec<String>,
}

/// A parsed issue file.
#[derive(Debug)]
pub struct Issue {
    /// Zero-padded ID extracted from the filename (e.g. `"00042"`).
    pub id: String,
    /// Absolute path to the issue file.
    pub path: PathBuf,
    /// Current status.
    pub status: Status,
    /// Priority.
    pub priority: Priority,
    /// Area category (e.g. `"core"`, `"cli"`).
    pub area: String,
    /// Labels attached to the issue.
    pub labels: Vec<String>,
    /// Title from the first `# Heading` in the body.
    pub title: String,
    /// Raw file content, preserved for display and in-place updates.
    pub raw_content: String,
}

impl Issue {
    /// Parse an issue from its file path and raw content string.
    pub fn parse(path: &Path, content: &str) -> Result<Self> {
        let (fm_str, body) = split_frontmatter(content)
            .with_context(|| format!("no frontmatter in {}", path.display()))?;
        let fm: FrontmatterRaw = serde_yaml::from_str(fm_str)
            .with_context(|| format!("invalid frontmatter in {}", path.display()))?;

        Ok(Issue {
            id: extract_id(path),
            path: path.to_path_buf(),
            status: fm.status.unwrap_or(Status::Open),
            priority: fm.priority.unwrap_or(Priority::Medium),
            area: fm.area.unwrap_or_else(|| "misc".to_string()),
            labels: fm.labels,
            title: extract_title(body),
            raw_content: content.to_string(),
        })
    }

    /// Load an issue from disk.
    pub fn load(path: &Path) -> Result<Self> {
        let content =
            std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
        Self::parse(path, &content)
    }
}

/// Search `issues_dir` (and optionally `done/`) for an issue by ID.
///
/// The `id` argument is parsed as an integer so both `"42"` and `"00042"` match
/// the same file.
pub fn find_issue(issues_dir: &Path, id: &str, include_done: bool) -> Result<Option<PathBuf>> {
    let num: u64 = id
        .parse()
        .with_context(|| format!("invalid issue ID: {id}"))?;
    let re = Regex::new(r"^(\d{4,5})-.*\.md$").unwrap();

    let mut dirs = vec![issues_dir.to_path_buf()];
    if include_done {
        dirs.push(issues_dir.join("done"));
    }

    for dir in &dirs {
        if !dir.exists() {
            continue;
        }
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let name = entry.file_name().to_string_lossy().to_string();
            if let Some(cap) = re.captures(&name) {
                if cap[1].parse::<u64>().unwrap_or(0) == num {
                    return Ok(Some(entry.path()));
                }
            }
        }
    }
    Ok(None)
}

/// Collect issues from `issues_dir`, applying optional filters.
///
/// `status_filter`:
/// - `None` — return all statuses from all directories (including `done/`)
/// - `Some(statuses)` — return only matching statuses; include `done/` only if
///   [`Status::Done`] is in the slice
pub fn all_issues(
    issues_dir: &Path,
    status_filter: Option<&[Status]>,
    area_filter: Option<&str>,
    label_filter: Option<&str>,
) -> Result<Vec<Issue>> {
    let done_dir = issues_dir.join("done");
    let mut dirs = vec![issues_dir.to_path_buf()];
    match status_filter {
        None => dirs.push(done_dir),
        Some(statuses) if statuses.contains(&Status::Done) => dirs.push(done_dir),
        _ => {}
    }

    let re = Regex::new(r"^\d{4,5}-.*\.md$").unwrap();
    let mut results = Vec::new();

    for dir in &dirs {
        if !dir.exists() {
            continue;
        }
        let mut files: Vec<PathBuf> = std::fs::read_dir(dir)?
            .filter_map(|e| e.ok())
            .filter(|e| re.is_match(&e.file_name().to_string_lossy()))
            .map(|e| e.path())
            .collect();
        files.sort();

        for path in files {
            let content = std::fs::read_to_string(&path)?;
            let issue = Issue::parse(&path, &content)?;

            if let Some(statuses) = status_filter {
                if !statuses.contains(&issue.status) {
                    continue;
                }
            }
            if let Some(area) = area_filter {
                if issue.area != area {
                    continue;
                }
            }
            if let Some(label) = label_filter {
                if !issue.labels.iter().any(|l| l == label) {
                    continue;
                }
            }
            results.push(issue);
        }
    }

    Ok(results)
}

/// Generate the next zero-padded 5-digit issue ID by scanning existing files.
pub fn next_id(issues_dir: &Path) -> Result<String> {
    let re = Regex::new(r"^(\d{4,5})-").unwrap();
    let mut max: u64 = 0;

    for entry in WalkDir::new(issues_dir).max_depth(2) {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }
        let name = entry.file_name().to_string_lossy();
        if let Some(cap) = re.captures(&name) {
            if let Ok(n) = cap[1].parse::<u64>() {
                max = max.max(n);
            }
        }
    }

    Ok(format!("{:05}", max + 1))
}

/// Generate a URL-safe kebab-case slug from a title (max 30 ASCII characters).
///
/// # Examples
///
/// ```
/// use fbim::issue::make_slug;
/// assert_eq!(make_slug("Hello World"), "hello-world");
/// assert_eq!(make_slug("Rust への書き直し"), "rust");
/// assert_eq!(make_slug(""), "issue");
/// ```
pub fn make_slug(title: &str) -> String {
    let re = Regex::new(r"[^a-zA-Z0-9]+").unwrap();
    let lower = title.to_lowercase();
    let slug = re.replace_all(&lower, "-");
    let slug = slug.trim_matches('-');
    // slug is ASCII-only after replacement, so byte slicing is safe
    let slug = if slug.len() > 30 { &slug[..30] } else { slug };
    if slug.is_empty() {
        "issue".to_string()
    } else {
        slug.to_string()
    }
}

/// Update a single frontmatter field in raw file content without re-serialising.
///
/// Leaves all other lines unchanged. If the field is not found, returns the
/// content unmodified.
///
/// # Examples
///
/// ```
/// use fbim::issue::set_frontmatter_field;
/// let content = "---\nstatus: open\npriority: high\n---\n\n# Title\n";
/// let updated = set_frontmatter_field(content, "status", "done");
/// assert!(updated.contains("status: done"));
/// assert!(updated.contains("priority: high"));
/// ```
pub fn set_frontmatter_field(content: &str, field: &str, value: &str) -> String {
    let prefix = format!("{field}:");
    let mut in_fm = false;
    let mut found = false;
    let mut out: Vec<String> = Vec::new();

    for line in content.lines() {
        if line.trim() == "---" {
            in_fm = !in_fm;
            out.push(line.to_string());
            continue;
        }
        if in_fm && !found && line.starts_with(&prefix) {
            out.push(format!("{field}: {value}"));
            found = true;
        } else {
            out.push(line.to_string());
        }
    }

    let mut result = out.join("\n");
    if content.ends_with('\n') {
        result.push('\n');
    }
    result
}

fn split_frontmatter(content: &str) -> Option<(&str, &str)> {
    let rest = content.strip_prefix("---\n")?;
    if let Some(pos) = rest.find("\n---\n") {
        Some((&rest[..pos], &rest[pos + 5..]))
    } else if let Some(pos) = rest.find("\n---") {
        Some((&rest[..pos], ""))
    } else {
        None
    }
}

fn extract_id(path: &Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .and_then(|s| {
            let re = Regex::new(r"^(\d{4,5})-").ok()?;
            Some(re.captures(s)?[1].to_string())
        })
        .unwrap_or_default()
}

fn extract_title(body: &str) -> String {
    // Strip legacy "NNN: " numeric prefix if present
    let prefix_re = Regex::new(r"^\d+:\s*").unwrap();
    for line in body.lines() {
        if let Some(rest) = line.strip_prefix("# ") {
            return prefix_re.replace(rest.trim(), "").to_string();
        }
    }
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn parse_issue_extracts_all_fields() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("00001-test-issue.md");
        std::fs::write(
            &path,
            "---\nstatus: open\npriority: high\narea: core\nlabels: []\n---\n\n# Test Issue\n",
        )
        .unwrap();
        let issue = Issue::load(&path).unwrap();
        assert_eq!(issue.id, "00001");
        assert_eq!(issue.status, Status::Open);
        assert_eq!(issue.priority, Priority::High);
        assert_eq!(issue.area, "core");
        assert_eq!(issue.title, "Test Issue");
        assert!(issue.labels.is_empty());
    }

    #[test]
    fn parse_issue_handles_4digit_id() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("0042-old.md");
        std::fs::write(
            &path,
            "---\nstatus: pending\npriority: low\narea: misc\nlabels: []\n---\n\n# Old\n",
        )
        .unwrap();
        let issue = Issue::load(&path).unwrap();
        assert_eq!(issue.id, "0042");
        assert_eq!(issue.status, Status::Pending);
    }

    #[test]
    fn set_frontmatter_field_updates_status() {
        let content = "---\nstatus: open\npriority: high\narea: core\n---\n\n# Title\n";
        let updated = set_frontmatter_field(content, "status", "done");
        assert!(updated.contains("status: done"));
        assert!(updated.contains("priority: high"));
        assert!(updated.contains("area: core"));
        assert!(updated.ends_with('\n'));
    }

    #[test]
    fn make_slug_converts_title() {
        assert_eq!(make_slug("Hello World"), "hello-world");
        assert_eq!(make_slug("  --  "), "issue");
    }

    #[test]
    fn make_slug_truncates_at_30_chars() {
        let long = "abcdefghijklmnopqrstuvwxyz12345";
        assert_eq!(make_slug(long).len(), 30);
    }

    #[test]
    fn next_id_returns_one_when_empty() {
        let dir = TempDir::new().unwrap();
        assert_eq!(next_id(dir.path()).unwrap(), "00001");
    }

    #[test]
    fn next_id_increments_from_existing() {
        let dir = TempDir::new().unwrap();
        std::fs::write(dir.path().join("00003-foo.md"), "").unwrap();
        std::fs::write(dir.path().join("00001-bar.md"), "").unwrap();
        assert_eq!(next_id(dir.path()).unwrap(), "00004");
    }

    #[test]
    fn next_id_scans_done_subdir() {
        let dir = TempDir::new().unwrap();
        let done = dir.path().join("done");
        std::fs::create_dir(&done).unwrap();
        std::fs::write(done.join("00010-old.md"), "").unwrap();
        assert_eq!(next_id(dir.path()).unwrap(), "00011");
    }

    #[test]
    fn find_issue_locates_by_numeric_id() {
        let dir = TempDir::new().unwrap();
        std::fs::write(
            dir.path().join("00005-my-issue.md"),
            "---\nstatus: open\n---\n\n# My Issue\n",
        )
        .unwrap();
        let found = find_issue(dir.path(), "5", false).unwrap();
        assert!(found.is_some());
        assert!(found.unwrap().ends_with("00005-my-issue.md"));
    }
}
