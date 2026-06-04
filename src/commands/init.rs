//! `renga init` handler.

use std::fs;

use anyhow::Result;

use crate::Context;

/// Run `renga init`: create the issues directory structure in the current project.
pub fn run(ctx: &Context) -> Result<()> {
    if ["open", "pending", "in-progress", "done", "unknown"]
        .iter()
        .all(|s| ctx.status_dir(s).exists())
    {
        println!("{} already initialized", ctx.issues_dir.display());
        return Ok(());
    }
    for status in ["open", "pending", "in-progress", "done", "unknown"] {
        fs::create_dir_all(ctx.status_dir(status))?;
    }
    println!("Initialized {}", ctx.issues_dir.display());
    Ok(())
}
