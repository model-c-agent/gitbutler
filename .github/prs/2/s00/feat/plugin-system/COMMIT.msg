feat: add cargo-style external plugin system for but CLI

Adds external subcommand discovery to `but`. When `but <name>` is invoked
and `<name>` is not a known subcommand or alias, the CLI now searches
PATH for `but-<name>` and spawns it as a child process, forwarding
arguments and propagating the exit code.

Resolution order for `but <name> args...`:
  1. Known clap subcommand -> dispatch normally
  2. Git config alias (but.alias.<name>) -> expand and re-parse
  3. Default alias (st, stf) -> expand and re-parse
  4. but-<name> on PATH -> spawn external plugin (NEW)
  5. None -> clap error

Also adds `but plugin list` (shows all but-* executables on PATH) and
`but plugin path` (shows directories searched for plugins).

Environment variables passed to plugins:
  - BUT_WORKSPACE_DIR (from -C flag or cwd)
  - BUT_OUTPUT_FORMAT (from --format flag)
  - BUT_JSON ("1" if --json passed)

Plugin execution is gated behind `#[cfg(not(feature = "wasi"))]` since
WASI does not support process spawning.

Affected files:
  - crates/but/src/alias.rs (plugin discovery functions)
  - crates/but/src/lib.rs (try_exec_plugin + Plugin match arm)
  - crates/but/src/args/mod.rs (Plugin variant in Subcommands)
  - crates/but/src/args/plugin.rs (new: args definitions)
  - crates/but/src/command/plugin.rs (new: list/path commands)
  - crates/but/src/command/mod.rs (register plugin module)
  - crates/but/src/command/help.rs (Plugins group in help)
  - crates/but/src/args/metrics.rs (PluginList/PluginPath)
  - crates/but/src/utils/metrics.rs (Plugin metrics mapping)
