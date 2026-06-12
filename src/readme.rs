//! `issues/README.md` generation.

use std::path::{Component, Path};

use anyhow::Result;

use crate::{
    config::Config,
    issue::{all_issues, Issue, Status},
};

/// Generate Markdown content for `issues/README.md` from a slice of issues.
pub fn generate(issues: &[Issue], config: &Config) -> String {
    generate_with_linker(issues, config, issue_file_name)
}

fn generate_with_linker<F>(issues: &[Issue], config: &Config, link_for: F) -> String
where
    F: Fn(&Issue) -> String,
{
    let mut by_area: std::collections::HashMap<&str, Vec<&Issue>> =
        std::collections::HashMap::new();
    for issue in issues {
        by_area.entry(issue.area.as_str()).or_default().push(issue);
    }

    let ordered_areas: Vec<&str> = if config.area_order.is_empty() {
        let mut areas: Vec<&str> = by_area.keys().copied().collect();
        areas.sort();
        areas
    } else {
        let mut areas: Vec<&str> = config
            .area_order
            .iter()
            .map(|s| s.as_str())
            .filter(|a| by_area.contains_key(a))
            .collect();
        let mut unknown: Vec<&str> = by_area
            .keys()
            .filter(|a| !config.area_order.iter().any(|o| o == *a))
            .copied()
            .collect();
        unknown.sort();
        areas.extend(unknown);
        areas
    };

    let mut lines = vec![
        "# Issues".to_string(),
        String::new(),
        "Open issues and decisions.".to_string(),
        String::new(),
        "Status: `open` = needs action / `pending` = blocked or deferred".to_string(),
        String::new(),
        "Closed issues are moved to `done/`.".to_string(),
        String::new(),
        "---".to_string(),
        String::new(),
    ];

    // Issues with no area are emitted first, flat (no heading).
    if let Some(no_area) = by_area.get("") {
        if !no_area.is_empty() {
            lines.push("| # | status | priority | title |".to_string());
            lines.push("|---|---|---|---|".to_string());
            for issue in no_area.iter() {
                let file = link_for(issue);
                lines.push(format!(
                    "| [{}]({}) | {} | {} | {} |",
                    issue.id, file, issue.status, issue.priority, issue.title
                ));
            }
            lines.push(String::new());
            lines.push("---".to_string());
            lines.push(String::new());
        }
    }

    for area in ordered_areas.iter().filter(|a| !a.is_empty()) {
        let area_issues = match by_area.get(area) {
            Some(v) if !v.is_empty() => v,
            _ => continue,
        };

        let display = config
            .area_labels
            .get(*area)
            .map(|s| s.as_str())
            .unwrap_or(area);
        lines.push(format!("## {display}"));
        lines.push(String::new());
        lines.push("| # | status | priority | title |".to_string());
        lines.push("|---|---|---|---|".to_string());

        for issue in area_issues.iter() {
            let file = link_for(issue);
            lines.push(format!(
                "| [{}]({}) | {} | {} | {} |",
                issue.id, file, issue.status, issue.priority, issue.title
            ));
        }

        lines.push(String::new());
        lines.push("---".to_string());
        lines.push(String::new());
    }

    lines.join("\n")
}

fn issue_file_name(issue: &Issue) -> String {
    issue
        .path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned()
}

fn issue_link_from_base(issue: &Issue, issues_dir: &Path) -> String {
    issue
        .path
        .strip_prefix(issues_dir)
        .map(markdown_path)
        .unwrap_or_else(|_| issue_file_name(issue))
}

fn markdown_path(path: &Path) -> String {
    let parts: Vec<String> = path
        .components()
        .filter_map(|component| match component {
            Component::CurDir => None,
            Component::Normal(part) => Some(part.to_string_lossy().into_owned()),
            other => Some(other.as_os_str().to_string_lossy().into_owned()),
        })
        .collect();

    if parts.is_empty() {
        path.to_string_lossy().into_owned()
    } else {
        parts.join("/")
    }
}

/// Regenerate and write `issues/README.md` to disk.
pub fn write_readme(issues_dir: &Path, config: &Config) -> Result<()> {
    let issues = all_issues(
        issues_dir,
        Some(&[Status::Open, Status::Pending]),
        None,
        None,
        None,
        None,
    )?;
    let content = generate_with_linker(&issues, config, |issue| {
        issue_link_from_base(issue, issues_dir)
    });
    std::fs::write(issues_dir.join("README.md"), content + "\n")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use super::*;
    use crate::issue::{Priority, Status};

    fn make_issue(id: &str, area: &str, title: &str) -> Issue {
        let slug = title.to_lowercase().replace(' ', "-");
        Issue {
            schema_version: None,
            id: id.to_string(),
            path: PathBuf::from(format!("issues/open/{id}-{slug}.md")),
            status: Status::Open,
            priority: Priority::Medium,
            area: area.to_string(),
            labels: vec![],
            milestone: None,
            assignee: None,
            title: title.to_string(),
            raw_content: String::new(),
        }
    }

    #[test]
    fn generate_respects_area_order() {
        let issues = vec![
            make_issue("1", "cli", "CLI Issue"),
            make_issue("2", "core", "Core Issue"),
        ];
        let config = Config {
            area_order: vec!["core".to_string(), "cli".to_string()],
            ..Config::default()
        };
        let out = generate(&issues, &config);
        let core_pos = out.find("## core").unwrap();
        let cli_pos = out.find("## cli").unwrap();
        assert!(core_pos < cli_pos, "core should appear before cli");
    }

    #[test]
    fn generate_appends_unknown_areas_after_ordered() {
        let issues = vec![
            make_issue("1", "cli", "CLI Issue"),
            make_issue("2", "misc", "Misc Issue"),
        ];
        let config = Config {
            area_order: vec!["cli".to_string()],
            ..Config::default()
        };
        let out = generate(&issues, &config);
        let cli_pos = out.find("## cli").unwrap();
        let misc_pos = out.find("## misc").unwrap();
        assert!(
            cli_pos < misc_pos,
            "ordered area should appear before unknown area"
        );
    }

    #[test]
    fn generate_skips_empty_area_in_ordered_list() {
        let issues = vec![make_issue("1", "core", "Core Issue")];
        let config = Config {
            area_order: vec!["core".to_string(), "cli".to_string()],
            ..Config::default()
        };
        let out = generate(&issues, &config);
        assert!(out.contains("## core"));
        assert!(!out.contains("## cli"), "empty area should not appear");
    }

    #[test]
    fn generate_no_area_section_emitted_before_named_areas() {
        let issues = vec![
            make_issue("1", "", "No-area Issue"),
            make_issue("2", "core", "Core Issue"),
        ];
        let out = generate(&issues, &Config::default());
        let no_area_pos = out.find("No-area Issue").unwrap();
        let core_pos = out.find("## core").unwrap();
        assert!(no_area_pos < core_pos);
    }

    #[test]
    fn generate_no_area_section_omitted_when_empty() {
        let issues = vec![make_issue("1", "core", "Core Issue")];
        let out = generate(&issues, &Config::default());
        // The no-area table header should not appear if there are no no-area issues.
        let heading_count = out.matches("| # | status").count();
        assert_eq!(heading_count, 1, "only one table expected");
    }

    #[test]
    fn generate_with_base_links_to_status_subdirectories() {
        let issues = vec![make_issue("1", "core", "Core Issue")];
        let out = generate_with_linker(&issues, &Config::default(), |issue| {
            issue_link_from_base(issue, Path::new("issues"))
        });
        assert!(out.contains("[1](open/1-core-issue.md)"));
    }

    #[test]
    fn generate_keeps_flat_layout_links_flat() {
        let mut issue = make_issue("1", "core", "Core Issue");
        issue.path = PathBuf::from("issues/1-core-issue.md");
        let out = generate(&[issue], &Config::default());
        assert!(out.contains("[1](1-core-issue.md)"));
    }

    #[test]
    fn generate_keeps_flat_links_when_base_name_is_status() {
        let mut issue = make_issue("1", "core", "Core Issue");
        issue.path = PathBuf::from("open/1-core-issue.md");
        let out = generate(&[issue], &Config::default());
        assert!(out.contains("[1](1-core-issue.md)"));
    }

    #[test]
    fn issue_link_from_base_links_to_status_subdirectories() {
        let issue = make_issue("1", "core", "Core Issue");
        let link = issue_link_from_base(&issue, Path::new("issues"));
        assert_eq!(link, "open/1-core-issue.md");
    }

    #[test]
    fn issue_link_from_base_uses_markdown_path_separators() {
        let mut issue = make_issue("1", "core", "Core Issue");
        issue.path = PathBuf::from("issues").join("open").join("1-core-issue.md");
        let link = issue_link_from_base(&issue, Path::new("issues"));
        assert_eq!(link, "open/1-core-issue.md");
    }

    #[test]
    fn issue_link_from_base_keeps_flat_links_when_base_name_is_status() {
        let mut issue = make_issue("1", "core", "Core Issue");
        issue.path = PathBuf::from("open/1-core-issue.md");
        let link = issue_link_from_base(&issue, Path::new("open"));
        assert_eq!(link, "1-core-issue.md");
    }
}
