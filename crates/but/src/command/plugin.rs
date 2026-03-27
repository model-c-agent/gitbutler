//! Command implementation for managing `but` external plugins.
//!
//! Provides subcommands to list available external plugins (but-* executables on PATH)
//! and show the directories searched for plugins.

use anyhow::Result;
use colored::Colorize;

use crate::utils::OutputChannel;

/// List all external plugins found on PATH.
pub fn list(out: &mut OutputChannel) -> Result<()> {
    let plugins = crate::alias::list_external_subcommands();

    if plugins.is_empty() {
        if let Some(out) = out.for_human() {
            writeln!(out, "No external plugins found on PATH.")?;
            writeln!(out)?;
            writeln!(
                out,
                "Plugins are executables named {} found on your PATH.",
                "but-<name>".cyan()
            )?;
        } else if let Some(out) = out.for_json() {
            out.write_value(serde_json::json!({ "plugins": [] }))?;
        }
        return Ok(());
    }

    if let Some(out) = out.for_human() {
        writeln!(out, "{}:", "Available plugins".bold())?;
        writeln!(out)?;

        let max_name_len = plugins.iter().map(|(n, _)| n.len()).max().unwrap_or(0);

        for (name, path) in &plugins {
            writeln!(
                out,
                "  {:<width$}  {}",
                name.green(),
                path.display().to_string().dimmed(),
                width = max_name_len
            )?;
        }
    } else if let Some(out) = out.for_json() {
        let entries: Vec<serde_json::Value> = plugins
            .iter()
            .map(|(name, path)| {
                serde_json::json!({
                    "name": name,
                    "path": path.display().to_string()
                })
            })
            .collect();
        out.write_value(serde_json::json!({ "plugins": entries }))?;
    }

    Ok(())
}

/// Show PATH directories searched for plugins.
pub fn path(out: &mut OutputChannel) -> Result<()> {
    let paths: Vec<std::path::PathBuf> = std::env::var_os("PATH")
        .map(|p| std::env::split_paths(&p).collect())
        .unwrap_or_default();

    if let Some(out) = out.for_human() {
        writeln!(out, "{}:", "Plugin search directories".bold())?;
        writeln!(out)?;
        for path in &paths {
            let has_plugins = std::fs::read_dir(path)
                .map(|entries| {
                    entries
                        .flatten()
                        .any(|e| e.file_name().to_str().is_some_and(|n| n.starts_with("but-")))
                })
                .unwrap_or(false);

            if has_plugins {
                writeln!(out, "  {} {}", "*".green(), path.display())?;
            } else {
                writeln!(out, "    {}", path.display().to_string().dimmed())?;
            }
        }
        writeln!(out)?;
        writeln!(out, "  {} = contains but-* plugins", "*".green())?;
    } else if let Some(out) = out.for_json() {
        let entries: Vec<serde_json::Value> = paths
            .iter()
            .map(|p| serde_json::json!(p.display().to_string()))
            .collect();
        out.write_value(serde_json::json!({ "paths": entries }))?;
    }

    Ok(())
}
