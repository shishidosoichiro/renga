//! Issue file parsing, searching, and manipulation.

use std::{
    fmt,
    path::{Path, PathBuf},
    str::FromStr,
    sync::LazyLock,
};

use anyhow::{Context as _, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

static RE_ISSUE_FILE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d+-.*\.md$").unwrap());
static RE_ISSUE_FILE_CAP: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(\d+)-.*\.md$").unwrap());
static RE_ID_PREFIX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(\d+)-").unwrap());
static RE_SLUG_SEPARATOR: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[^a-zA-Z0-9]+").unwrap());
static RE_LINENUM_PREFIX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d+:\s*").unwrap());

/// Status of an issue.
///
/// `Unknown` is assigned when a Markdown file cannot be parsed as an issue
/// (e.g. frontmatter is absent). It is read-only: renga never writes it to a file.
///
/// # Examples
///
/// ```
/// use renga::issue::Status;
/// assert_eq!(Status::Open.to_string(), "open");
/// assert_eq!("pending".parse::<Status>().unwrap(), Status::Pending);
/// assert_eq!(Status::Unknown.to_string(), "unknown");
/// assert_eq!("unknown".parse::<Status>().unwrap(), Status::Unknown);
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
    /// The status could not be determined (e.g. frontmatter is missing).
    #[serde(other)]
    Unknown,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Open => write!(f, "open"),
            Status::Pending => write!(f, "pending"),
            Status::Done => write!(f, "done"),
            Status::Unknown => write!(f, "unknown"),
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
            "unknown" => Ok(Status::Unknown),
            other => Err(anyhow::anyhow!("unknown status: {other}")),
        }
    }
}

/// Priority of an issue.
///
/// `Unknown` is assigned when a file has no frontmatter or the `priority`
/// field is absent.
///
/// # Examples
///
/// ```
/// use renga::issue::Priority;
/// assert_eq!(Priority::High.to_string(), "high");
/// assert_eq!("low".parse::<Priority>().unwrap(), Priority::Low);
/// assert_eq!(Priority::Unknown.to_string(), "-");
/// assert_eq!("-".parse::<Priority>().unwrap(), Priority::Unknown);
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
    /// Priority is not set (file has no frontmatter or missing field).
    Unknown,
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Priority::High => write!(f, "high"),
            Priority::Medium => write!(f, "medium"),
            Priority::Low => write!(f, "low"),
            Priority::Unknown => write!(f, "-"),
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
            "-" => Ok(Priority::Unknown),
            other => Err(anyhow::anyhow!("unknown priority: {other}")),
        }
    }
}

#[derive(Deserialize)]
struct FrontmatterRaw {
    schema_version: Option<u32>,
    status: Option<Status>,
    priority: Option<Priority>,
    area: Option<String>,
    #[serde(default)]
    labels: Vec<String>,
    milestone: Option<String>,
}

/// A parsed issue file.
#[derive(Debug)]
pub struct Issue {
    /// Schema version declared in frontmatter (`schema_version` field).
    ///
    /// `None` means the field is absent (file predates the field). `Some(1)`
    /// is the current version. Files without this field are handled the same
    /// as version 1.
    pub schema_version: Option<u32>,
    /// Integer ID extracted from the filename prefix (e.g. `"42"`).
    ///
    /// The ID lives only in the filename — not in frontmatter or body.
    /// Legacy zero-padded filenames (e.g. `00042-foo.md`) are supported;
    /// their ID string preserves the original form from the filename.
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
    /// Milestone the issue belongs to (e.g. `"v1.0"`, `"2026-Q3"`). Optional.
    pub milestone: Option<String>,
    /// Title extracted from the first `# Heading` line in the body.
    ///
    /// The title lives only in the body — not in frontmatter.
    /// Falls back to the file stem if no heading is found.
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
            schema_version: fm.schema_version,
            id: extract_id(path),
            path: path.to_path_buf(),
            status: fm.status.unwrap_or(Status::Open),
            priority: fm.priority.unwrap_or(Priority::Medium),
            area: fm.area.unwrap_or_default(),
            labels: fm.labels,
            milestone: fm.milestone,
            title: title_or_stem(body, path),
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

/// Search `issues_dir` recursively for an issue by ID.
///
/// The `id` argument is parsed as an integer so both `"42"` and `"00042"` match
/// the same file. When `include_done` is `false`, files under the `done/`
/// subtree are skipped.
pub fn find_issue(issues_dir: &Path, id: &str, include_done: bool) -> Result<Option<PathBuf>> {
    let num: u64 = id
        .parse()
        .with_context(|| format!("invalid issue ID: {id}"))?;
    for entry in WalkDir::new(issues_dir).min_depth(1) {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }
        let name = entry.file_name().to_string_lossy();
        if !RE_ISSUE_FILE_CAP.is_match(name.as_ref()) {
            continue;
        }
        if !include_done {
            let rel = entry
                .path()
                .strip_prefix(issues_dir)
                .unwrap_or(entry.path());
            if rel.starts_with("done") {
                continue;
            }
        }
        if let Some(cap) = RE_ISSUE_FILE_CAP.captures(name.as_ref()) {
            if cap[1].parse::<u64>().unwrap_or(0) == num {
                return Ok(Some(entry.path().to_path_buf()));
            }
        }
    }
    Ok(None)
}

/// Collect issues from `issues_dir` recursively, applying optional filters.
///
/// `status_filter`:
/// - `None` — return all issues regardless of status
/// - `Some(statuses)` — return only issues whose status is in the slice
pub fn all_issues(
    issues_dir: &Path,
    status_filter: Option<&[Status]>,
    area_filter: Option<&str>,
    label_filter: Option<&str>,
    milestone_filter: Option<&str>,
) -> Result<Vec<Issue>> {
    let mut files: Vec<PathBuf> = WalkDir::new(issues_dir)
        .min_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| RE_ISSUE_FILE.is_match(&e.file_name().to_string_lossy()))
        .map(|e| e.path().to_path_buf())
        .collect();

    files.sort_by_key(|p| {
        p.file_name()
            .and_then(|n| n.to_str())
            .and_then(|n| n.split('-').next())
            .and_then(|n| n.parse::<u64>().ok())
            .unwrap_or(0)
    });

    let mut results = Vec::new();
    for path in files {
        let content = std::fs::read_to_string(&path)?;
        let issue = match Issue::parse(&path, &content) {
            Ok(i) => i,
            Err(_) => {
                let title_src = split_frontmatter(&content)
                    .map(|(_, b)| b)
                    .unwrap_or(&content);
                Issue {
                    schema_version: None,
                    id: extract_id(&path),
                    path: path.clone(),
                    status: Status::Unknown,
                    priority: Priority::Unknown,
                    area: String::new(),
                    labels: vec![],
                    milestone: None,
                    title: title_or_stem(title_src, &path),
                    raw_content: content,
                }
            }
        };

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
        if let Some(milestone) = milestone_filter {
            if issue.milestone.as_deref() != Some(milestone) {
                continue;
            }
        }
        results.push(issue);
    }

    Ok(results)
}

/// Generate the next issue ID by scanning existing files.
///
/// Returns a plain integer string with no zero-padding (e.g. `"1"`, `"42"`).
/// Existing zero-padded filenames (e.g. `00042-foo.md`) are recognised and
/// their numeric value is included when computing the next ID.
pub fn next_id(issues_dir: &Path) -> Result<String> {
    let mut max: u64 = 0;

    for entry in WalkDir::new(issues_dir) {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }
        let name = entry.file_name().to_string_lossy();
        if let Some(cap) = RE_ID_PREFIX.captures(&name) {
            if let Ok(n) = cap[1].parse::<u64>() {
                max = max.max(n);
            }
        }
    }

    Ok(format!("{}", max + 1))
}

/// Generate a URL-safe kebab-case slug from a title (max 30 ASCII characters).
///
/// # Examples
///
/// ```
/// use renga::issue::make_slug;
/// assert_eq!(make_slug("Hello World"), "hello-world");
/// assert_eq!(make_slug("Rust への書き直し"), "rust");
/// assert_eq!(make_slug(""), "issue");
/// ```
pub fn make_slug(title: &str) -> String {
    let lower = title.to_lowercase();
    let slug = RE_SLUG_SEPARATOR.replace_all(&lower, "-");
    let slug = slug.trim_matches('-');
    // slug is ASCII-only after replacement, so byte slicing is safe
    let slug = if slug.len() > 30 { &slug[..30] } else { slug };
    let slug = slug.trim_end_matches('-');
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
/// use renga::issue::set_frontmatter_field;
/// let content = "---\nstatus: open\npriority: high\n---\n\n# Title\n";
/// let updated = set_frontmatter_field(content, "status", "done");
/// assert!(updated.contains("status: done"));
/// assert!(updated.contains("priority: high"));
/// ```
pub fn set_frontmatter_field(content: &str, field: &str, value: &str) -> String {
    let prefix = format!("{field}:");
    let mut in_fm = false;
    let mut fm_closed = false;
    let mut found = false;
    let mut out: Vec<String> = Vec::new();

    for line in content.lines() {
        if !fm_closed && line.trim() == "---" {
            if in_fm {
                in_fm = false;
                fm_closed = true;
            } else if out.is_empty() {
                // opening fence must be the very first line
                in_fm = true;
            }
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
        .and_then(|s| Some(RE_ID_PREFIX.captures(s)?[1].to_string()))
        .unwrap_or_default()
}

fn extract_title(body: &str) -> String {
    // Strip legacy "NNN: " numeric prefix if present
    for line in body.lines() {
        if let Some(rest) = line.strip_prefix("# ") {
            return RE_LINENUM_PREFIX.replace(rest.trim(), "").to_string();
        }
    }
    String::new()
}

fn title_or_stem(body: &str, path: &Path) -> String {
    let t = extract_title(body);
    if t.is_empty() {
        path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default()
            .to_string()
    } else {
        t
    }
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
            "---\nschema_version: 1\nstatus: open\npriority: high\narea: core\nlabels: []\n---\n\n# Test Issue\n",
        )
        .unwrap();
        let issue = Issue::load(&path).unwrap();
        assert_eq!(issue.schema_version, Some(1));
        assert_eq!(issue.id, "00001");
        assert_eq!(issue.status, Status::Open);
        assert_eq!(issue.priority, Priority::High);
        assert_eq!(issue.area, "core");
        assert_eq!(issue.title, "Test Issue");
        assert!(issue.labels.is_empty());
    }

    #[test]
    fn parse_issue_schema_version_absent_gives_none() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("1-old.md");
        std::fs::write(
            &path,
            "---\nstatus: open\npriority: medium\narea: core\nlabels: []\n---\n\n# Old Issue\n",
        )
        .unwrap();
        let issue = Issue::load(&path).unwrap();
        assert_eq!(issue.schema_version, None);
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
    fn parse_issue_falls_back_to_stem_when_no_title() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("7-no-title.md");
        std::fs::write(
            &path,
            "---\nstatus: open\npriority: medium\narea: misc\nlabels: []\n---\n\nNo H1 heading here.\n",
        )
        .unwrap();
        let issue = Issue::load(&path).unwrap();
        assert_eq!(issue.title, "7-no-title");
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
    fn set_frontmatter_field_ignores_hr_in_body() {
        // '---' in the body must not re-enable frontmatter parsing
        let content = "---\nstatus: open\n---\n\n# Title\n\n---\nstatus: see notes\n";
        let updated = set_frontmatter_field(content, "status", "done");
        assert!(
            updated.contains("status: done"),
            "frontmatter status should be updated"
        );
        assert!(
            updated.contains("status: see notes"),
            "body line must not be modified"
        );
    }

    #[test]
    fn set_frontmatter_field_no_frontmatter_content_unchanged() {
        // content without frontmatter must be returned as-is
        let content = "# Title\n\n---\nstatus: open\n---\n";
        let updated = set_frontmatter_field(content, "status", "done");
        assert_eq!(updated, content);
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
    fn make_slug_no_trailing_dash_after_truncation() {
        // 29 word chars + space + more → dash lands at position 29, truncation at 30 exposes it
        let title = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaa bb";
        let slug = make_slug(title);
        assert!(!slug.ends_with('-'), "slug must not end with dash: {slug}");
    }

    #[test]
    fn next_id_returns_one_when_empty() {
        let dir = TempDir::new().unwrap();
        assert_eq!(next_id(dir.path()).unwrap(), "1");
    }

    #[test]
    fn next_id_increments_from_existing() {
        let dir = TempDir::new().unwrap();
        std::fs::write(dir.path().join("3-foo.md"), "").unwrap();
        std::fs::write(dir.path().join("1-bar.md"), "").unwrap();
        assert_eq!(next_id(dir.path()).unwrap(), "4");
    }

    #[test]
    fn next_id_scans_done_subdir() {
        let dir = TempDir::new().unwrap();
        let done = dir.path().join("done");
        std::fs::create_dir(&done).unwrap();
        std::fs::write(done.join("10-old.md"), "").unwrap();
        assert_eq!(next_id(dir.path()).unwrap(), "11");
    }

    #[test]
    fn next_id_handles_zero_padded_legacy_files() {
        let dir = TempDir::new().unwrap();
        std::fs::write(dir.path().join("00003-foo.md"), "").unwrap();
        std::fs::write(dir.path().join("5-bar.md"), "").unwrap();
        assert_eq!(next_id(dir.path()).unwrap(), "6");
    }

    #[test]
    fn all_issues_unknown_fallback_uses_body_for_title() {
        let dir = TempDir::new().unwrap();
        // Invalid YAML (not: a valid mapping) but with a frontmatter block and H1 in body.
        std::fs::write(
            dir.path().join("5-bad.md"),
            "---\nnot valid yaml: [\n---\n\n# My Title\n",
        )
        .unwrap();
        let issues = all_issues(dir.path(), None, None, None, None).unwrap();
        let issue = issues.iter().find(|i| i.id == "5").unwrap();
        assert_eq!(issue.status, Status::Unknown);
        assert_eq!(issue.title, "My Title");
    }

    #[test]
    fn all_issues_unknown_fallback_uses_stem_when_no_title() {
        let dir = TempDir::new().unwrap();
        std::fs::write(
            dir.path().join("6-no-title.md"),
            "---\nnot valid yaml: [\n---\n\nno h1 here\n",
        )
        .unwrap();
        let issues = all_issues(dir.path(), None, None, None, None).unwrap();
        let issue = issues.iter().find(|i| i.id == "6").unwrap();
        assert_eq!(issue.title, "6-no-title");
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
