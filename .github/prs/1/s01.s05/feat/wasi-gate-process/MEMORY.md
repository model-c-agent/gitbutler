# Memory: wasi-gate-process (s05)

## Status: plan-complete

## What Was Found (2026-03-13)

Comprehensive search of `crates/but/src/**/*.rs` and `crates/but-link/src/**/*.rs`
for all process-spawning APIs.

### Process spawn sites — need gating

| Site | Kind | Gate needed |
|------|------|-------------|
| `utils/metrics.rs:404` | `tokio::process::Command::new` (emit_metrics subprocess) | `#[cfg(feature = "native")]` |
| `utils/metrics.rs:5` | `use command_group::AsyncCommandGroup` | `#[cfg(feature = "native")]` |
| `setup.rs:323` | `tokio::process::Command::new` (background sync) | `#[cfg(feature = "native")]` |
| `setup.rs:6` | `use command_group::AsyncCommandGroup` | `#[cfg(feature = "native")]` |
| `setup.rs:341` | `std::process::Stdio::null()` (arg to subprocess) | disappears when spawn is gated |

### Process spawn sites — already gated (no action)

| Site | Gate | Reason |
|------|------|--------|
| `tui/get_text.rs:73-78` | `#[cfg(feature = "tui")]` via `mod tui` in lib.rs | tui not active in wasi builds |
| `tui/get_text.rs:153` | `#[cfg(feature = "tui")]` via `mod tui` in lib.rs | tui not active in wasi builds |
| `utils/pager.rs:55,67,82` | `#[cfg(feature = "tui")]` via `mod pager` in utils/mod.rs | tui not active in wasi builds |
| `command/legacy/teardown.rs:140,159` | `#[cfg(feature = "legacy")]` via `mod legacy` in command/mod.rs | legacy not active in wasi builds |

### `std::process::exit` — no gate needed

`utils/mod.rs:43` calls `std::process::exit`. This maps to the WASI
`proc_exit` syscall and is fully supported on WASI targets. Not a spawning
call.

### Cargo.toml deps needing `optional = true` + added to `native` feature

- `command-group = "5.0.1"` — currently unconditional
- `posthog-rs` (git dep) — currently unconditional; used in metrics.rs
- `machine-uid = "0.5.4"` — currently unconditional; used in metrics.rs::machine()

The `native` feature comment already says "native platform: git2, posthog,
machine-uid, keyring" but none of those are actually listed in the feature
deps yet. This PR adds them properly.

### but-link — clean

No process spawning found in `crates/but-link/src/**/*.rs`. No changes needed.

## Errors & Fixes

### 2026-03-12 -- Plan files committed to wrong branch
**Error:** The INDEX.md and MEMORY.md plan files were absorbed into the s04 commit (`2729b64`, "plan: wasi-gate-tui") due to hunk lock assignment. The `but` tool locked these files to the s04 commit because they were created by the same initial infrastructure commit (`ddf6dfc`). Attempts to move them via `but rub` or `but stage`/`but commit` resulted in empty commits on the s05 branch due to the hunk lock preventing content transfer.
**Fix:** The plan files are correct on disk. The s05 branch has two empty commits (`d5bea01`, `eb0277c`). The coordinator should squash the s04 commit to remove s05 files, and the s05 branch commits should be replaced with one that actually contains the plan files. Alternatively, during Phase 2 implementation, the plan commit can be amended with actual code changes which will properly include all file content.
**Why:** Race condition between multiple sub-agents staging to branches in the same stack. The hunk lock mechanism treats files created by the same parent commit as belonging together.

## Decisions

- `std::process::exit` does NOT need gating (WASI supports proc_exit syscall).
- `gix::command::prepare(...).spawn()` in `tui/get_text.rs` is already safe — the
  entire `tui` module is excluded from wasi builds via `#[cfg(feature = "tui")]`.
- `legacy/teardown.rs` subprocess calls are already safe — fully inside
  `#[cfg(feature = "legacy")]`.
- The `native` feature needs three new optional deps: `command-group`,
  `posthog-rs`, `machine-uid`.

## Implementation (2026-03-13)

### Status: implementation-complete

All changes implemented and both acceptance criteria cargo checks pass.

### Changes Made

**`crates/but/Cargo.toml`**
- `command-group`: added `optional = true`
- `posthog-rs`: added `optional = true`
- `machine-uid`: changed from bare string to table form with `optional = true`
- `native` feature updated to: `["dep:command-group", "dep:posthog-rs", "dep:machine-uid"]`

**`crates/but/src/utils/metrics.rs`**
- `use command_group::AsyncCommandGroup` gated with `#[cfg(feature = "native")]`
- `use posthog_rs::Client` gated with `#[cfg(feature = "native")]`
- `BackgroundMetrics::new_in_background` split: native impl gated, WASI stub returns `BackgroundMetrics { sender: None }`
- `capture_event_blocking` split: native impl gated, WASI stub is empty async fn
- `do_capture` gated with `#[cfg(feature = "native")]`
- `machine()` gated with `#[cfg(feature = "native")]`
- `posthog_client` gated with `#[cfg(feature = "native")]`
- `emit_metrics` subprocess spawn body gated with `#[cfg(feature = "native")]`; WASI path discards context and returns `self.map(|_| ())`

**`crates/but/src/setup.rs`**
- `use command_group::AsyncCommandGroup` gated with `#[cfg(feature = "native")]`
- `spawn_background_sync` function gated with `#[cfg(feature = "native")]`
- Call site of `spawn_background_sync` in `init_ctx` wrapped with `#[cfg(feature = "native")]`

### Acceptance Criteria Results

- `OPENSSL_NO_VENDOR=1 cargo check -p but --no-default-features --features wasi` — PASS
- `OPENSSL_NO_VENDOR=1 cargo check -p but` — PASS

### Commit

`b06097e` on `pr1/s01.s05/feat/wasi-gate-process` — "feat: gate process spawning for WASI builds"

## Blockers

None.
