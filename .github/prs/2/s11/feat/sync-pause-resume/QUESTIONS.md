# Questions: s11 — Sync Pause/Resume

## Resolved

1. **Where does background sync run?**
   - Answer: `setup.rs` `init_ctx()` spawns a detached child process via `spawn_background_sync()`.
   The child process calls `but refresh-remote-data --fetch --pr --ci --updates`.
   The actual work is in `command/legacy/refresh.rs` `handle()`.

2. **Is it a thread, tokio task, or separate process?**
   - Answer: Separate process. `tokio::process::Command` with `.group()` and `.kill_on_drop(false)`.
   This means the marker file check must happen in both the parent (before spawning) and the child
   (inside `refresh::handle()`) since the child is an independent process.

3. **What commands trigger background sync?**
   - Answer: Any command that passes `BackgroundSync::Enabled` to `InitCtxOptions`.
   Currently: `status`, `diff`, `show`, `branch`, `rub`, `mark`, `unmark`, `commit`, `stage`,
   `unstage`, `squash`, `move`, `pick`, `unapply`, `apply`, `resolve`.

4. **Where should the marker file live?**
   - Answer: `<gitdir>/gitbutler/sync-paused`. The `gitbutler/` subdirectory already exists
   inside `.git/` for other GitButler metadata. Using `ctx.gitdir` gives us the `.git` path.

## No Open Questions

All architecture questions have been resolved through codebase exploration.
