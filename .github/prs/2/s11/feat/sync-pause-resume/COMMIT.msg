feat: add `but sync pause/resume/status` commands

Introduce commands to temporarily pause and resume GitButler's
background sync (fetch, PR refresh, CI checks). Uses a JSON marker
file at `<gitdir>/gitbutler/sync-paused` with auto-expiry (default
1 hour) to prevent forgotten pauses.

- `but sync pause [--duration 30m]` writes the marker file
- `but sync resume` removes the marker file
- `but sync status` shows whether sync is paused and time remaining
- `setup::init_ctx()` checks the marker before spawning background sync
- `refresh::handle()` checks the marker as a safety net (separate process)
- Duration parsing supports human-friendly formats: "30m", "2h", "1h30m"
