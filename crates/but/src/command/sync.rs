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
/// Returns `true` if paused, `false` if not paused or expired.
/// Silently removes expired markers.
pub fn is_sync_paused(gitdir: &Path) -> bool {
    let path = marker_path(gitdir);
    let content = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return false,
    };
    let marker: SyncPauseMarker = match serde_json::from_str(&content) {
        Ok(m) => m,
        Err(_) => return false,
    };

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    if marker.expires_at <= now {
        // Expired — clean up the stale marker
        let _ = std::fs::remove_file(&path);
        false
    } else {
        true
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
    let path = marker_path(gitdir);
    let content = std::fs::read_to_string(&path);
    let marker: Option<SyncPauseMarker> =
        content.ok().and_then(|c| serde_json::from_str(&c).ok());

    match marker {
        Some(m) => {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs();
            if m.expires_at <= now {
                let _ = std::fs::remove_file(&path);
                writeln!(out, "Background sync is active.")?;
            } else {
                let remaining = m.expires_at.saturating_sub(now);
                let display_remaining = format_duration(remaining);
                writeln!(
                    out,
                    "Background sync is paused. Resumes in {display_remaining}."
                )?;
            }
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
