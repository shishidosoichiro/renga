//! Issue file parsing, searching, and manipulation.

use std::{
    fmt,
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::{Context as _, Result};
use serde::{Deserialize, Serialize};
use walkdir::{DirEntry, WalkDir};

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
/// assert_eq!("in-progress".parse::<Status>().unwrap(), Status::InProgress);
/// assert_eq!(Status::InProgress.to_string(), "in-progress");
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
    /// The issue is actively being worked on.
    #[serde(rename = "in-progress")]
    InProgress,
    /// The issue is complete.
    Done,
    /// The status could not be determined (e.g. frontmatter is missing).
    #[serde(other)]
    Unknown,
}

impl Status {
    /// Returns all status values accepted by `list --status`.
    ///
    /// # Examples
    ///
    /// ```
    /// use renga::issue::Status;
    /// assert!(Status::all_values().contains(&Status::Done));
    /// assert!(Status::all_values().contains(&Status::Unknown));
    /// ```
    pub fn all_values() -> &'static [Status] {
        &[
            Status::Open,
            Status::Pending,
            Status::InProgress,
            Status::Done,
            Status::Unknown,
        ]
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Open => write!(f, "open"),
            Status::Pending => write!(f, "pending"),
            Status::InProgress => write!(f, "in-progress"),
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
            "in-progress" => Ok(Status::InProgress),
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
    assignee: Option<String>,
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
    /// Assignee responsible for the issue (e.g. `"alice"`, `"app-implementer"`). Optional.
    pub assignee: Option<String>,
    /// Title extracted from the first `# Heading` line in the body.
    ///
    /// The title lives only in the body — not in frontmatter.
    /// Falls back to the file stem if no heading is found.
    pub title: String,
    /// Raw file content, preserved for display and in-place updates.
    pub raw_content: String,
}

/// Result of resolving an active issue by ID.
pub struct ActiveIssuePath {
    /// Path to the issue file.
    pub path: PathBuf,
    /// Warning to display when the issue was found through a recoverable
    /// status-directory mismatch.
    pub warning: Option<StatusDirectoryMismatch>,
}

/// A recoverable mismatch between frontmatter status and the status directory.
pub struct StatusDirectoryMismatch {
    /// Numeric issue ID requested by the user.
    pub id: String,
    /// Directory where the file is currently stored.
    pub actual_dir: String,
    /// Status declared by frontmatter.
    pub frontmatter_status: Status,
}

impl StatusDirectoryMismatch {
    /// Print an actionable warning to stderr.
    pub fn warn(&self) {
        eprintln!(
            "warning: issue {} is stored in {}/ but frontmatter status is {}",
            self.id, self.actual_dir, self.frontmatter_status
        );
        eprintln!(
            "warning: run `renga validate {} --auto-correct` to fix the directory layout",
            self.id
        );
    }
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
            assignee: fm.assignee,
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
    let mut it = WalkDir::new(issues_dir).min_depth(1).into_iter();
    while let Some(entry) = it.next() {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().into_owned();

        if entry.file_type().is_dir() {
            if !is_dir_based_issue(&entry) {
                continue;
            }
            // The issue's own files are not issues — never descend, even when
            // this directory is filtered out below.
            it.skip_current_dir();
            if !include_done && status_dir_name(entry.path()).as_deref() == Some("done") {
                continue;
            }
            if id_prefix(&name).and_then(|id| id.parse::<u64>().ok()) == Some(num) {
                let readme = entry.path().join("README.md");
                if readme.exists() {
                    return Ok(Some(readme));
                }
            }
            continue;
        }

        if !entry.file_type().is_file() {
            continue;
        }
        if issue_file_id(&name).is_none() {
            continue;
        }
        if !include_done && status_dir_name(entry.path()).as_deref() == Some("done") {
            continue;
        }
        if let Some(id) = issue_file_id(&name) {
            if id.parse::<u64>().unwrap_or(0) == num {
                return Ok(Some(entry.path().to_path_buf()));
            }
        }
    }
    Ok(None)
}

/// Search for an active issue by ID.
///
/// Normal `done/` issues are excluded. If the file is misplaced under `done/`
/// but its frontmatter status is active (`open`, `pending`, or `in-progress`),
/// it is returned with a warning so callers can keep the visible active issue
/// operable while guiding the user to repair the directory layout.
pub fn find_active_issue(issues_dir: &Path, id: &str) -> Result<Option<ActiveIssuePath>> {
    if let Some(path) = find_issue(issues_dir, id, false)? {
        return Ok(Some(ActiveIssuePath {
            path,
            warning: None,
        }));
    }

    let Some(path) = find_issue(issues_dir, id, true)? else {
        return Ok(None);
    };
    let actual_dir = status_dir_name(&path);
    if actual_dir.as_deref() != Some("done") {
        return Ok(None);
    }

    let Some(frontmatter_status) = explicit_frontmatter_status(&path)? else {
        return Ok(None);
    };
    if !matches!(
        frontmatter_status,
        Status::Open | Status::Pending | Status::InProgress
    ) {
        return Ok(None);
    }

    Ok(Some(ActiveIssuePath {
        path,
        warning: Some(StatusDirectoryMismatch {
            id: id.to_string(),
            actual_dir: actual_dir.unwrap_or_else(|| "done".to_string()),
            frontmatter_status,
        }),
    }))
}

/// Search for an issue that `update`/`edit` may modify.
///
/// Extends [`find_active_issue`] by also matching a normal `done/` issue
/// (frontmatter `status: done`, correctly stored under `done/`). This lets
/// field edits (labels, assignee, milestone, body, title) remain possible on
/// closed issues without requiring `reopen` first, following GitHub/GitLab/Jira
/// convention. Status *transition* commands (`done`, `pending`, `in-progress`,
/// `reopen`) must keep calling [`find_active_issue`] directly so they stay
/// restricted to active issues.
///
/// The misplaced-file warning behavior of [`find_active_issue`] is preserved
/// unchanged; no warning is emitted for a normal done issue since its directory
/// and frontmatter status agree.
pub fn find_editable_issue(issues_dir: &Path, id: &str) -> Result<Option<ActiveIssuePath>> {
    if let Some(active) = find_active_issue(issues_dir, id)? {
        return Ok(Some(active));
    }
    let Some(path) = find_issue(issues_dir, id, true)? else {
        return Ok(None);
    };
    let Some(frontmatter_status) = explicit_frontmatter_status(&path)? else {
        return Ok(None);
    };
    if frontmatter_status != Status::Done {
        return Ok(None);
    }
    Ok(Some(ActiveIssuePath {
        path,
        warning: None,
    }))
}

/// Whether `entry` is a directory-based issue's own directory.
///
/// Callers walking the issues tree must not descend into one: its contents
/// belong to the issue, so a file inside it named `N-slug.md` is an attachment,
/// not an issue of its own (see issue #241).
///
/// A `group_by` area directory can carry an ID prefix too — area `2024 Q1`
/// slugs to `2024-q1` — so the name alone cannot decide this. An area is
/// recognised by its shape instead: it sits directly under the issues root and
/// holds status subdirectories (`issues/<area>/<status>/`). Requiring both
/// keeps an issue that merely happens to hold an attachment folder named
/// `done/` from being read as an area.
///
/// Assumes `entry` comes from a walk rooted at the issues directory, so depth 1
/// means a direct child of it.
fn is_dir_based_issue(entry: &DirEntry) -> bool {
    if !entry.file_type().is_dir() {
        return false;
    }
    if id_prefix(&entry.file_name().to_string_lossy()).is_none() {
        return false;
    }
    !is_area_dir(entry)
}

/// Whether `entry` is a `group_by` area directory rather than an issue.
fn is_area_dir(entry: &DirEntry) -> bool {
    entry.depth() == 1
        && Status::all_values()
            .iter()
            .any(|status| entry.path().join(status.to_string()).is_dir())
}

/// Recursively collect the primary file path of every issue under `issues_dir`
/// (flat `N-slug.md` files and `N-slug/README.md` for directory-based issues),
/// regardless of nesting depth above the status directory.
pub(crate) fn collect_issue_files(issues_dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut it = WalkDir::new(issues_dir).min_depth(1).into_iter();
    while let Some(entry) = it.next() {
        let Ok(entry) = entry else { continue };
        if is_dir_based_issue(&entry) {
            let readme = entry.path().join("README.md");
            if readme.exists() {
                files.push(readme);
            }
            it.skip_current_dir();
            continue;
        }
        if entry.file_type().is_file() && is_issue_file_name(&entry.file_name().to_string_lossy()) {
            files.push(entry.path().to_path_buf());
        }
    }
    files
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
    assignee_filter: Option<&str>,
) -> Result<Vec<Issue>> {
    let mut files = collect_issue_files(issues_dir);

    files.sort_by_key(|p| extract_id(p).parse::<u64>().unwrap_or(0));

    let mut results = Vec::new();
    for path in files {
        // A file that cannot be read still exists, so keep going rather than
        // aborting the whole listing: it falls through to the unknown-status
        // fallback below. Warn, because an unknown-status issue drops out of
        // the default `list` filter and would otherwise disappear without a
        // word. `renga validate` reports the I/O error in full.
        let content = match std::fs::read_to_string(&path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("warning: cannot read {}: {e}", path.display());
                String::new()
            }
        };
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
                    assignee: None,
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
        if let Some(assignee) = assignee_filter {
            if issue.assignee.as_deref() != Some(assignee) {
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

    let mut it = WalkDir::new(issues_dir).into_iter();
    while let Some(entry) = it.next() {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().into_owned();

        if entry.file_type().is_dir() {
            // A `group_by` area directory can carry an ID prefix (area
            // "2024 Q1" -> `2024-q1/`) without being an issue, so it must not
            // reserve an ID either.
            if !is_dir_based_issue(&entry) {
                continue;
            }
            if let Some(n) = id_prefix(&name).and_then(|id| id.parse::<u64>().ok()) {
                max = max.max(n);
            }
            // Count the directory itself, then stop: attachments inside it are
            // not issues and must not reserve IDs.
            it.skip_current_dir();
            continue;
        }

        if let Some(n) = issue_file_id(&name).and_then(|id| id.parse::<u64>().ok()) {
            max = max.max(n);
        }
    }

    Ok(format!("{}", max + 1))
}

/// Maximum byte length of a slug produced by [`make_slug`] (see issue #214).
const SLUG_MAX_BYTES: usize = 80;

/// Generate a kebab-case slug from a title (max 80 bytes, see issue #214).
///
/// Unicode alphanumeric characters are preserved, so Japanese and other
/// non-ASCII titles produce meaningful slugs. The limit is measured in bytes
/// rather than characters, since a UTF-8 byte roughly tracks how much
/// information a character carries regardless of script (e.g. a Japanese
/// character is ~3 bytes against 1 for ASCII). If the cut lands in the
/// middle of a multi-byte character, it is rounded down to the previous
/// character boundary so the slug is never more than `SLUG_MAX_BYTES` and is
/// always valid UTF-8.
///
/// # Examples
///
/// ```
/// use renga::issue::make_slug;
/// assert_eq!(make_slug("Hello World"), "hello-world");
/// assert_eq!(make_slug("Rust への書き直し"), "rust-への書き直し");
/// assert_eq!(make_slug(""), "issue");
/// ```
pub fn make_slug(title: &str) -> String {
    let lower = title.to_lowercase();
    let slug = replace_non_alnum_runs(&lower);
    let slug = slug.trim_matches('-');

    let mut cut = slug.len().min(SLUG_MAX_BYTES);
    while cut > 0 && !slug.is_char_boundary(cut) {
        cut -= 1;
    }
    let slug = &slug[..cut];

    let slug = slug.trim_end_matches('-');
    if slug.is_empty() {
        "issue".to_string()
    } else {
        slug.to_string()
    }
}

/// Validate a label string for use in YAML flow sequences.
///
/// Labels are stored as unquoted scalars inside a YAML inline sequence
/// (`labels: [bug, urgent]`). Characters that act as YAML flow-sequence
/// delimiters (`,`, `[`, `]`, `{`, `}`) would silently corrupt the sequence
/// if left unquoted, so they are rejected here.
///
/// # Errors
///
/// Returns an error when `label` contains `,`, `[`, `]`, `{`, or `}`.
///
/// # Examples
///
/// ```
/// use renga::issue::validate_label;
/// assert!(validate_label("bug").is_ok());
/// assert!(validate_label("bug, urgent").is_err());
/// assert!(validate_label("[invalid]").is_err());
/// ```
pub fn validate_label(label: &str) -> Result<()> {
    for ch in [',', '[', ']', '{', '}'] {
        if label.contains(ch) {
            anyhow::bail!(
                "label '{}' contains invalid character '{}'; use a separate --label flag for each label",
                label,
                ch
            );
        }
    }
    Ok(())
}

/// Reject an `area` value that would collide with a reserved status
/// directory name once nested under `group_by`.
///
/// An area whose slug merely *looks* like an issue ID (`2024 Q1` ->
/// `2024-q1`) is fine: an area directory is told from an issue directory by
/// shape — a direct child of the issues root holding status subdirectories —
/// not by name.
///
/// A no-op when `group_by` is empty or `area` is empty — the collision only
/// matters once the area is actually used as a directory segment.
///
/// # Examples
///
/// ```
/// use renga::issue::validate_area_for_group_by;
///
/// assert!(validate_area_for_group_by("core", &["area".to_string()]).is_ok());
/// assert!(validate_area_for_group_by("2024 Q1", &["area".to_string()]).is_ok());
/// assert!(validate_area_for_group_by("done", &["area".to_string()]).is_err());
/// assert!(validate_area_for_group_by("done", &[]).is_ok());
/// ```
pub fn validate_area_for_group_by(area: &str, group_by: &[String]) -> Result<()> {
    if group_by.is_empty() || area.is_empty() {
        return Ok(());
    }
    let slug = make_slug(area);
    if Status::all_values().iter().any(|s| s.to_string() == slug) {
        anyhow::bail!(
            "area '{area}' is not allowed: its slug '{slug}' collides with a reserved status directory name"
        );
    }
    Ok(())
}

/// Replace the first `# Heading` line in `body` with `# {title}`.
///
/// If no heading exists, prepends `# {title}\n\n` to the body.
///
/// # Examples
///
/// ```
/// use renga::issue::replace_or_prepend_heading;
/// assert_eq!(replace_or_prepend_heading("# Old\n\nbody\n", "New"), "# New\n\nbody\n");
/// assert_eq!(replace_or_prepend_heading("no heading\n", "Title"), "# Title\n\nno heading\n");
/// ```
pub fn replace_or_prepend_heading(body: &str, title: &str) -> String {
    let heading = format!("# {title}");
    let mut replaced = false;
    let mut out = String::with_capacity(body.len() + heading.len());
    for line in body.lines() {
        if !replaced && line.starts_with("# ") {
            out.push_str(&heading);
            replaced = true;
        } else {
            out.push_str(line);
        }
        out.push('\n');
    }
    if !replaced {
        format!("{heading}\n\n{body}")
    } else {
        out
    }
}

/// Update a single frontmatter field in raw file content without re-serialising.
///
/// Leaves all other lines unchanged. If the field is not found in frontmatter,
/// inserts it before the closing frontmatter fence.
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
                if !found {
                    out.push(format!("{field}: {value}"));
                    found = true;
                }
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

/// Remove a single frontmatter field from raw file content without re-serialising.
///
/// Leaves all other lines unchanged. If the field is not present, the content is
/// returned unchanged.
///
/// # Examples
///
/// ```
/// use renga::issue::remove_frontmatter_field;
/// let content = "---\nstatus: open\nmilestone: v1\n---\n\n# Title\n";
/// let updated = remove_frontmatter_field(content, "milestone");
/// assert!(!updated.contains("milestone:"));
/// assert!(updated.contains("status: open"));
/// ```
pub fn remove_frontmatter_field(content: &str, field: &str) -> String {
    let prefix = format!("{field}:");
    let mut in_fm = false;
    let mut fm_closed = false;
    let mut out: Vec<String> = Vec::new();

    for line in content.lines() {
        if !fm_closed && line.trim() == "---" {
            if in_fm {
                in_fm = false;
                fm_closed = true;
            } else if out.is_empty() {
                in_fm = true;
            }
            out.push(line.to_string());
            continue;
        }
        if in_fm && line.starts_with(&prefix) {
            continue; // drop this line
        }
        out.push(line.to_string());
    }

    let mut result = out.join("\n");
    if content.ends_with('\n') {
        result.push('\n');
    }
    result
}

pub(crate) fn split_frontmatter(content: &str) -> Option<(&str, &str)> {
    let rest = content.strip_prefix("---\n")?;
    if let Some(pos) = rest.find("\n---\n") {
        Some((&rest[..pos], &rest[pos + 5..]))
    } else if let Some(pos) = rest.find("\n---") {
        Some((&rest[..pos], ""))
    } else {
        None
    }
}

/// Returns `true` if the issue is stored as `N-title/README.md`.
pub fn is_dir_based(path: &Path) -> bool {
    path.file_name().map(|n| n == "README.md").unwrap_or(false)
}

/// Returns the filesystem entry that represents this issue.
///
/// For directory-based issues (`N-title/README.md`) this is the parent
/// directory. For flat-file issues this is the file itself. Use this when
/// you need the path to rename or remove the issue as a whole.
pub fn issue_root(path: &Path) -> &Path {
    if is_dir_based(path) {
        path.parent().unwrap_or(path)
    } else {
        path
    }
}

/// Move an issue to `dest_dir`, writing `content` first.
///
/// Handles both directory-based issues (the whole `N-title/` directory is
/// renamed atomically) and flat-file issues (content is written to a
/// temporary file inside `dest_dir` and renamed into place, so the issue is
/// never briefly missing from disk). If the issue is already located at
/// `dest_dir`, no rename occurs — only the content is (re)written in place.
///
/// Returns the path to the issue's primary file after the operation
/// (`README.md` for directory-based issues, the file itself otherwise).
pub fn relocate_issue(path: &Path, content: &str, dest_dir: &Path) -> Result<PathBuf> {
    std::fs::create_dir_all(dest_dir)?;
    let entry_name = issue_root(path)
        .file_name()
        .with_context(|| format!("invalid path: {}", path.display()))?;

    if is_dir_based(path) {
        let src_root = issue_root(path);
        let dst_root = dest_dir.join(entry_name);
        std::fs::write(path, content)?;
        if src_root != dst_root {
            std::fs::rename(src_root, &dst_root)?;
        }
        Ok(dst_root.join("README.md"))
    } else {
        let dest = dest_dir.join(entry_name);
        if dest == path {
            std::fs::write(&dest, content)?;
        } else {
            let tmp = dest.with_extension("tmp");
            std::fs::write(&tmp, content)?;
            if let Err(e) = std::fs::rename(&tmp, &dest) {
                let _ = std::fs::remove_file(&tmp);
                return Err(e.into());
            }
            std::fs::remove_file(path)?;
        }
        Ok(dest)
    }
}

/// Convert a flat-file issue (`N-title.md`) into a directory-based issue
/// (`N-title/README.md`), in place — the containing directory is unchanged.
///
/// This is the pure filesystem mechanics shared by `update --dir=true`
/// and `migrate`'s `defaults.dir` step. Callers own README regeneration and
/// any user-facing printing.
///
/// # Errors
///
/// Returns an error if `path` is already directory-based, or if a file or
/// directory already occupies the destination.
pub(crate) fn convert_flat_to_dir(path: &Path) -> Result<PathBuf> {
    if is_dir_based(path) {
        anyhow::bail!("issue is already a directory: {}", path.display());
    }
    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .with_context(|| format!("invalid path: {}", path.display()))?;
    let parent = path
        .parent()
        .with_context(|| format!("invalid path: {}", path.display()))?;
    let dir = parent.join(stem);
    std::fs::create_dir(&dir).with_context(|| format!("failed to create {}", dir.display()))?;
    let readme = dir.join("README.md");
    std::fs::rename(path, &readme)
        .with_context(|| format!("failed to move {} to {}", path.display(), readme.display()))?;
    Ok(readme)
}

/// Compute the canonical directory for an issue's status file, honoring the
/// `group_by` project config.
///
/// When `group_by` is non-empty and `area` is non-empty, the area is
/// slugified (via [`make_slug`]) and nested above the status directory:
/// `<issues_dir>/<area-slug>/<status>`. Otherwise (no `group_by`, or an
/// issue with no `area`), this is `<issues_dir>/<status>`, matching the
/// classic flat layout.
///
/// # Examples
///
/// ```
/// use renga::issue::canonical_status_dir;
/// use std::path::Path;
///
/// let issues_dir = Path::new("issues");
/// assert_eq!(
///     canonical_status_dir(issues_dir, &[], "core", "open"),
///     issues_dir.join("open")
/// );
/// assert_eq!(
///     canonical_status_dir(issues_dir, &["area".to_string()], "core", "open"),
///     issues_dir.join("core").join("open")
/// );
/// assert_eq!(
///     canonical_status_dir(issues_dir, &["area".to_string()], "", "open"),
///     issues_dir.join("open")
/// );
/// // An area colliding with a reserved status name falls back to flat.
/// assert_eq!(
///     canonical_status_dir(issues_dir, &["area".to_string()], "done", "open"),
///     issues_dir.join("open")
/// );
/// // An area that merely looks like an issue ID is nested normally.
/// assert_eq!(
///     canonical_status_dir(issues_dir, &["area".to_string()], "2024 Q1", "open"),
///     issues_dir.join("2024-q1").join("open")
/// );
/// ```
pub fn canonical_status_dir(
    issues_dir: &Path,
    group_by: &[String],
    area: &str,
    status: &str,
) -> PathBuf {
    // An area rejected by `validate_area_for_group_by` is treated like no area
    // at all, the same way an unparseable frontmatter is: renga must never
    // build the very directory layout its own `create` refuses to produce.
    // `validate` still reports the area itself as an error.
    if !group_by.is_empty()
        && !area.is_empty()
        && validate_area_for_group_by(area, group_by).is_ok()
    {
        issues_dir.join(make_slug(area)).join(status)
    } else {
        issues_dir.join(status)
    }
}

/// Extract the numeric ID string from an issue's filename or directory name.
pub(crate) fn extract_id(path: &Path) -> String {
    if is_dir_based(path) {
        return path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .and_then(id_prefix)
            .map(str::to_string)
            .unwrap_or_default();
    }
    path.file_stem()
        .and_then(|s| s.to_str())
        .and_then(id_prefix)
        .map(str::to_string)
        .unwrap_or_default()
}

fn extract_title(body: &str) -> String {
    // Strip legacy "NNN: " numeric prefix if present
    for line in body.lines() {
        if let Some(rest) = line.strip_prefix("# ") {
            return strip_linenum_prefix(rest.trim()).to_string();
        }
    }
    String::new()
}

pub(crate) fn is_issue_file_name(name: &str) -> bool {
    issue_file_id(name).is_some()
}

pub(crate) fn issue_file_id(name: &str) -> Option<&str> {
    let stem = name.strip_suffix(".md")?;
    id_prefix(stem)
}

pub(crate) fn id_prefix(s: &str) -> Option<&str> {
    let (id, _) = s.split_once('-')?;
    if id.is_empty() || !id.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    Some(id)
}

fn replace_non_alnum_runs(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut last_was_separator = false;
    for ch in s.chars() {
        if ch.is_alphanumeric() {
            out.push(ch);
            last_was_separator = false;
        } else if !last_was_separator {
            out.push('-');
            last_was_separator = true;
        }
    }
    out
}

fn strip_linenum_prefix(s: &str) -> &str {
    let Some((prefix, rest)) = s.split_once(':') else {
        return s;
    };
    if prefix.is_empty() || !prefix.bytes().all(|b| b.is_ascii_digit()) {
        return s;
    }
    rest.trim_start()
}

/// Returns the name of the directory that directly contains this issue.
///
/// This is the status directory (e.g. `"open"`, `"done"`) regardless of how
/// many directory levels sit above it (e.g. under a `group_by` area
/// nesting), since it is always the issue's immediate parent — `issue_root`
/// already collapses directory-based vs. flat-file issues to a single
/// filesystem entry.
pub(crate) fn status_dir_name(path: &Path) -> Option<String> {
    issue_root(path)
        .parent()?
        .file_name()?
        .to_str()
        .map(str::to_string)
}

fn explicit_frontmatter_status(path: &Path) -> Result<Option<Status>> {
    let content =
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let (fm_str, _) = split_frontmatter(&content)
        .with_context(|| format!("no frontmatter in {}", path.display()))?;
    let fm: FrontmatterRaw = serde_yaml::from_str(fm_str)
        .with_context(|| format!("invalid frontmatter in {}", path.display()))?;
    Ok(fm.status)
}

fn title_or_stem(body: &str, path: &Path) -> String {
    let t = extract_title(body);
    if t.is_empty() {
        let stem_path = if is_dir_based(path) {
            path.parent().unwrap_or(path)
        } else {
            path
        };
        stem_path
            .file_stem()
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
    fn set_frontmatter_field_adds_missing_field() {
        let content = "---\nstatus: open\npriority: high\n---\n\n# Title\n";
        let updated = set_frontmatter_field(content, "milestone", "v1");
        assert!(updated.contains("status: open"));
        assert!(updated.contains("priority: high"));
        assert!(updated.contains("milestone: v1\n---"));
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
    fn remove_frontmatter_field_removes_existing_field() {
        let content = "---\nstatus: open\nmilestone: v1\narea: core\n---\n\n# Title\n";
        let updated = remove_frontmatter_field(content, "milestone");
        assert!(!updated.contains("milestone:"));
        assert!(updated.contains("status: open"));
        assert!(updated.contains("area: core"));
    }

    #[test]
    fn remove_frontmatter_field_noop_when_field_absent() {
        let content = "---\nstatus: open\narea: core\n---\n\n# Title\n";
        let updated = remove_frontmatter_field(content, "milestone");
        assert_eq!(updated, content);
    }

    #[test]
    fn remove_frontmatter_field_ignores_hr_in_body() {
        let content = "---\nstatus: open\nmilestone: v1\n---\n\n# Title\n\n---\nmilestone: fake\n";
        let updated = remove_frontmatter_field(content, "milestone");
        assert!(!updated.contains("milestone: v1"));
        assert!(updated.contains("milestone: fake"));
    }

    #[test]
    fn make_slug_converts_title() {
        assert_eq!(make_slug("Hello World"), "hello-world");
        assert_eq!(make_slug("  --  "), "issue");
    }

    #[test]
    fn make_slug_preserves_japanese() {
        assert_eq!(make_slug("Rust への書き直し"), "rust-への書き直し");
        assert_eq!(make_slug("日本語タイトル"), "日本語タイトル");
        assert_eq!(
            make_slug("ADR: 日本語タイトルの設計判断"),
            "adr-日本語タイトルの設計判断"
        );
        assert_eq!(
            make_slug("実装 (implementation) の話"),
            "実装-implementation-の話"
        );
        // 全角記号・中黒・全角スペースは区切り文字として扱われる
        assert_eq!(make_slug("foo・bar　baz"), "foo-bar-baz");
    }

    #[test]
    fn make_slug_truncates_ascii_at_80_bytes() {
        let long = "a".repeat(90);
        let slug = make_slug(&long);
        assert_eq!(slug.len(), 80);
        assert_eq!(slug, "a".repeat(80));
    }

    #[test]
    fn make_slug_truncates_japanese_by_byte_length_not_char_count() {
        // Each hiragana character is 3 bytes, so an 80-byte budget fits 26
        // full characters (78 bytes) — the 27th would need 3 more bytes and
        // is dropped whole rather than emitting a partial/invalid character.
        let long = "あ".repeat(40);
        let slug = make_slug(&long);
        assert_eq!(slug.len(), 78);
        assert_eq!(slug, "あ".repeat(26));
    }

    #[test]
    fn make_slug_rounds_down_when_cut_lands_mid_character() {
        // No separator between the 79 ASCII bytes and "あ", so the 80-byte
        // cutoff itself lands on the second byte of "あ"'s 3-byte encoding —
        // this exercises the is_char_boundary decrement directly, unlike a
        // dash-separated title where trim_end_matches('-') would do the work.
        let title = format!("{}あ", "a".repeat(79));
        let slug = make_slug(&title);
        assert_eq!(slug.len(), 79);
        assert_eq!(slug, "a".repeat(79));
    }

    #[test]
    fn make_slug_no_trailing_dash_after_truncation() {
        // 79 word bytes + the separator lands exactly on the 80-byte cutoff;
        // trimming must remove that trailing dash rather than leave it dangling.
        let title = format!("{} bb", "a".repeat(79));
        let slug = make_slug(&title);
        assert!(!slug.ends_with('-'), "slug must not end with dash: {slug}");
        assert_eq!(slug, "a".repeat(79));
    }

    #[test]
    fn issue_file_name_requires_integer_prefix_and_md_extension() {
        assert!(is_issue_file_name("1-task.md"));
        assert!(is_issue_file_name("00001-task.md"));
        assert!(!is_issue_file_name("task.md"));
        assert!(!is_issue_file_name("1-task.txt"));
        assert!(!is_issue_file_name("1task.md"));
    }

    #[test]
    fn extract_title_strips_legacy_line_number_prefix() {
        assert_eq!(extract_title("# 123:   Old title\n"), "Old title");
        assert_eq!(extract_title("# 123 Old title\n"), "123 Old title");
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
    fn all_issues_lists_an_unreadable_file_as_unknown() {
        let dir = TempDir::new().unwrap();
        std::fs::write(
            dir.path().join("1-ok.md"),
            "---\nstatus: open\n---\n\n# Ok\n",
        )
        .unwrap();
        // A directory-based issue whose README.md is itself a directory: it is
        // collected, but reading it fails. Stands in for any I/O failure — the
        // listing must survive it rather than aborting or dropping the issue.
        std::fs::create_dir_all(dir.path().join("2-broken").join("README.md")).unwrap();

        let issues = all_issues(dir.path(), None, None, None, None, None).unwrap();

        let ids: Vec<&str> = issues.iter().map(|i| i.id.as_str()).collect();
        assert_eq!(ids, vec!["1", "2"]);
        assert_eq!(issues[1].status, Status::Unknown);
    }

    #[test]
    fn next_id_handles_zero_padded_legacy_files() {
        let dir = TempDir::new().unwrap();
        std::fs::write(dir.path().join("00003-foo.md"), "").unwrap();
        std::fs::write(dir.path().join("5-bar.md"), "").unwrap();
        assert_eq!(next_id(dir.path()).unwrap(), "6");
    }

    /// Build `<dir>/open/1-task/` as a directory-based issue holding an
    /// attachment that looks like an issue file. See issue #241.
    fn dir_based_issue_with_attachment(dir: &TempDir) -> PathBuf {
        let issue_dir = dir.path().join("open").join("1-task");
        std::fs::create_dir_all(&issue_dir).unwrap();
        std::fs::write(
            issue_dir.join("README.md"),
            "---\nstatus: open\n---\n\n# Task\n",
        )
        .unwrap();
        std::fs::write(
            issue_dir.join("9-design.md"),
            "---\nstatus: open\n---\n\n# Attached design note\n",
        )
        .unwrap();
        issue_dir
    }

    #[test]
    fn collect_issue_files_ignores_files_inside_dir_based_issue() {
        let dir = TempDir::new().unwrap();
        let issue_dir = dir_based_issue_with_attachment(&dir);

        let files = collect_issue_files(dir.path());

        assert_eq!(files, vec![issue_dir.join("README.md")]);
    }

    #[test]
    fn next_id_ignores_files_inside_dir_based_issue() {
        let dir = TempDir::new().unwrap();
        dir_based_issue_with_attachment(&dir);

        // The directory itself still counts (ID 1); the 9-design.md attachment
        // inside it must not reserve ID 9.
        assert_eq!(next_id(dir.path()).unwrap(), "2");
    }

    #[test]
    fn find_issue_ignores_files_inside_dir_based_issue() {
        let dir = TempDir::new().unwrap();
        let issue_dir = dir_based_issue_with_attachment(&dir);

        assert_eq!(find_issue(dir.path(), "9", true).unwrap(), None);
        assert_eq!(
            find_issue(dir.path(), "1", true).unwrap(),
            Some(issue_dir.join("README.md"))
        );
    }

    #[test]
    fn find_issue_ignores_attachments_in_done_dir_based_issue() {
        let dir = TempDir::new().unwrap();
        let issue_dir = dir.path().join("done").join("1-task");
        std::fs::create_dir_all(&issue_dir).unwrap();
        std::fs::write(
            issue_dir.join("README.md"),
            "---\nstatus: done\n---\n\n# Task\n",
        )
        .unwrap();
        std::fs::write(
            issue_dir.join("9-design.md"),
            "---\nstatus: open\n---\n\n# Attached\n",
        )
        .unwrap();

        // The done/ directory is filtered out, but the walk must still not
        // descend into it and surface the attachment as issue 9.
        assert_eq!(find_issue(dir.path(), "9", false).unwrap(), None);
    }

    /// Build `<dir>/2024-q1/open/1-task.md` — a `group_by` area directory whose
    /// slug carries an ID prefix (area "2024 Q1"). Returns the issue path.
    fn numeric_prefixed_area_dir(dir: &TempDir) -> PathBuf {
        let area_open = dir.path().join("2024-q1").join("open");
        std::fs::create_dir_all(&area_open).unwrap();
        let path = area_open.join("1-task.md");
        std::fs::write(&path, "---\nstatus: open\narea: 2024 Q1\n---\n\n# Task\n").unwrap();
        path
    }

    #[test]
    fn collect_issue_files_descends_into_numeric_prefixed_area_dir() {
        let dir = TempDir::new().unwrap();
        let issue = numeric_prefixed_area_dir(&dir);

        assert_eq!(collect_issue_files(dir.path()), vec![issue]);
    }

    #[test]
    fn collect_issue_files_descends_into_area_dir_holding_a_readme() {
        let dir = TempDir::new().unwrap();
        let issue = numeric_prefixed_area_dir(&dir);
        // An area may carry its own README. It must not turn the area into an
        // issue and hide everything filed under it.
        std::fs::write(dir.path().join("2024-q1").join("README.md"), "# Q1 notes\n").unwrap();

        assert_eq!(collect_issue_files(dir.path()), vec![issue]);
    }

    #[test]
    fn collect_issue_files_skips_inside_dir_based_issue_missing_its_readme() {
        let dir = TempDir::new().unwrap();
        let issue_dir = dir.path().join("open").join("1-task");
        std::fs::create_dir_all(&issue_dir).unwrap();
        // No README.md: the directory represents no issue, but its contents are
        // still the issue's, not issues of their own.
        std::fs::write(
            issue_dir.join("9-design.md"),
            "---\nstatus: open\n---\n\n# Attached\n",
        )
        .unwrap();

        assert!(collect_issue_files(dir.path()).is_empty());
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
        let issues = all_issues(dir.path(), None, None, None, None, None).unwrap();
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
        let issues = all_issues(dir.path(), None, None, None, None, None).unwrap();
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

    #[test]
    fn find_editable_issue_matches_normal_done_issue() {
        let dir = TempDir::new().unwrap();
        let done = dir.path().join("done");
        std::fs::create_dir(&done).unwrap();
        std::fs::write(done.join("1-task.md"), "---\nstatus: done\n---\n\n# Task\n").unwrap();
        let found = find_editable_issue(dir.path(), "1").unwrap();
        assert!(found.is_some());
        assert!(found.unwrap().warning.is_none());
    }

    #[test]
    fn relocate_issue_flat_file_moves_across_dirs() {
        let dir = TempDir::new().unwrap();
        let open_dir = dir.path().join("open");
        std::fs::create_dir(&open_dir).unwrap();
        let path = open_dir.join("1-task.md");
        std::fs::write(&path, "---\nstatus: open\n---\n\n# Task\n").unwrap();

        let done_dir = dir.path().join("done");
        let updated = "---\nstatus: done\n---\n\n# Task\n";
        let dest = relocate_issue(&path, updated, &done_dir).unwrap();

        assert_eq!(dest, done_dir.join("1-task.md"));
        assert!(!path.exists());
        assert_eq!(std::fs::read_to_string(&dest).unwrap(), updated);
    }

    #[test]
    fn relocate_issue_flat_file_same_dir_writes_in_place() {
        let dir = TempDir::new().unwrap();
        let open_dir = dir.path().join("open");
        std::fs::create_dir(&open_dir).unwrap();
        let path = open_dir.join("1-task.md");
        std::fs::write(&path, "---\nstatus: open\n---\n\n# Task\n").unwrap();

        let updated = "---\nstatus: open\npriority: high\n---\n\n# Task\n";
        let dest = relocate_issue(&path, updated, &open_dir).unwrap();

        assert_eq!(dest, path);
        assert_eq!(std::fs::read_to_string(&path).unwrap(), updated);
        assert!(!open_dir.join("1-task.tmp").exists());
    }

    #[test]
    fn relocate_issue_dir_based_moves_whole_directory() {
        let dir = TempDir::new().unwrap();
        let open_dir = dir.path().join("open");
        let issue_dir = open_dir.join("1-task");
        std::fs::create_dir_all(&issue_dir).unwrap();
        let path = issue_dir.join("README.md");
        std::fs::write(&path, "---\nstatus: open\n---\n\n# Task\n").unwrap();
        std::fs::write(issue_dir.join("notes.md"), "sibling file").unwrap();

        let done_dir = dir.path().join("done");
        let updated = "---\nstatus: done\n---\n\n# Task\n";
        let dest = relocate_issue(&path, updated, &done_dir).unwrap();

        assert_eq!(dest, done_dir.join("1-task").join("README.md"));
        assert!(!issue_dir.exists());
        assert_eq!(std::fs::read_to_string(&dest).unwrap(), updated);
        assert_eq!(
            std::fs::read_to_string(done_dir.join("1-task").join("notes.md")).unwrap(),
            "sibling file"
        );
    }

    #[test]
    fn relocate_issue_dir_based_same_dir_no_op_rename() {
        let dir = TempDir::new().unwrap();
        let open_dir = dir.path().join("open");
        let issue_dir = open_dir.join("1-task");
        std::fs::create_dir_all(&issue_dir).unwrap();
        let path = issue_dir.join("README.md");
        std::fs::write(&path, "---\nstatus: open\n---\n\n# Task\n").unwrap();
        std::fs::write(issue_dir.join("notes.md"), "sibling file").unwrap();

        let updated = "---\nstatus: open\npriority: high\n---\n\n# Task\n";
        let dest = relocate_issue(&path, updated, &open_dir).unwrap();

        assert_eq!(dest, path);
        assert!(issue_dir.exists());
        assert_eq!(std::fs::read_to_string(&path).unwrap(), updated);
        assert_eq!(
            std::fs::read_to_string(issue_dir.join("notes.md")).unwrap(),
            "sibling file"
        );
    }

    #[test]
    fn status_dir_name_reads_immediate_parent_for_flat_file() {
        let dir = TempDir::new().unwrap();
        let open_dir = dir.path().join("open");
        std::fs::create_dir(&open_dir).unwrap();
        let path = open_dir.join("1-task.md");
        assert_eq!(status_dir_name(&path).as_deref(), Some("open"));
    }

    #[test]
    fn status_dir_name_reads_immediate_parent_for_dir_based_issue() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("done").join("1-task").join("README.md");
        assert_eq!(status_dir_name(&path).as_deref(), Some("done"));
    }

    #[test]
    fn status_dir_name_ignores_extra_nesting_above_status() {
        // Regression guard: even with directory levels above the status dir
        // (e.g. a future group_by area nesting), the immediate parent is
        // still read as the status directory name.
        let path = dir_path_buf(&["issues", "core", "done", "1-task.md"]);
        assert_eq!(status_dir_name(&path).as_deref(), Some("done"));

        let path = dir_path_buf(&["issues", "core", "done", "1-task", "README.md"]);
        assert_eq!(status_dir_name(&path).as_deref(), Some("done"));
    }

    fn dir_path_buf(components: &[&str]) -> PathBuf {
        components.iter().collect()
    }

    #[test]
    fn canonical_status_dir_flattens_slashes_in_area() {
        let issues_dir = Path::new("issues");
        let group_by = ["area".to_string()];
        assert_eq!(
            canonical_status_dir(issues_dir, &group_by, "Core/Backend", "open"),
            issues_dir.join("core-backend").join("open")
        );
    }

    #[test]
    fn canonical_status_dir_area_empty_falls_back_to_flat() {
        let issues_dir = Path::new("issues");
        let group_by = ["area".to_string()];
        assert_eq!(
            canonical_status_dir(issues_dir, &group_by, "", "open"),
            issues_dir.join("open")
        );
    }

    #[test]
    fn validate_area_for_group_by_allows_reserved_word_when_group_by_off() {
        assert!(validate_area_for_group_by("done", &[]).is_ok());
        assert!(validate_area_for_group_by("", &["area".to_string()]).is_ok());
    }

    #[test]
    fn validate_area_for_group_by_rejects_all_reserved_status_names() {
        let group_by = ["area".to_string()];
        for status in Status::all_values() {
            let name = status.to_string();
            assert!(
                validate_area_for_group_by(&name, &group_by).is_err(),
                "expected '{name}' to be rejected"
            );
        }
    }

    #[test]
    fn convert_flat_to_dir_moves_file_into_directory() {
        let dir = TempDir::new().unwrap();
        let open_dir = dir.path().join("open");
        std::fs::create_dir(&open_dir).unwrap();
        let path = open_dir.join("1-task.md");
        std::fs::write(&path, "---\nstatus: open\n---\n\n# Task\n").unwrap();

        let readme = convert_flat_to_dir(&path).unwrap();

        assert_eq!(readme, open_dir.join("1-task").join("README.md"));
        assert!(!path.exists());
        assert_eq!(
            std::fs::read_to_string(&readme).unwrap(),
            "---\nstatus: open\n---\n\n# Task\n"
        );
    }

    #[test]
    fn convert_flat_to_dir_rejects_already_dir_based() {
        let dir = TempDir::new().unwrap();
        let issue_dir = dir.path().join("open").join("1-task");
        std::fs::create_dir_all(&issue_dir).unwrap();
        let readme = issue_dir.join("README.md");
        std::fs::write(&readme, "---\nstatus: open\n---\n\n# Task\n").unwrap();

        let err = convert_flat_to_dir(&readme).unwrap_err();
        assert!(err.to_string().contains("already a directory"), "{err}");
    }

    #[test]
    fn convert_flat_to_dir_errors_on_existing_destination() {
        let dir = TempDir::new().unwrap();
        let open_dir = dir.path().join("open");
        std::fs::create_dir(&open_dir).unwrap();
        let path = open_dir.join("1-task.md");
        std::fs::write(&path, "---\nstatus: open\n---\n\n# Task\n").unwrap();
        std::fs::create_dir(open_dir.join("1-task")).unwrap();

        assert!(convert_flat_to_dir(&path).is_err());
        assert!(path.exists());
    }
}
