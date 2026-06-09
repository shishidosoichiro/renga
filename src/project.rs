//! Project root discovery by walking up the filesystem.

use std::path::{Path, PathBuf};

/// Walk up from the current directory to find the project root and issues directory.
///
/// Search priority:
/// 1. `.renga.yml` present → use its `issues_dir` key (default: `"issues"`)
/// 2. `issues/` directory present
///
/// Falls back to `$CWD/issues` if nothing is found upstream.
pub fn find_project_root() -> (PathBuf, PathBuf) {
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    find_project_root_from(&cwd)
}

fn find_project_root_from(start: &Path) -> (PathBuf, PathBuf) {
    for dir in start.ancestors() {
        let renga_yml = dir.join(".renga.yml");
        if renga_yml.exists() {
            let rel = read_issues_dir_from_yml(&renga_yml);
            let issues_dir = PathBuf::from(&rel);
            let issues_dir = if issues_dir.is_absolute() {
                issues_dir
            } else {
                dir.join(issues_dir)
            };
            return (dir.to_path_buf(), issues_dir);
        }
        let issues = dir.join("issues");
        if issues.is_dir() {
            return (dir.to_path_buf(), issues);
        }
    }
    let issues = start.join("issues");
    (start.to_path_buf(), issues)
}

/// Read the `issues_dir` value from a `.renga.yml` file without a full YAML parse.
fn read_issues_dir_from_yml(path: &Path) -> String {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return "issues".to_string(),
    };
    for line in content.lines() {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix("issues_dir") {
            let rest = rest.trim_start();
            if let Some(rest) = rest.strip_prefix(':') {
                let val = rest.trim().trim_matches('"').trim_matches('\'');
                if !val.is_empty() && !val.starts_with('#') {
                    return val.to_string();
                }
            }
        }
    }
    "issues".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn finds_root_by_issues_dir() {
        let dir = TempDir::new().unwrap();
        fs::create_dir(dir.path().join("issues")).unwrap();
        // canonicalize to resolve macOS /var -> /private/var symlink
        let canonical = dir.path().canonicalize().unwrap();

        let (root, issues) = find_project_root_from(&canonical);

        assert_eq!(root, canonical);
        assert_eq!(issues, canonical.join("issues"));
    }

    #[test]
    fn finds_root_by_renga_yml() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join(".renga.yml"), "issues_dir: my-issues\n").unwrap();
        let canonical = dir.path().canonicalize().unwrap();

        let (root, issues) = find_project_root_from(&canonical);

        assert_eq!(root, canonical);
        assert_eq!(issues, canonical.join("my-issues"));
    }

    #[test]
    fn finds_root_from_subdirectory() {
        let dir = TempDir::new().unwrap();
        fs::create_dir(dir.path().join("issues")).unwrap();
        let subdir = dir.path().join("docs").join("adr");
        fs::create_dir_all(&subdir).unwrap();
        let canonical_root = dir.path().canonicalize().unwrap();

        let (root, issues) = find_project_root_from(&subdir.canonicalize().unwrap());

        assert_eq!(root, canonical_root);
        assert_eq!(issues, canonical_root.join("issues"));
    }
}
