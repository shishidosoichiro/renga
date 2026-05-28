//! `renga init` handler.

use std::fs;

use anyhow::Result;

use crate::Context;

/// Run `renga init`: create the issues directory structure in the current project.
pub fn run(ctx: &Context) -> Result<()> {
    if ctx.done_dir.exists() {
        println!("{} already initialized", ctx.issues_dir.display());
        return Ok(());
    }
    fs::create_dir_all(&ctx.done_dir)?;
    println!("Initialized {}", ctx.issues_dir.display());
    Ok(())
}
