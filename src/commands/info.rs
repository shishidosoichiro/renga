//! `renga info` command handler.

use anyhow::Result;

use crate::Context;

/// Run the info command.
pub fn run(ctx: &Context) -> Result<()> {
    let config_path = ctx.project_root.join(".renga.yml");
    let config_status = if config_path.exists() {
        config_path.display().to_string()
    } else {
        format!("{} (not found — using defaults)", config_path.display())
    };

    let area_order = if ctx.config.area_order.is_empty() {
        "(none)".to_string()
    } else {
        ctx.config.area_order.join(", ")
    };

    let area_labels = if ctx.config.area_labels.is_empty() {
        "(none)".to_string()
    } else {
        let mut pairs: Vec<_> = ctx.config.area_labels.iter().collect();
        pairs.sort_by_key(|(k, _)| k.as_str());
        pairs
            .iter()
            .map(|(k, v)| format!("{k} → \"{v}\""))
            .collect::<Vec<_>>()
            .join(", ")
    };

    println!("project root:  {}", ctx.project_root.display());
    println!("issues dir:    {}", ctx.issues_dir.display());
    println!("config file:   {config_status}");
    println!();
    println!("configuration:");
    println!(
        "  issues_dir    {:<20} # path to the issues directory",
        ctx.config.issues_dir
    );
    println!(
        "  area_order    {:<20} # ordered area list for README grouping",
        area_order
    );
    println!(
        "  area_labels   {:<20} # display labels per area (e.g. core → \"Core\")",
        area_labels
    );
    let group_by = if ctx.config.group_by.is_empty() {
        "(none)".to_string()
    } else {
        ctx.config.group_by.join(", ")
    };
    println!(
        "  group_by      {:<20} # extra directory level(s) above status (e.g. area)",
        group_by
    );

    Ok(())
}
