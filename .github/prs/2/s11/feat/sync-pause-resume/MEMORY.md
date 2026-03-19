# Memory: s11 — Sync Pause/Resume

## Status: patch-ready

## Key Findings
- Background sync is spawned in `setup.rs` via `spawn_background_sync()` as a detached child process
- The child process runs `but refresh-remote-data --fetch --pr --ci --updates`
- The actual work happens in `command/legacy/refresh.rs` `handle()`
- Existing gate: `NO_BG_TASKS` env var skips sync entirely
- Existing gate: non-human output format skips sync
- Existing gate: fetch interval <= 0 disables sync
- Existing gate: inter-process lock prevents concurrent background refreshes
- `ctx.gitdir` provides the `.git` directory path; marker goes at `<gitdir>/gitbutler/sync-paused`
- `Sync` command does NOT need `#[cfg(feature = "legacy")]` — marker file ops have no legacy deps
- The `Sync` variant goes at line 1073 in `args/mod.rs` (after `Onboarding`, before `EvalHook`)
- Match arm goes at line 1261 in `lib.rs` (before closing `}` of `match_subcommand`)
- New module registered in `command/mod.rs` at line 19 (after `pub mod update;`)
