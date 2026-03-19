# s00: Cargo-Style Plugin System

## Scope
Add external subcommand discovery to `but`. When `but <name>` is invoked and `<name>` is not a known subcommand or alias, search PATH for `but-<name>` and exec it. Also add `but plugin list` and `but plugin path`.

## Dependencies
None (Tier 0)

## Resolution Order
```
but <name> args...
  1. Known clap subcommand -> dispatch normally
  2. Git config alias (but.alias.<name>) -> expand and re-parse
  3. Default alias (st, stf) -> expand and re-parse
  4. but-<name> on PATH -> exec external plugin  <-- NEW
  5. None -> clap error
```

## Environment Variables Passed to Plugins
- `BUT_WORKSPACE_DIR` -- from `-C` flag or cwd
- `BUT_OUTPUT_FORMAT` -- from `--format` flag
- `BUT_JSON` -- "1" if `--json` passed

## WASI
Plugin execution gated behind `#[cfg(not(feature = "wasi"))]`.

## Acceptance Criteria
- `but plugin list` shows all `scripts/bin/but-*` tools (when on PATH)
- `but overview` works identically to `./scripts/bin/but-overview`
- Unknown plugin `but nonexistent` still produces helpful error
- Known subcommands not affected by plugin lookup
- `cargo check -p but` and `cargo test -p but` pass

## Complexity: M

---

## Implementation Plan

### Overview of the dispatch flow (current state)

In `lib.rs::handle_args()`:
1. Lines 65-69: `--version` check
2. Lines 73-79: Top-level `--help` check, calls `command::help::print_grouped()`
3. Line 82: `alias::expand_aliases(args)` -- expands git config aliases and defaults
4. Line 104: `Args::parse_from(args)` -- clap parsing happens here
5. Lines 135-216: Match on `args.cmd`:
   - `None` with `source_or_path` + `target` -> rub (line 136)
   - `None` with `source_or_path` only -> check if path, bail if not (line 163)
   - `None` with nothing -> run default alias (line 181)
   - `Some(cmd)` -> `match_subcommand()` (line 215)

**Key insight**: When `but overview` is typed and `overview` is not a known subcommand, clap parses it as the positional `source_or_path` field. The `None if args.source_or_path.is_some() && args.target.is_none()` arm at line 163 currently checks if it's a filesystem path and bails with an error if not. **This is where external plugin lookup must be inserted** -- before the path check.

### File-by-file changes

---

### 1. `crates/but/src/alias.rs` -- Plugin discovery functions

Add three new public functions after the existing code (after line 155, before `#[cfg(test)]` on line 157):

```rust
/// Search PATH for executables matching `but-<name>`.
///
/// Returns the full path to the executable if found, or `None`.
#[cfg(not(feature = "wasi"))]
pub fn find_external_subcommand(name: &str) -> Option<std::path::PathBuf> {
    let target = format!("but-{name}");
    std::env::var_os("PATH")
        .and_then(|paths| {
            std::env::split_paths(&paths).find_map(|dir| {
                let candidate = dir.join(&target);
                if candidate.is_file() && is_executable(&candidate) {
                    Some(candidate)
                } else {
                    None
                }
            })
        })
}

/// List all external subcommands found on PATH.
///
/// Returns a sorted, deduplicated list of (name, path) tuples where `name`
/// is the subcommand name (without the `but-` prefix).
#[cfg(not(feature = "wasi"))]
pub fn list_external_subcommands() -> Vec<(String, std::path::PathBuf)> {
    let mut seen = std::collections::HashSet::new();
    let mut result = Vec::new();

    if let Some(paths) = std::env::var_os("PATH") {
        for dir in std::env::split_paths(&paths) {
            if let Ok(entries) = std::fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    let file_name = entry.file_name();
                    if let Some(name) = file_name.to_str() {
                        if let Some(subcommand) = name.strip_prefix("but-") {
                            let path = entry.path();
                            if path.is_file()
                                && is_executable(&path)
                                && !is_known_subcommand(subcommand)
                                && seen.insert(subcommand.to_string())
                            {
                                result.push((subcommand.to_string(), path));
                            }
                        }
                    }
                }
            }
        }
    }

    result.sort_by(|a, b| a.0.cmp(&b.0));
    result
}

/// Checks whether a path is executable.
#[cfg(not(feature = "wasi"))]
fn is_executable(path: &std::path::Path) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        path.metadata()
            .map(|m| m.permissions().mode() & 0o111 != 0)
            .unwrap_or(false)
    }
    #[cfg(not(unix))]
    {
        // On Windows, just check if the file exists -- executable bit doesn't apply
        path.is_file()
    }
}
```

**Also add tests** inside the existing `#[cfg(test)] mod tests` block:

```rust
#[test]
#[cfg(not(feature = "wasi"))]
fn find_external_subcommand_nonexistent() {
    assert!(find_external_subcommand("nonexistent-plugin-xyz").is_none());
}

#[test]
#[cfg(not(feature = "wasi"))]
fn list_external_subcommands_returns_sorted() {
    let plugins = list_external_subcommands();
    let names: Vec<&str> = plugins.iter().map(|(n, _)| n.as_str()).collect();
    let mut sorted = names.clone();
    sorted.sort();
    assert_eq!(names, sorted);
}
```

---

### 2. `crates/but/src/lib.rs` -- Plugin exec between alias expansion and clap parsing

**Change 1**: Add plugin exec attempt in `handle_args()`.

The plugin lookup must happen **after alias expansion** (line 82) but **before `Args::parse_from`** (line 104). Specifically, after the expanded args come back from `expand_aliases`, if the first non-flag argument is NOT a known subcommand and NOT a flag, try to find it as a plugin.

Insert between lines 82 and 84 (after `let args = alias::expand_aliases(args)?;`):

```rust
    // Try external plugin before clap parsing (Step 4 in resolution order)
    #[cfg(not(feature = "wasi"))]
    {
        if let Some(plugin_result) = try_exec_plugin(&args) {
            return plugin_result;
        }
    }
```

**Change 2**: Add the `try_exec_plugin` helper function (add as a private function, e.g., after the `handle_args` function, before `match_subcommand`):

```rust
/// Attempts to find and exec an external plugin for the given args.
///
/// Returns `Some(Result)` if a plugin was found (and executed or failed),
/// or `None` if no plugin matches (caller should proceed with clap parsing).
#[cfg(not(feature = "wasi"))]
fn try_exec_plugin(args: &[OsString]) -> Option<Result<()>> {
    // Need at least "but <name>"
    if args.len() < 2 {
        return None;
    }

    let potential_name = args[1].to_str()?;

    // Skip flags
    if potential_name.starts_with('-') {
        return None;
    }

    // Skip known subcommands (they'll be handled by clap)
    if alias::is_known_subcommand(potential_name) {
        return None;
    }

    // Try to find but-<name> on PATH
    let plugin_path = alias::find_external_subcommand(potential_name)?;

    // Found a plugin -- exec it
    let plugin_args = &args[2..]; // everything after "but <name>"

    // Set environment variables for the plugin
    let current_dir = std::env::args_os()
        .collect::<Vec<_>>()
        .windows(2)
        .find_map(|w| {
            if w[0] == "-C" || w[0] == "--current-dir" {
                Some(w[1].clone())
            } else {
                None
            }
        })
        .unwrap_or_else(|| OsString::from("."));

    let format_val = std::env::args_os()
        .collect::<Vec<_>>()
        .windows(2)
        .find_map(|w| {
            if w[0] == "-f" || w[0] == "--format" {
                Some(w[1].clone())
            } else {
                None
            }
        })
        .unwrap_or_else(|| OsString::from("human"));

    let has_json = args.iter().any(|a| a == "--json" || a == "-j");

    let status = std::process::Command::new(&plugin_path)
        .args(plugin_args)
        .env("BUT_WORKSPACE_DIR", &current_dir)
        .env("BUT_OUTPUT_FORMAT", &format_val)
        .env("BUT_JSON", if has_json { "1" } else { "0" })
        .status();

    Some(match status {
        Ok(status) if status.success() => Ok(()),
        Ok(status) => {
            std::process::exit(status.code().unwrap_or(1));
        }
        Err(e) => Err(anyhow::anyhow!(
            "Failed to execute plugin '{}': {}",
            plugin_path.display(),
            e
        )),
    })
}
```

**Change 3**: In `match_subcommand()`, add the `Plugin` variant handler (inside the match, after `Subcommands::Alias(...)` block around line 286):

```rust
        #[cfg(not(feature = "wasi"))]
        Subcommands::Plugin(args::plugin::Platform { cmd }) => {
            match cmd {
                Some(args::plugin::Subcommands::List) | None => {
                    command::plugin::list(out).emit_metrics(metrics_ctx)
                }
                Some(args::plugin::Subcommands::Path) => {
                    command::plugin::path(out).emit_metrics(metrics_ctx)
                }
            }
        }
```

**Change 4**: Add `use args::plugin as plugin_args;` (not strictly needed if using fully qualified `args::plugin::Platform`). Actually, look at the existing import pattern at line 33:

```rust
use args::{
    Args, OutputFormat, Subcommands, actions, alias as alias_args, branch, claude, cursor, forge,
    metrics, update as update_args, worktree,
};
```

Add `plugin as plugin_args` to this list. But since the match uses `args::plugin::Platform` directly (following the pattern of `args::skill::Platform`), no extra import is needed.

---

### 3. `crates/but/src/args/mod.rs` -- Add Plugin variant to Subcommands enum

**Change 1**: Add the `Plugin` variant to the `Subcommands` enum (after `Alias` at line 690, before `Config` at line 692).

Insert after line 690 (`Alias(alias::Platform),`):

```rust
    /// Manage external plugins (but-* executables on PATH).
    ///
    /// Lists available external subcommands that extend `but` functionality.
    /// These are executables named `but-<name>` found on your PATH.
    ///
    /// Running `but plugin` without a subcommand lists available plugins
    /// (same as `but plugin list`).
    ///
    /// ## Examples
    ///
    /// List available plugins:
    ///
    /// ```text
    /// but plugin list
    /// ```
    ///
    /// Show PATH directories searched for plugins:
    ///
    /// ```text
    /// but plugin path
    /// ```
    ///
    #[cfg(not(feature = "wasi"))]
    #[clap(verbatim_doc_comment)]
    Plugin(plugin::Platform),
```

**Change 2**: Add `pub mod plugin;` declaration. Looking at the existing module declarations:
- Line 1082: `pub mod alias;`
- Line 1083: `pub mod commit;`
- Line 1084: `pub mod config;`
- ...

Add after `pub mod alias;` (line 1082), keeping alphabetical order:

But actually the modules aren't strictly alphabetical. Looking again:
- Line 1082: `pub mod alias;`
- Line 1083: `pub mod commit;`
- Line 1084: `pub mod config;`
- Line 1085-1086: `#[cfg(feature = "native")] pub mod link;`
- Line 1087: `pub mod skill;`
- Line 1088: `pub mod update;`

Insert after line 1084 (`pub mod config;`) -- or after `pub mod link;` at line 1086. Between `link` and `skill` (alphabetically `p` goes between `l` and `s`):

```rust
#[cfg(not(feature = "wasi"))]
pub mod plugin;
```

Insert after `pub mod link;` (line 1086), before `pub mod skill;` (line 1087).

---

### 4. `crates/but/src/args/plugin.rs` -- New file: Plugin args

Follow the pattern from `args/alias.rs`:

```rust
//! Command-line argument definitions for the `but plugin` command.

#[derive(Debug, clap::Parser)]
pub struct Platform {
    #[clap(subcommand)]
    pub cmd: Option<Subcommands>,
}

#[derive(Debug, clap::Subcommand)]
pub enum Subcommands {
    /// List all external plugins found on PATH
    List,

    /// Show PATH directories searched for plugins
    Path,
}
```

---

### 5. `crates/but/src/command/plugin.rs` -- New file: Plugin command implementations

Follow the pattern from `command/alias.rs` (human + JSON output via `OutputChannel`):

```rust
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
                        .any(|e| {
                            e.file_name()
                                .to_str()
                                .is_some_and(|n| n.starts_with("but-"))
                        })
                })
                .unwrap_or(false);

            if has_plugins {
                writeln!(out, "  {} {}", "*".green(), path.display())?;
            } else {
                writeln!(out, "    {}", path.display().to_string().dimmed())?;
            }
        }
        writeln!(out)?;
        writeln!(
            out,
            "  {} = contains but-* plugins",
            "*".green()
        )?;
    } else if let Some(out) = out.for_json() {
        let entries: Vec<serde_json::Value> = paths
            .iter()
            .map(|p| serde_json::json!(p.display().to_string()))
            .collect();
        out.write_value(serde_json::json!({ "paths": entries }))?;
    }

    Ok(())
}
```

---

### 6. `crates/but/src/command/mod.rs` -- Register plugin module

After line 17 (`pub mod skill;`), or in alphabetical order between `push` (line 15) and `skill` (line 17):

Insert after line 15 (`pub mod push;`), before `#[cfg(feature = "native")]`:

```rust
#[cfg(not(feature = "wasi"))]
pub mod plugin;
```

Current file content:
```
 1  pub mod legacy;               // #[cfg(feature = "legacy")]
 2  ...
 5  pub mod alias;
 6  // #[cfg(not(feature = "legacy"))]
 7  pub mod branch;
 8  pub mod completions;
 9  // #[cfg(feature = "native")]
10  pub mod config;
11  pub mod eval_hook;
12  pub mod gui;
13  pub mod help;
14  pub mod onboarding;
15  pub mod push;
16  // #[cfg(feature = "native")]
17  pub mod skill;
18  pub mod update;
```

Insert between line 15 (`pub mod push;`) and line 16 (`#[cfg(feature = "native")]`):

```rust
#[cfg(not(feature = "wasi"))]
pub mod plugin;
```

---

### 7. `crates/but/src/command/help.rs` -- Add Plugins section

In the `groups` array at lines 17-40, add a new group for Plugins. Insert before the closing `];` of the array, after the "Operation History" group:

After line 39 (`vec!["oplog", "undo", "restore"],`), line 40 (`)`) and before `];`:

```rust
        (
            "Plugins".yellow(),
            vec!["plugin"],
        ),
```

This will show:
```
Plugins:
  plugin       Manage external plugins (but-* executables on PATH).
```

---

### 8. `crates/but/src/args/metrics.rs` -- Add Plugin metrics

Add `PluginList` and `PluginPath` to the `CommandName` enum (after `SkillCheck` at line 59):

```rust
    PluginList,
    PluginPath,
```

---

### 9. `crates/but/src/utils/metrics.rs` -- Add Plugin metrics mapping

In `to_metrics_command()`, add a match arm for `Subcommands::Plugin` (after the `Skill` match around line 198-201):

```rust
            #[cfg(not(feature = "wasi"))]
            Subcommands::Plugin(plugin::Platform { cmd }) => match cmd {
                None | Some(plugin::Subcommands::List) => PluginList,
                Some(plugin::Subcommands::Path) => PluginPath,
            },
```

Also add `use crate::args::plugin;` in the match function's local imports (line 65):

```rust
#[cfg(not(feature = "wasi"))]
use crate::args::plugin;
```

---

## Summary of all files

| File | Action | Notes |
|------|--------|-------|
| `crates/but/src/alias.rs` | Modify | Add `find_external_subcommand()`, `list_external_subcommands()`, `is_executable()` + tests |
| `crates/but/src/lib.rs` | Modify | Add `try_exec_plugin()` between alias expansion and clap, add Plugin match in `match_subcommand()` |
| `crates/but/src/args/mod.rs` | Modify | Add `Plugin` variant to `Subcommands`, add `pub mod plugin;` |
| `crates/but/src/args/plugin.rs` | **New** | Plugin args: Platform + Subcommands (List, Path) |
| `crates/but/src/command/plugin.rs` | **New** | Plugin commands: `list()`, `path()` |
| `crates/but/src/command/mod.rs` | Modify | Add `pub mod plugin;` |
| `crates/but/src/command/help.rs` | Modify | Add "Plugins" group |
| `crates/but/src/args/metrics.rs` | Modify | Add `PluginList`, `PluginPath` to `CommandName` |
| `crates/but/src/utils/metrics.rs` | Modify | Add `Subcommands::Plugin` match arm |

## Patterns to follow

- **Alias variant**: `Alias(alias::Platform)` at args/mod.rs line 690 -- follow exactly for `Plugin(plugin::Platform)`
- **Alias match**: `Subcommands::Alias(alias_args::Platform { cmd })` at lib.rs line 264 -- follow for Plugin
- **Alias command module**: `command/alias.rs` -- follow `OutputChannel` pattern (`for_human()` / `for_json()`)
- **Skill gating**: `#[cfg(feature = "native")]` pattern used for Skill -- use `#[cfg(not(feature = "wasi"))]` for Plugin
- **Args file**: `args/skill.rs` -- follow `Platform` + `Subcommands` pattern
