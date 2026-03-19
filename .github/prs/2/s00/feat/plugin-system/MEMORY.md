# Memory: s00 — Plugin System

## Status: patch-ready

## Decisions
### 2026-03-17 -- Plugin exec insertion point
**Context:** Need to decide where in the `handle_args()` flow to intercept unknown commands for plugin lookup.
**Decision:** Insert `try_exec_plugin()` call between alias expansion (line 82) and `Args::parse_from` (line 104) in `lib.rs`. This is before clap parsing, so unknown subcommands never reach clap's error handling. The function checks if the first non-flag arg is a known subcommand (skip) or finds `but-<name>` on PATH (exec it).
**Alternatives considered:** (1) Intercepting at the `None if source_or_path` match arm (line 163) -- rejected because by then clap has already parsed the unknown command name as a positional arg, which could conflict with rub source/target semantics. (2) Adding `External(String)` variant to Subcommands -- rejected because clap can't know which external commands exist at compile time.

### 2026-03-17 -- Feature gate choice
**Context:** Need to gate plugin execution for WASI builds.
**Decision:** Use `#[cfg(not(feature = "wasi"))]` since the `wasi` feature already exists in `Cargo.toml` at line 45 (`wasi = []`). Plugin execution uses `std::process::Command` which is not available under WASI.
**Alternatives considered:** Using `#[cfg(feature = "native")]` -- rejected because plugin discovery is lightweight and should work even without the heavy native dependencies.

### 2026-03-17 -- Plugin subcommand itself (but plugin list/path) visibility
**Context:** Should `but plugin` be a hidden command or visible?
**Decision:** Make it visible (not hidden). It appears in the "Plugins" group in help output. This follows the `Alias` precedent which is also a visible utility command.

### 2026-03-17 -- try_exec_plugin reads env from args slice, not std::env::args_os()
**Context:** The plan had `try_exec_plugin` calling `std::env::args_os().collect()` twice to find -C and -f flags. This is redundant since the function already receives the expanded args slice.
**Decision:** Use the `args` parameter directly with `.windows(2)` to find flag values. Simpler, no redundant allocation.

## Errors & Fixes
(none)

## Implementation Notes
### 2026-03-17 -- Patch produced
All 9 files covered in INDEX.patch:
- alias.rs: 3 new functions (find_external_subcommand, list_external_subcommands, is_executable) + 2 tests
- lib.rs: try_exec_plugin() inserted after alias expansion, Plugin match arm in match_subcommand()
- args/mod.rs: Plugin variant added after Alias, pub mod plugin declaration
- args/plugin.rs: new file with Platform + Subcommands
- command/plugin.rs: new file with list() and path() implementations
- command/mod.rs: pub mod plugin declaration
- command/help.rs: "Plugins" group added
- args/metrics.rs: PluginList/PluginPath variants
- utils/metrics.rs: Plugin match arm + import
