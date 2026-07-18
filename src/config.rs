//! Configuration loaded from `.renga.yml`.

use std::{collections::HashMap, path::Path};

use anyhow::{Context as _, Result};
use serde::Deserialize;

/// Project configuration from `.renga.yml`.
#[derive(Debug, Deserialize)]
pub struct Config {
    /// Path to the issues directory (relative to project root).
    #[serde(default = "default_issues_dir")]
    pub issues_dir: String,
    /// Ordered list of area names for README grouping.
    #[serde(default)]
    pub area_order: Vec<String>,
    /// Display labels for each area (e.g. `core` → `"Core"`).
    #[serde(default)]
    pub area_labels: HashMap<String, String>,
    /// Directory nesting level(s) to add above the status directory (e.g.
    /// `issues/<area>/<status>/...` instead of `issues/<status>/...`).
    ///
    /// Currently only a single `"area"` element is supported. An empty list
    /// (the default) keeps the classic flat-under-status layout.
    #[serde(default)]
    pub group_by: Vec<String>,
}

fn default_issues_dir() -> String {
    "issues".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Config {
            issues_dir: "issues".to_string(),
            area_order: Vec::new(),
            area_labels: std::collections::HashMap::new(),
            group_by: Vec::new(),
        }
    }
}

impl Config {
    /// Load configuration from `.renga.yml` in the given directory.
    ///
    /// Returns a default [`Config`] if the file does not exist.
    /// Unknown keys in the file are silently ignored.
    pub fn load(project_root: &Path) -> Result<Self> {
        let path = project_root.join(".renga.yml");
        if !path.exists() {
            return Ok(Config::default());
        }
        let content = std::fs::read_to_string(&path)?;
        let config: Config = serde_yaml::from_str(&content)
            .with_context(|| format!("invalid YAML in {}", path.display()))?;
        config.validate_group_by()?;
        Ok(config)
    }

    fn validate_group_by(&self) -> Result<()> {
        match self.group_by.as_slice() {
            [] => Ok(()),
            [only] if only == "area" => Ok(()),
            _ => anyhow::bail!(
                "invalid group_by {:?}: only a single \"area\" element is supported",
                self.group_by
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn loads_default_when_no_file() {
        let dir = TempDir::new().unwrap();
        let config = Config::load(dir.path()).unwrap();
        assert_eq!(config.issues_dir, "issues");
        assert!(config.area_order.is_empty());
    }

    #[test]
    fn invalid_yaml_returns_error() {
        let dir = TempDir::new().unwrap();
        std::fs::write(dir.path().join(".renga.yml"), "area_order: [\n").unwrap();
        let err = Config::load(dir.path()).unwrap_err();
        assert!(err.to_string().contains(".renga.yml"), "{err}");
    }

    #[test]
    fn loads_area_order_from_yml() {
        let dir = TempDir::new().unwrap();
        std::fs::write(
            dir.path().join(".renga.yml"),
            "area_order: [core, cli, docs]\n",
        )
        .unwrap();
        let config = Config::load(dir.path()).unwrap();
        assert_eq!(config.area_order, ["core", "cli", "docs"]);
    }

    #[test]
    fn group_by_empty_by_default() {
        let dir = TempDir::new().unwrap();
        let config = Config::load(dir.path()).unwrap();
        assert!(config.group_by.is_empty());
    }

    #[test]
    fn loads_group_by_area_from_yml() {
        let dir = TempDir::new().unwrap();
        std::fs::write(dir.path().join(".renga.yml"), "group_by: [area]\n").unwrap();
        let config = Config::load(dir.path()).unwrap();
        assert_eq!(config.group_by, ["area"]);
    }

    #[test]
    fn group_by_rejects_multiple_elements() {
        let dir = TempDir::new().unwrap();
        std::fs::write(dir.path().join(".renga.yml"), "group_by: [area, label]\n").unwrap();
        let err = Config::load(dir.path()).unwrap_err();
        assert!(err.to_string().contains("group_by"), "{err}");
    }

    #[test]
    fn group_by_rejects_non_area_value() {
        let dir = TempDir::new().unwrap();
        std::fs::write(dir.path().join(".renga.yml"), "group_by: [label]\n").unwrap();
        let err = Config::load(dir.path()).unwrap_err();
        assert!(err.to_string().contains("group_by"), "{err}");
    }
}
