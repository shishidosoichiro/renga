//! `issues/README.md` generation.

use std::path::Path;

use anyhow::Result;

use crate::{
    config::Config,
    issue::{all_issues, Issue, Status},
};

/// Generate Markdown content for `issues/README.md` from a slice of issues.
pub fn generate(issues: &[Issue], config: &Config) -> String {
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
                let file = issue.path.file_name().unwrap_or_default().to_string_lossy();
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
            let file = issue.path.file_name().unwrap_or_default().to_string_lossy();
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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::issue::{Priority, Status};

    fn make_issue(id: &str, area: &str, title: &str) -> Issue {
        Issue {
            schema_version: None,
            id: id.to_string(),
            path: PathBuf::from(format!("issues/{id}-{}.md", title.to_lowercase())),
            status: Status::Open,
            priority: Priority::Medium,
            area: area.to_string(),
            labels: vec![],
            milestone: None,
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
        let mut config = Config::default();
        config.area_order = vec!["core".to_string(), "cli".to_string()];
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
        let mut config = Config::default();
        config.area_order = vec!["cli".to_string()];
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
        let mut config = Config::default();
        config.area_order = vec!["core".to_string(), "cli".to_string()];
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
}

/// Regenerate and write `issues/README.md` to disk.
pub fn write_readme(issues_dir: &Path, config: &Config) -> Result<()> {
    let issues = all_issues(
        issues_dir,
        Some(&[Status::Open, Status::Pending]),
        None,
        None,
        None,
    )?;
    let content = generate(&issues, config);
    std::fs::write(issues_dir.join("README.md"), content + "\n")?;
    Ok(())
}
