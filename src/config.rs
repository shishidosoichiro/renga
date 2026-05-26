//! Configuration loaded from `.fbim.yml`.

use std::{collections::HashMap, path::Path};

use anyhow::{Context as _, Result};
use serde::Deserialize;

/// Project configuration from `.fbim.yml`.
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
        }
    }
}

impl Config {
    /// Load configuration from `.fbim.yml` in the given directory.
    ///
    /// Returns a default [`Config`] if the file does not exist.
    /// Unknown keys in the file are silently ignored.
    pub fn load(project_root: &Path) -> Result<Self> {
        let path = project_root.join(".fbim.yml");
        if !path.exists() {
            return Ok(Config::default());
        }
        let content = std::fs::read_to_string(&path)?;
        let config = serde_yaml::from_str(&content)
            .with_context(|| format!("invalid YAML in {}", path.display()))?;
        Ok(config)
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
        std::fs::write(dir.path().join(".fbim.yml"), "area_order: [\n").unwrap();
        let err = Config::load(dir.path()).unwrap_err();
        assert!(err.to_string().contains(".fbim.yml"), "{err}");
    }

    #[test]
    fn loads_area_order_from_yml() {
        let dir = TempDir::new().unwrap();
        std::fs::write(
            dir.path().join(".fbim.yml"),
            "area_order: [core, cli, docs]\n",
        )
        .unwrap();
        let config = Config::load(dir.path()).unwrap();
        assert_eq!(config.area_order, ["core", "cli", "docs"]);
    }
}
