# Questions: s00 -- Plugin System

## Q: Should `try_exec_plugin` use `exec` (replace process) or `spawn` (child process)?
**Status:** open
**Blocking:** yes
**Question:** The current plan uses `std::process::Command::status()` which spawns a child process and waits for it. An alternative is to use `std::os::unix::process::CommandExt::exec()` which replaces the current process entirely (like cargo does for `cargo-<name>`). The exec approach is simpler (no need to propagate exit codes), but loses the ability to do post-plugin cleanup. Which approach should we use?

**Response:** 2026-03-17 — Use `spawn` + `status()` + `process::exit(code)`. Reason: exec is Unix-only and we already need `#[cfg(not(feature = "wasi"))]` gating. Spawn works cross-platform and matches the existing pattern in `setup.rs`. The exit code propagation is trivial (3 lines). No post-plugin cleanup is needed today, but spawn keeps the door open.
**Source:** coordinator decision

## Q: Should the error message for unknown commands mention plugins?
**Status:** open
**Blocking:** no
**Question:** Currently, when `but <unknown>` is typed and it's not a path, the error is: `"but {name}" is not a command. Type "but --help" to see all available commands.` With plugins, this error will only trigger if no plugin is found either. Should we enhance this message to say something like `"but {name}" is not a command or plugin. Type "but plugin list" to see available plugins.`? This would happen at the `None if args.source_or_path.is_some()` match arm in lib.rs (line 173), but only if the plugin lookup already failed earlier in the flow (before clap parsing). Since plugin lookup happens pre-clap, the error message at line 173-174 would only fire for values that are valid paths that don't exist. Non-path values that aren't plugins would get a different error from clap. Need to verify this flow.

**Response:** 2026-03-17 — Skip for now. The error message change is cosmetic and the flow analysis is uncertain. The clap error for unknown subcommands will naturally be the fallback after plugin lookup fails. Ship the core plugin system first; error message polish can come later.
**Source:** coordinator decision
