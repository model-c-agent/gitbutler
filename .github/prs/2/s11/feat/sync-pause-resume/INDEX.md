# s11: Add `but sync pause` / `but sync resume`

## Scope
Add subcommands to temporarily pause and resume GitButler's background sync. Uses a marker file with auto-expiry (1 hour).

## Dependencies
None (Tier 0)

## Architecture Summary

Background sync in `but` works as follows:
1. Commands that set `BackgroundSync::Enabled` in `InitCtxOptions` (e.g. `status`, `diff`, `branch`) trigger `setup::init_ctx()`.
2. `init_ctx()` calls `determine_sync_operations()` which checks the fetch interval and lock availability.
3. If work is needed, `spawn_background_sync()` spawns a detached child process: `but refresh-remote-data --fetch --pr --ci --updates`.
4. The child process runs `command::legacy::refresh::handle()` which does the actual fetch/PR/CI/update work.

The pause marker file will be checked in **two places**:
- In `setup::init_ctx()` right before `determine_sync_operations()` — to prevent spawning background sync.
- In `command::legacy::refresh::handle()` at the top — as a safety net, since `refresh-remote-data` is a separate process.

## Marker Design
- **Path**: `<gitdir>/gitbutler/sync-paused` (inside `.git/gitbutler/` to keep it alongside other GitButler metadata)
- **Format**: JSON for easy parsing:
  ```json
  {
    "paused_at": 1710700000,
    "expires_at": 1710703600
  }
  ```
  Both are Unix timestamps (seconds since epoch).
- Auto-expiry: `but sync pause` writes the marker with `expires_at = now + duration` (default 3600s / 1 hour).
- `but sync resume` removes the marker file.
- `but sync status` reads the marker and reports paused/active state and time remaining.
- The `is_sync_paused()` check: if marker file exists, parse it; if `expires_at <= now`, treat as expired (not paused), optionally delete the stale marker.

## Files to Modify

### 1. `crates/but/src/args/mod.rs` — Add `Sync` variant (line ~1079, before `EvalHook`)

Add at line 1073 (after `Onboarding` variant, before `EvalHook`):
```rust
    /// Manage background sync behavior.
    ///
    /// Pause, resume, or check the status of GitButler's background sync.
    /// When paused, GitButler will not automatically fetch from remotes,
    /// refresh pull request data, or check CI status.
    ///
    /// Pause auto-expires after 1 hour by default to prevent forgotten pauses.
    ///
    /// ## Examples
    ///
    /// Pause background sync for the default duration (1 hour):
    ///
    /// ```text
    /// but sync pause
    /// ```
    ///
    /// Pause for a specific duration:
    ///
    /// ```text
    /// but sync pause --duration 30m
    /// but sync pause --duration 2h
    /// ```
    ///
    /// Resume background sync:
    ///
    /// ```text
    /// but sync resume
    /// ```
    ///
    /// Check sync status:
    ///
    /// ```text
    /// but sync status
    /// ```
    ///
    #[clap(verbatim_doc_comment)]
    Sync(sync::Platform),
```

Add to the module declarations (after line 1088, near other `pub mod` declarations):
```rust
pub mod sync;
```

### 2. `crates/but/src/args/sync.rs` — New file

```rust
/// Arguments for sync management commands.
#[derive(Debug, clap::Parser)]
pub struct Platform {
    #[clap(subcommand)]
    pub cmd: Subcommands,
}

#[derive(Debug, clap::Subcommand)]
pub enum Subcommands {
    /// Pause background sync temporarily.
    ///
    /// Creates a marker file that tells GitButler to skip background sync
    /// operations (fetch, PR refresh, CI checks). The pause auto-expires
    /// after the specified duration (default: 1 hour).
    Pause {
        /// How long to pause sync. Accepts formats like "30m", "2h", "1h30m", "90m".
        /// Default: 1h
        #[clap(long, short = 'd', default_value = "1h")]
        duration: String,
    },
    /// Resume background sync.
    ///
    /// Removes the sync pause marker, allowing background sync to proceed
    /// on the next command that triggers it.
    Resume,
    /// Show whether background sync is currently paused.
    ///
    /// Displays the current sync state and, if paused, how much time
    /// remains before the pause auto-expires.
    Status,
}
```

### 3. `crates/but/src/command/sync.rs` — New file

```rust
//! Implementation of `but sync pause/resume/status` commands.
//!
//! Uses a marker file at `<gitdir>/gitbutler/sync-paused` to signal
//! that background sync should be skipped. The marker contains a JSON
//! object with `paused_at` and `expires_at` Unix timestamps.

use std::fmt::Write;
use std::path::{Path, PathBuf};

use anyhow::{Context as _, Result};

use crate::utils::OutputChannel;

/// JSON structure stored in the marker file.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct SyncPauseMarker {
    paused_at: u64,
    expires_at: u64,
}

/// Return the path to the sync-paused marker file for a given gitdir.
pub fn marker_path(gitdir: &Path) -> PathBuf {
    gitdir.join("gitbutler").join("sync-paused")
}

/// Check whether sync is currently paused (marker exists and not expired).
/// Returns `Some(marker)` if paused, `None` if not paused or expired.
/// Silently removes expired markers.
pub fn is_sync_paused(gitdir: &Path) -> Option<SyncPauseMarker> {
    let path = marker_path(gitdir);
    let content = std::fs::read_to_string(&path).ok()?;
    let marker: SyncPauseMarker = serde_json::from_str(&content).ok()?;

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .ok()?
        .as_secs();

    if marker.expires_at <= now {
        // Expired — clean up the stale marker
        let _ = std::fs::remove_file(&path);
        None
    } else {
        Some(marker)
    }
}

/// Parse a human-friendly duration string like "30m", "2h", "1h30m", "90m".
/// Returns duration in seconds.
fn parse_duration(s: &str) -> Result<u64> {
    let s = s.trim();
    let mut total_secs: u64 = 0;
    let mut current_num = String::new();

    for ch in s.chars() {
        if ch.is_ascii_digit() {
            current_num.push(ch);
        } else {
            let n: u64 = current_num
                .parse()
                .with_context(|| format!("Invalid duration: '{s}'"))?;
            current_num.clear();
            match ch {
                'h' | 'H' => total_secs += n * 3600,
                'm' | 'M' => total_secs += n * 60,
                's' | 'S' => total_secs += n,
                _ => anyhow::bail!("Unknown duration unit '{ch}' in '{s}'. Use h, m, or s."),
            }
        }
    }

    // If there's a trailing number with no unit, treat as minutes
    if !current_num.is_empty() {
        let n: u64 = current_num
            .parse()
            .with_context(|| format!("Invalid duration: '{s}'"))?;
        total_secs += n * 60;
    }

    if total_secs == 0 {
        anyhow::bail!("Duration must be greater than zero.");
    }
    Ok(total_secs)
}

/// Handle `but sync pause --duration <dur>`.
pub fn pause(gitdir: &Path, out: &mut OutputChannel, duration_str: &str) -> Result<()> {
    let duration_secs = parse_duration(duration_str)?;
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();

    let marker = SyncPauseMarker {
        paused_at: now,
        expires_at: now + duration_secs,
    };

    let path = marker_path(gitdir);
    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(&marker)?;
    std::fs::write(&path, json)?;

    let display_dur = format_duration(duration_secs);
    writeln!(out, "Background sync paused for {display_dur}.")?;
    Ok(())
}

/// Handle `but sync resume`.
pub fn resume(gitdir: &Path, out: &mut OutputChannel) -> Result<()> {
    let path = marker_path(gitdir);
    if path.exists() {
        std::fs::remove_file(&path)?;
        writeln!(out, "Background sync resumed.")?;
    } else {
        writeln!(out, "Background sync is not paused.")?;
    }
    Ok(())
}

/// Handle `but sync status`.
pub fn status(gitdir: &Path, out: &mut OutputChannel) -> Result<()> {
    match is_sync_paused(gitdir) {
        Some(marker) => {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs();
            let remaining = marker.expires_at.saturating_sub(now);
            let display_remaining = format_duration(remaining);
            writeln!(
                out,
                "Background sync is paused. Resumes in {display_remaining}."
            )?;
        }
        None => {
            writeln!(out, "Background sync is active.")?;
        }
    }
    Ok(())
}

/// Format seconds into a human-friendly string like "1h 30m" or "45m".
fn format_duration(secs: u64) -> String {
    let hours = secs / 3600;
    let minutes = (secs % 3600) / 60;
    match (hours, minutes) {
        (0, m) => format!("{m}m"),
        (h, 0) => format!("{h}h"),
        (h, m) => format!("{h}h {m}m"),
    }
}
```

### 4. `crates/but/src/command/mod.rs` — Register `pub mod sync;` (line ~18)

Add after line 18 (after `pub mod update;`):
```rust
pub mod sync;
```

### 5. `crates/but/src/lib.rs` — Add match arm (line ~1261, before closing `}` of `match_subcommand`)

Add before the `#[cfg(feature = "legacy")] Subcommands::Apply { .. }` arm at line 1247 (or just before the closing `}` at line 1262):

```rust
        Subcommands::Sync(args::sync::Platform { cmd }) => {
            let ctx = setup::init_ctx(&args, InitCtxOptions::default(), out)?;
            match cmd {
                args::sync::Subcommands::Pause { duration } => {
                    command::sync::pause(&ctx.gitdir, out, &duration).emit_metrics(metrics_ctx)
                }
                args::sync::Subcommands::Resume => {
                    command::sync::resume(&ctx.gitdir, out).emit_metrics(metrics_ctx)
                }
                args::sync::Subcommands::Status => {
                    command::sync::status(&ctx.gitdir, out).emit_metrics(metrics_ctx)
                }
            }
        }
```

Note: `Sync` does NOT need `#[cfg(feature = "legacy")]` since it only reads/writes a marker file — no legacy crate dependencies. It uses `InitCtxOptions::default()` (sync disabled) since we just need the `gitdir` path.

### 6. `crates/but/src/setup.rs` — Check marker before spawning background sync (line ~219)

In the `BackgroundSync::Enabled` arm, after the `NO_BG_TASKS` check (line 208-210) and the human-output check (line 212-215), add:

```rust
            // Check if sync is paused via marker file
            if crate::command::sync::is_sync_paused(&ctx.gitdir).is_some() {
                return Ok(ctx);
            }
```

### 7. `crates/but/src/command/legacy/refresh.rs` — Check marker at top of handle() (line ~13)

At the beginning of the `handle()` function, before the lock acquisition (line 16), add:

```rust
    // Check if sync is paused via marker file
    if crate::command::sync::is_sync_paused(&ctx.gitdir).is_some() {
        out.write_str("\nSync is paused. Skipping background refresh.")?;
        return Ok(());
    }
```

### 8. `crates/but/src/utils/metrics.rs` — Add metrics mapping (line ~206)

Before the closing `Subcommands::Onboarding | Subcommands::EvalHook => Unknown,` line, add:

```rust
            Subcommands::Sync { .. } => Unknown,
```

Note: we map to `Unknown` since this is a low-traffic utility command and doesn't need its own metric name. If desired, a `SyncPause`/`SyncResume` variant could be added to `CommandName` later.

### 9. `crates/but/src/args/mod.rs` — Import sync module

The `use args::{ ... }` import at line 32-35 of `lib.rs` needs to include `sync`:
```rust
use args::{
    Args, OutputFormat, Subcommands, actions, alias as alias_args, branch, claude, cursor, forge,
    metrics, sync as sync_args, update as update_args, worktree,
};
```

(Or we can reference it as `args::sync::Platform` directly in the match arm, which avoids import changes.)

## Line Number Reference (current file state)

| File | Line | What |
|------|------|------|
| `args/mod.rs` | 1073 | Insert `Sync(sync::Platform)` variant after `Onboarding` |
| `args/mod.rs` | 1087 | Insert `pub mod sync;` near other module declarations |
| `command/mod.rs` | 19 | Insert `pub mod sync;` after `pub mod update;` |
| `lib.rs` | 33 | Optionally add `sync` to the `use args::{ ... }` import |
| `lib.rs` | 1261 | Insert `Subcommands::Sync(..)` match arm before closing `}` |
| `setup.rs` | 215 | Insert `is_sync_paused()` check after human-output gate |
| `legacy/refresh.rs` | 13 | Insert `is_sync_paused()` check at top of `handle()` |
| `utils/metrics.rs` | 206 | Insert `Subcommands::Sync { .. } => Unknown` |

## New Files

| File | Purpose |
|------|---------|
| `crates/but/src/args/sync.rs` | Clap argument definitions for `Pause`, `Resume`, `Status` |
| `crates/but/src/command/sync.rs` | Marker file read/write/check logic, duration parsing |

## Acceptance Criteria
- `but sync pause` pauses background sync
- `but sync resume` resumes it
- `but sync status` shows current state
- Marker auto-expires after 1 hour by default
- `--duration` flag supports formats like "30m", "2h", "1h30m"
- Background sync in `setup.rs` checks marker before spawning
- `refresh-remote-data` checks marker at entry
- `cargo check -p but` and `cargo test -p but` pass

## Complexity: M
