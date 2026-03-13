# s05: Gate Process Spawning for WASI Builds

| Field     | Value                                              |
|-----------|----------------------------------------------------|
| **ID**    | s05                                                |
| **Branch**| `pr1/s01.s05/feat/wasi-gate-process`               |
| **Anchor**| `pr1/s01/feat/wasi-feature-flags`                  |
| **Deps**  | s01                                                |
| **Size**  | M                                                  |
| **Commit**| `feat: gate process spawning for WASI builds`      |

## Status: implementation-complete

## Scope

All process spawning in the `but` crate must be unreachable when
`--no-default-features --features wasi` is used. This sub-PR gates every
such site behind `#[cfg(feature = "native")]` (or verifies it is already
gated by `legacy` or `tui`).

`crates/but-link` has no process spawning and requires no changes.

---

## Site-by-Site Analysis

### 1. `crates/but/src/utils/metrics.rs` — `emit_metrics` subprocess

**Lines:** 5 (`use command_group::AsyncCommandGroup`), 404–414

**What it does:** Spawns a fresh `but metrics` subprocess (via
`tokio::process::Command`) wrapped in a process group so the child outlives
the parent. This is how one-shot CLI telemetry is emitted in the background.

**Current gate:** None — `metrics.rs` is compiled unconditionally. The
`BackgroundMetrics` type is re-exported under `#[cfg(feature = "legacy")]`
in `utils/mod.rs`, but the module itself and `ResultMetricsExt::emit_metrics`
are always compiled.

**What needs to change:**

- `emit_metrics` in `ResultMetricsExt` impl spawns a subprocess only when
  there is a `OneshotMetricsContext`. The entire body of that impl block
  (lines 402–416) must be wrapped in `#[cfg(feature = "native")]`. In the
  `wasi` stub the method should just discard the context and return
  `self.map(|_| ())`.
- The `use command_group::AsyncCommandGroup` import at line 5 and the
  `BackgroundMetrics::new_in_background` function do not spawn processes
  (they use Tokio tasks, not subprocesses), but they depend on `posthog-rs`
  and `machine-uid` which must be made optional. See Cargo.toml changes.
- The `command-group` import in this file is only used inside `emit_metrics`
  and must be guarded with `#[cfg(feature = "native")]`.

**Files to edit:** `crates/but/src/utils/metrics.rs`, `crates/but/Cargo.toml`

---

### 2. `crates/but/src/setup.rs` — background sync subprocess

**Lines:** 6 (`use command_group::AsyncCommandGroup`), 323, 341–346

**What it does:** `spawn_background_sync` spawns a detached `but
refresh-remote-data` subprocess (via `tokio::process::Command` +
`.group().kill_on_drop(false)`) to do fetch/PR/CI work in the background.

**Current gate:** None — `setup.rs` is compiled unconditionally. The
`BackgroundSync::Enabled` path is reachable from non-`legacy` builds via
`match options.background_sync { BackgroundSync::Enabled => ... }`.

**What needs to change:**

- The entire `spawn_background_sync` function body, including the
  `tokio::process::Command::new` call, must be gated with
  `#[cfg(feature = "native")]`.
- The `use command_group::AsyncCommandGroup` import at the top of the file
  must be guarded with `#[cfg(feature = "native")]`.
- The call site `spawn_background_sync(args, out, last_fetch,
  sync_operations)` in `init_ctx` (line ~221) must be wrapped in
  `#[cfg(feature = "native")]`. Under `wasi` the `BackgroundSync::Enabled`
  arm should simply do nothing (or fall through to `return Ok(ctx)`).
- `std::process::Stdio::null()` on lines 341–342 is used as argument to the
  subprocess command; it will disappear once the process spawn is gated.

**Files to edit:** `crates/but/src/setup.rs`

---

### 3. `crates/but/src/utils/pager.rs` — external pager subprocess

**Lines:** 49–96 (`try_spawn_external_pager`)

**What it does:** Spawns an external `less` (or `$BUT_PAGER`) process with a
pipe for stdin so the pager can receive formatted output.

**Current gate:** The entire `pager` module is already gated:

```
// crates/but/src/utils/mod.rs, line 8–9
#[cfg(feature = "tui")]
mod pager;
```

Since `tui` is not enabled for `wasi` builds (`default = ["legacy", "tui",
"native"]`; a `wasi` build uses `--no-default-features --features wasi`),
this entire file is already excluded. **No action required.**

---

### 4. `crates/but/src/tui/get_text.rs` — two process spawn sites

#### 4a. `from_external_editor` — `gix::command::prepare(...).spawn()` (line 73–78)

**What it does:** Runs the user's `$EDITOR` / `$GIT_EDITOR` / `core.editor`
to edit a temp file.

**Current gate:** The entire `tui` module is wrapped by
`#[cfg(feature = "tui")]` in `lib.rs` (line 53: `mod tui`). Since `tui` is
not enabled under `wasi`, this file is already unreachable in `wasi` builds.
**No action required for `from_external_editor`.**

#### 4b. `get_editor_command_impl` — `std::process::Command::new(git)` (line 153)

**What it does:** Runs `git config core.editor` to read the user's editor
preference.

**Current gate:** Same as 4a — inside `#[cfg(feature = "tui")]` module.
Already excluded from `wasi` builds. **No action required.**

**Note:** `config.rs` also calls `crate::tui::get_text::get_editor_command()`
but that call site is already properly wrapped in `#[cfg(feature = "tui")]`
/ `#[cfg(not(feature = "tui"))]` (lines 274–277). **No action required.**

---

### 5. `crates/but/src/command/legacy/teardown.rs` — git subprocess (lines 140, 159)

**What it does:** Runs `git checkout` and `git symbolic-ref` to leave
GitButler mode.

**Current gate:** The entire `legacy` command module is gated by
`#[cfg(feature = "legacy")]` in `command/mod.rs` (line 2–3). The
`Subcommands::Teardown` arm in `lib.rs` is also `#[cfg(feature = "legacy")]`
(line 874). **Already fully gated. No action required.**

---

### 6. `crates/but/src/utils/mod.rs` — `std::process::exit` (line 43)

**What it does:** `ResultErrorExt::show_root_cause_error_then_exit_without_destructors`
calls `std::process::exit(code)` to terminate the process immediately after
printing an error.

**Current gate:** None.

**Assessment:** `std::process::exit` is supported under WASI (it maps to the
WASI `proc_exit` syscall). This is not "spawning a subprocess" — it is
terminating the current process. WASI runtimes support this. **No gate
required.**

---

## Cargo.toml Changes Required

### `command-group`

Currently unconditional:
```toml
command-group = { version = "5.0.1", features = ["with-tokio"] }
```

Must become optional and only activated by `native`:
```toml
command-group = { version = "5.0.1", features = ["with-tokio"], optional = true }
```

And in the `native` feature definition:
```toml
native = ["dep:command-group"]
```

### `posthog-rs`

Currently unconditional:
```toml
posthog-rs = { git = "...", rev = "..." }
```

Must become optional and only activated by `native`. The `do_capture`,
`posthog_client`, and `machine()` functions all depend on it. The
`BackgroundMetrics::new_in_background` function also uses `posthog_client`
internally. All of this belongs under `#[cfg(feature = "native")]`.

```toml
posthog-rs = { git = "...", rev = "...", optional = true }
```

And in the `native` feature:
```toml
native = ["dep:command-group", "dep:posthog-rs"]
```

### `machine-uid`

Currently unconditional:
```toml
machine-uid = "0.5.4"
```

Used only in `metrics.rs::machine()` which is only called from `do_capture`.
Must become optional and activated by `native`:
```toml
machine-uid = { version = "0.5.4", optional = true }
```

And in the `native` feature:
```toml
native = ["dep:command-group", "dep:posthog-rs", "dep:machine-uid"]
```

### Final `native` feature line in Cargo.toml

```toml
native = ["dep:command-group", "dep:posthog-rs", "dep:machine-uid"]
```

---

## Metrics Module Refactoring Strategy

Because `metrics.rs` is shared across feature sets, the cleanest approach is
to keep the WASI-safe parts (event types, `Props`, `OneshotMetricsContext`,
`BackgroundMetrics` struct, `EventKind` enum) compiled unconditionally, and
gate only the native-platform-specific implementations:

- `BackgroundMetrics::new_in_background` — uses `posthog_client` (posthog-rs).
  Wrap with `#[cfg(feature = "native")]`; provide a stub that returns a
  no-op `BackgroundMetrics` for `wasi`.
- `capture_event_blocking` — uses `posthog_client`. Gate with
  `#[cfg(feature = "native")]`; under `wasi` it should be a no-op async fn.
- `do_capture` — internal helper, fully gate with `#[cfg(feature = "native")]`.
- `posthog_client` — fully gate with `#[cfg(feature = "native")]`.
- `machine()` — uses `machine-uid`. Gate with `#[cfg(feature = "native")]`.
- `ResultMetricsExt::emit_metrics` — subprocess spawn + `command-group`. Gate
  the spawning logic with `#[cfg(feature = "native")]`; fall back to
  `self.map(|_| ())`.
- `use command_group::AsyncCommandGroup` — gate with `#[cfg(feature = "native")]`.

The `OneshotMetricsContext::new_if_enabled` method reads `AppSettings`; it
has no process dependency and can remain ungated (it will simply always return
`None` on WASI since the subprocess cannot be launched).

---

## Summary Table

| File | Site | Current Gate | Action Required |
|------|------|-------------|-----------------|
| `utils/metrics.rs:5` | `use command_group::AsyncCommandGroup` | none | Add `#[cfg(feature = "native")]` |
| `utils/metrics.rs:391–416` | `emit_metrics` subprocess spawn | none | Gate body with `#[cfg(feature = "native")]` |
| `utils/metrics.rs` | `BackgroundMetrics::new_in_background`, `do_capture`, `posthog_client`, `machine()` | none | Gate with `#[cfg(feature = "native")]`; stubs for wasi |
| `setup.rs:6` | `use command_group::AsyncCommandGroup` | none | Add `#[cfg(feature = "native")]` |
| `setup.rs:316–380` | `spawn_background_sync` function | none | Gate entire function with `#[cfg(feature = "native")]` |
| `setup.rs:220–223` | `spawn_background_sync(...)` call site | none | Wrap call with `#[cfg(feature = "native")]` |
| `utils/pager.rs:49–96` | `try_spawn_external_pager` | `#[cfg(feature = "tui")]` on `mod pager` | Already gated — no action |
| `tui/get_text.rs:73–78` | `gix::command::prepare(...).spawn()` | `#[cfg(feature = "tui")]` on `mod tui` | Already gated — no action |
| `tui/get_text.rs:153` | `std::process::Command::new(git)` | `#[cfg(feature = "tui")]` on `mod tui` | Already gated — no action |
| `command/legacy/teardown.rs:140,159` | `std::process::Command::new(git)` | `#[cfg(feature = "legacy")]` on `mod legacy` | Already gated — no action |
| `utils/mod.rs:43` | `std::process::exit` | none | No action — WASI supports `proc_exit` |
| `Cargo.toml` | `command-group` dep | unconditional | Make optional, add to `native` feature |
| `Cargo.toml` | `posthog-rs` dep | unconditional | Make optional, add to `native` feature |
| `Cargo.toml` | `machine-uid` dep | unconditional | Make optional, add to `native` feature |

---

## Acceptance Criteria

- [x] `command-group` is optional in `Cargo.toml`, activated only by `native`
- [x] `posthog-rs` is optional in `Cargo.toml`, activated only by `native`
- [x] `machine-uid` is optional in `Cargo.toml`, activated only by `native`
- [x] `native` feature declaration updated: `native = ["dep:command-group", "dep:posthog-rs", "dep:machine-uid"]`
- [x] `use command_group::AsyncCommandGroup` in `metrics.rs` gated `#[cfg(feature = "native")]`
- [x] `ResultMetricsExt::emit_metrics` subprocess spawn gated `#[cfg(feature = "native")]`; no-op stub for wasi
- [x] `BackgroundMetrics::new_in_background`, `capture_event_blocking`, `do_capture`, `posthog_client`, `machine()` gated `#[cfg(feature = "native")]`; stubs provided
- [x] `use command_group::AsyncCommandGroup` in `setup.rs` gated `#[cfg(feature = "native")]`
- [x] `spawn_background_sync` function in `setup.rs` gated `#[cfg(feature = "native")]`
- [x] Call site to `spawn_background_sync` in `init_ctx` wrapped `#[cfg(feature = "native")]`
- [x] No `std::process::Command::new` or `tokio::process::Command::new` reachable under `--no-default-features --features wasi`
- [x] No `command_group` imports reachable under `--no-default-features --features wasi`
- [x] `cargo check -p but --no-default-features --features wasi` succeeds
- [x] `cargo check -p but` succeeds with default features (native build unaffected)
- [ ] `cargo test -p but` passes (existing tests unbroken)
