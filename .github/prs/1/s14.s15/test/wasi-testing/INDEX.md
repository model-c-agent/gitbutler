# s15: Add WASI integration test harness and smoke tests

- **Branch:** `pr1/s14.s15/test/wasi-testing`
- **Anchor:** `pr1/s13.s14/feat/wasi-runtime-host`
- **Deps:** s14 (runtime host exists — `but-wasi-host` crate with `but-wasi` binary)
- **Size:** M
- **Commit message:** `test: add WASI integration test harness and smoke tests`

## Scope

- Smoke tests: process start/exit, --help, version
- Command tests: branch, config, alias, completions
- Sandbox tests: filesystem isolation, network denial
- Comparison tests: native vs WASI JSON output parity

## Files

- `crates/but-wasi-host/tests/integration/mod.rs` (new)
- `crates/but-wasi-host/tests/integration/smoke.rs` (new)
- `crates/but-wasi-host/tests/integration/commands.rs` (new)
- `crates/but-wasi-host/tests/integration/sandbox.rs` (new)
- `crates/but-wasi-host/tests/integration/comparison.rs` (new)
- `crates/but-wasi-host/tests/fixtures/scenario/` (new — minimal git repos for WASI tests)
- `crates/but-wasi-host/Cargo.toml` (add dev-dependencies for tests)

## Acceptance Criteria

- All core commands have integration tests under `cargo test -p but-wasi-host`
- Sandbox tests verify filesystem isolation and network denial
- Comparison tests confirm JSON output parity between native and WASI
- All tests pass on `cargo test` with no network calls, no timing sensitivity
- Tests are deterministic (stable git fixtures via gix_testtools, no external git)

---

## Design

### Where Tests Live

Tests go in `crates/but-wasi-host/tests/integration/`. This is where s14 placed the
`but-wasi-host` crate. The test suite exercises the WASI module through the same public
API that `but-wasi` (the binary) exposes — using wasmtime's programmatic embedding API
directly, not by invoking the binary via `std::process::Command`.

The `but` crate's existing `crates/but/tests/wasi/` directory mentioned in INDEX.md is
NOT used — it would require cross-crate test coordination. All WASI integration tests
belong in `but-wasi-host`, which already holds the wasmtime embedding code.

### Test Infrastructure

#### 1. WasiTestFixture — the core helper struct

```rust
// crates/but-wasi-host/tests/integration/mod.rs

pub struct WasiTestFixture {
    /// Temporary directory holding a minimal git repository.
    repo_dir: tempfile::TempDir,
    /// Path to but.wasm — resolved once at test startup.
    wasm_path: PathBuf,
    /// Wasmtime Engine with cache enabled (shared across all tests in a process via once_cell).
    engine: Engine,
}
```

Key methods:
- `WasiTestFixture::new(scenario: &str) -> Self` — copies a named scenario fixture into
  a temp dir via `gix_testtools::scripted_fixture_writable_with_args`, then resolves the
  `.wasm` path.
- `WasiTestFixture::empty() -> Self` — no git repo at all; useful for smoke tests.
- `fn run(&self, args: &[&str]) -> WasiOutput` — loads and runs the WASI module with
  the repo dir as the only preopen. Captures stdout, stderr, exit code.
- `fn run_with_preopens(&self, preopens: &[(&str, &Path)], args: &[&str]) -> WasiOutput`
  — explicit preopen control for sandbox tests.

#### 2. WasiOutput — result capture

```rust
pub struct WasiOutput {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

impl WasiOutput {
    pub fn assert_success(&self) -> &Self { ... }
    pub fn assert_failure(&self) -> &Self { ... }
    pub fn stdout_json(&self) -> serde_json::Value { ... }
}
```

#### 3. .wasm artifact resolution

At test start, resolve the `.wasm` path once via `once_cell::sync::Lazy`:

```rust
static WASM_PATH: Lazy<PathBuf> = Lazy::new(|| {
    // Prefer WASI_WASM_PATH env var (set by CI / cargo-make)
    if let Ok(p) = std::env::var("WASI_WASM_PATH") {
        return PathBuf::from(p);
    }
    // Fall back to cargo build output directory
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .ancestors()
        .nth(2)          // workspace root
        .unwrap()
        .join("target/wasm32-wasip2/debug/but.wasm")
});
```

If the `.wasm` is absent, every test that needs it prints a clear message and skips
(via `#[ignore]` or a guard at the top of the test). This prevents hard failures in
a dev environment that hasn't built the WASI target yet.

#### 4. Engine / module caching

Use `wasmtime::Engine` with `Config::new().cache_config_load_default()` (if available)
so the module is compiled once per test binary. Store in a `once_cell::sync::Lazy<Engine>`.

#### 5. Git fixtures for WASI tests

Reuse the existing `crates/but/tests/fixtures/scenario/*.sh` scripts where possible,
referenced via `gix_testtools::scripted_fixture_writable_with_args`. The test harness
copies the fixture into a temp dir and opens it as the WASI preopen.

New scenario scripts needed (minimal):
- `wasi-empty-repo.sh` — `git init && git commit --allow-empty -m 'init'`
- `wasi-one-branch.sh` — one branch with one commit beyond main

These go in `crates/but-wasi-host/tests/fixtures/scenario/`.

#### 6. Determinism controls

- Use `gix_testtools`-style isolation: no `$HOME`, no `$GIT_CONFIG_GLOBAL`, fixed
  `GIT_AUTHOR_DATE` / `GIT_COMMITTER_DATE` inside scenario scripts.
- Strip commit hashes from JSON before snapshot comparison (use snapbox `[HASH]`
  redaction or a custom normaliser).
- No `std::time` / `Instant` assertions in tests — do not assert on wall-clock time.
- No network calls: WASI sandbox has network disabled by default; tests must not call
  any remote.

---

## Test Categories

### 1. Smoke Tests (`smoke.rs`)

These tests require a `.wasm` binary but no git repository.

| Test name | What it checks |
|---|---|
| `wasm_exits_zero_on_help` | `but.wasm -- --help` exits 0 |
| `wasm_exits_zero_on_dash_h` | `but.wasm -- -h` exits 0 |
| `wasm_help_contains_expected_text` | stdout contains "Usage: but" |
| `wasm_exits_nonzero_on_unknown_command` | exits non-zero, stderr contains "not a command" |
| `wasm_version_flag` | `but.wasm -- --version` exits 0, stdout contains version string |
| `wasm_completions_bash` | `but.wasm -- completions bash` exits 0, stdout non-empty |
| `wasm_completions_zsh` | `but.wasm -- completions zsh` exits 0, stdout non-empty |

### 2. Command Tests (`commands.rs`)

These tests require a minimal git repo passed as a preopen.

| Test name | Command exercised | What it checks |
|---|---|---|
| `branch_list_empty_repo` | `but branch` | exits 0, valid JSON or human output |
| `branch_list_one_branch` | `but branch` on `wasi-one-branch` fixture | branch name appears in output |
| `config_read_key` | `but config --json` | exits 0, JSON is valid |
| `alias_list_no_aliases` | `but alias` | exits 0 |
| `completions_bash_is_valid_shell` | `but completions bash` | output is valid bash (contains `complete`) |

### 3. Sandbox Tests (`sandbox.rs`)

These tests verify that the WASI sandbox enforces capability restrictions.

| Test name | What it checks |
|---|---|
| `cannot_read_outside_preopen` | Run `but --help` but point CWD at a dir not in preopens; must not be able to read `~/.gitconfig` or `/etc/passwd` |
| `cannot_write_outside_preopen` | Verify no file is created outside the preopen dir after a write command |
| `network_access_denied` | A command that would normally make an HTTP request (e.g. `but forge`) must fail with a WASI networking error, not succeed |
| `preopen_is_read_write` | A write command inside the preopened repo dir succeeds |

**Implementation note for sandbox tests:** The tests use `run_with_preopens` with a
deliberately restricted set. For network denial, they rely on the `but-wasi-host`
sandbox config having `wasi-http` disabled (no `allow_network` flag). The test simply
runs a command that would require network if native, and asserts that it either exits
non-zero or the error message mentions WASI/network, not a higher-level logic error.

### 4. Comparison Tests (`comparison.rs`)

These tests run the same command on both the native `but` binary (via `snapbox::cmd::cargo_bin!`)
and the WASI module, then compare JSON output.

Approach:
1. Set up a git fixture via the existing `but_testsupport::Sandbox` infrastructure.
2. Run `but --json <command>` natively with `BUT_OUTPUT_FORMAT` and the sandbox env.
3. Run the same command via `WasiTestFixture::run` with `--json`.
4. Normalise both outputs (strip hashes, timestamps) using `sanitize_uuids_and_timestamps`.
5. `assert_eq!(native_json, wasi_json)`.

| Test name | Command compared |
|---|---|
| `help_output_matches_native` | `--help` |
| `branch_list_matches_native` | `branch --json` on `wasi-one-branch` fixture |
| `completions_bash_matches_native` | `completions bash` |

**Divergence policy:** If a comparison test reveals a genuine divergence (e.g. a field
that only native can provide), the test documents it with a comment and uses a normaliser
to mask the divergent field. Do not paper over real bugs — flag them in MEMORY.md.

---

## Dev Dependencies to Add

In `crates/but-wasi-host/Cargo.toml` under `[dev-dependencies]`:

```toml
but-testsupport = { workspace = true, features = ["sandbox"] }
wasmtime = { workspace = true }   # already in [dependencies], so no new dep
once_cell.workspace = true
tempfile.workspace = true
serde_json.workspace = true
gix_testtools.workspace = true    # via but-testsupport re-export
snapbox = { workspace = true, features = ["term-svg"] }
insta.workspace = true
anyhow.workspace = true
```

---

## CI Notes (for s16)

- The WASI integration tests are gated on `#[cfg(feature = "wasi-tests")]` or an
  env-var guard (`WASI_WASM_PATH` set). This means `cargo test -p but-wasi-host`
  in a normal dev environment skips gracefully.
- CI builds `but.wasm`, sets `WASI_WASM_PATH`, then runs the integration tests.
- The comparison tests additionally require the native `but` binary in `$PATH`
  (or via `cargo_bin!("but")`), so CI must build both targets.

---

## Open Questions / Risk Notes

- `gix_testtools` is a transitive dep of `but-testsupport`; confirm it is
  re-exported or add it directly to `but-wasi-host` dev-deps.
- The sandbox network test depends on s14 having disabled `wasi-http` by default.
  If s14 does not expose a clean "no network" mode, the sandbox test will need to
  construct the wasmtime `Store` directly.
- Comparison tests require the native `but` binary to be compiled with `--features wasi`
  (not `legacy`) to ensure the command set matches. The `Sandbox::but()` helper defaults
  to `cargo_bin!("but")`, which uses whatever features were compiled. A separate helper
  `wasi_feature_but()` may be needed that invokes `but` with `--no-default-features
  --features wasi` — or simply skip comparison tests if the native binary has `legacy`.
