# s16: Add GitHub Actions workflow for WASI target builds and tests

- **Branch:** `pr1/s15.s16/feat/wasi-ci-pipeline`
- **Anchor:** `pr1/s14.s15/test/wasi-testing`
- **Deps:** s15 (tests exist to run)
- **Size:** S
- **Commit message:** `feat: add GitHub Actions workflow for WASI target builds and tests`

## Scope

- New workflow: `.github/workflows/wasi.yaml`
- Triggers: push to `feat/wasi`, `pr1/**`, PRs targeting `feat/wasi`
- Steps: checkout, install Rust + wasm32-wasip2 target, install Wasmtime, optional WASI SDK, build WASI binary, build but-wasi-host, run tests, upload .wasm artifact, report binary size
- Size budget: threshold (e.g. 50MB debug, 20MB release), fail CI if exceeded
- Optional reusable action: `.github/actions/init-env-wasi/`

## Files

- `.github/workflows/wasi.yaml` (new)
- `.github/actions/init-env-wasi/action.yml` (new, optional)

## Reference

- Existing `.github/workflows/push.yaml`
- `.github/actions/init-env-node/action.yaml` (the only reusable action in repo — no `init-env-rust` exists)
- Wasmtime GH Action `bytecodealliance/actions/wasmtime/setup@v1`

## Acceptance Criteria

- CI builds WASI on push
- Runs tests
- Uploads artifact
- Reports size
- Fails on regression

---

## Plan (added by s16 planning agent)

### Trigger Configuration

```yaml
on:
  push:
    branches:
      - feat/wasi
      - pr1/**
  pull_request:
    branches:
      - feat/wasi
      - pr1/**
```

This matches how the existing `push.yaml` handles triggers (push to master + any PR), but scoped to WASI branches only. No workflow_dispatch needed initially — can add later.

### Concurrency

Follow the existing repo pattern exactly:

```yaml
concurrency:
  group: ${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: true
```

### Permissions

Follow the existing repo minimum — `contents: read`, `pull-requests: read`. No write access needed.

### Jobs

#### Job 1: `wasi-build-and-test` (ubuntu-latest, single job)

Since the workflow is small (S size), one job covers build + test + report. No matrix needed initially — ubuntu-latest is sufficient. A macOS runner can be added later if cross-platform CI is needed.

**Steps in order:**

1. **Checkout**
   ```yaml
   - uses: actions/checkout@<pinned-sha>
     with:
       persist-credentials: false
   ```
   Pinned SHA required — the existing `check-no-persist-credentials` workflow scans all workflows and will fail CI if `persist-credentials: false` is missing.

2. **Rust target setup**
   ```yaml
   - name: Add wasm32-wasip2 target
     run: rustup target add wasm32-wasip2
   ```
   The `wasm32-wasip2` target is not included in the default Rust toolchain. No custom toolchain override needed — the project's `rust-toolchain.toml` (if present) drives the version.

3. **Cargo cache** (Swatinem/rust-cache, matching existing jobs)
   ```yaml
   - name: Rust Cache
     uses: Swatinem/rust-cache@<pinned-sha>
     with:
       shared-key: wasi-build
       save-if: ${{ github.ref == 'refs/heads/feat/wasi' }}
   ```
   Use a distinct `shared-key` so this cache does not collide with native build caches. Cache is only saved on the primary `feat/wasi` branch to avoid cache thrashing from `pr1/**` branches.

4. **Install Wasmtime** (using the official Bytecode Alliance action)
   ```yaml
   - name: Install Wasmtime
     uses: bytecodealliance/actions/wasmtime/setup@v1
   ```
   This action installs the `wasmtime` CLI into PATH. The version can be pinned via the `version` input. Use the latest stable Wasmtime that supports `wasip2` (29.0.1 or newer, per s14 requirements). Pin by commit SHA once a stable version is identified.

5. **Build WASI binary**
   ```yaml
   - name: Build but.wasm (debug)
     env:
       CARGO_TERM_COLOR: always
     run: |
       cargo build -p but --no-default-features --features wasi --target wasm32-wasip2
   ```
   Debug build for CI speed. Release build is separate (see size check step).

6. **Build but-wasi-host**
   ```yaml
   - name: Build but-wasi-host
     env:
       CARGO_TERM_COLOR: always
     run: cargo build -p but-wasi-host
   ```
   Native binary needed to run the integration tests from s15 (the host embeds Wasmtime and loads but.wasm).

7. **Run WASI tests** (from s15)
   ```yaml
   - name: Run WASI integration tests
     env:
       CARGO_TERM_COLOR: always
     run: |
       cargo test -p but-wasi-host --test '*'
       cargo test -p but --test 'wasi*'
   ```
   These paths match the test file locations defined in s15: `crates/but-wasi-host/tests/` and `crates/but/tests/wasi/`. If s15 uses a different test invocation, this step must be updated.

8. **Build release binary for size check**
   ```yaml
   - name: Build but.wasm (release)
     env:
       CARGO_TERM_COLOR: always
     run: |
       cargo build -p but --no-default-features --features wasi --target wasm32-wasip2 --release
   ```

9. **Report and enforce binary size**
   ```yaml
   - name: Check binary size
     run: |
       WASM_PATH="target/wasm32-wasip2/release/but.wasm"
       SIZE_BYTES=$(stat -c%s "$WASM_PATH")
       SIZE_MB=$(echo "scale=2; $SIZE_BYTES / 1048576" | bc)
       echo "but.wasm release size: ${SIZE_MB} MB (${SIZE_BYTES} bytes)"
       echo "wasm_size_mb=${SIZE_MB}" >> $GITHUB_STEP_SUMMARY
       # Fail if over 20 MB release threshold
       MAX_BYTES=20971520  # 20 MB
       if [ "$SIZE_BYTES" -gt "$MAX_BYTES" ]; then
         echo "ERROR: but.wasm exceeds 20 MB size budget (${SIZE_MB} MB)"
         exit 1
       fi
   ```
   The `stat -c%s` flag is Linux/GNU specific — ubuntu-latest is Linux so this works. The 20 MB release threshold is a starting point that can be tightened once a baseline is established. The debug threshold is not enforced (debug binaries are large and variable).

10. **Upload .wasm artifact**
    ```yaml
    - name: Upload but.wasm artifact
      uses: actions/upload-artifact@<pinned-sha>
      with:
        name: "but-wasm-${{ github.run_number }}"
        path: target/wasm32-wasip2/release/but.wasm
        if-no-files-found: error
        retention-days: 7
    ```
    Matches the artifact upload pattern in `push.yaml` (7-day retention, `if-no-files-found: error`). Upload the release binary, not debug.

### Workflow-Level env

```yaml
env:
  RUST_BACKTRACE: full
  CARGO_TERM_COLOR: always
```

Matches `push.yaml` — `RUST_BACKTRACE: full` ensures useful output on test failures.

### Optional Reusable Action: `.github/actions/init-env-wasi/`

A reusable composite action can wrap steps 2–4 (target add, rust-cache, wasmtime install). This follows the pattern of `.github/actions/init-env-node/action.yaml` which bundles pnpm install + node setup.

**`action.yml` structure:**
```yaml
name: init-env-wasi
description: Prepare WASI build environment (Rust target + cache + Wasmtime)
inputs:
  wasmtime-version:
    description: Wasmtime version to install
    required: false
    default: latest
  cache-save-if:
    description: Condition for saving the Rust cache
    required: false
    default: 'false'
runs:
  using: composite
  steps:
    - name: Add wasm32-wasip2 target
      shell: bash
      run: rustup target add wasm32-wasip2
    - name: Rust Cache
      uses: Swatinem/rust-cache@<sha>
      with:
        shared-key: wasi-build
        save-if: ${{ inputs.cache-save-if }}
    - name: Install Wasmtime
      uses: bytecodealliance/actions/wasmtime/setup@v1
      with:
        version: ${{ inputs.wasmtime-version }}
```

This action is **optional** — create it only if the wasi.yaml workflow becomes unwieldy or if other workflows also need WASI setup. For a single workflow, inlining the steps is cleaner.

**Decision: Skip the reusable action for s16.** Only one workflow will use it; the action abstraction adds indirection without benefit. If a second WASI workflow is added later, extract then.

### Caching Strategy

| Cache | Key pattern | Saved on |
|-------|-------------|----------|
| Cargo registry + build | `Swatinem/rust-cache` with `shared-key: wasi-build` | `feat/wasi` pushes only |
| Wasmtime binary | Handled automatically by `bytecodealliance/actions/wasmtime/setup` | Always |
| WASI SDK | Not needed — wasip2 target is bundled with the Rust toolchain; no separate SDK required |

The WASI SDK (wasi-sdk from WebAssembly/wasi-sdk) is only needed if C code in the build requires a WASI-compatible libc. Since the plan is to gate out C dependencies (git2 via s09, sqlite via s08, rusqlite via s08), no WASI SDK cache is needed. Re-evaluate if C dependencies remain.

### Size Budget

| Artifact | Threshold | Action on breach |
|----------|-----------|-----------------|
| `but.wasm` debug | No limit (not enforced) | — |
| `but.wasm` release | 20 MB | Fail CI |

The 20 MB threshold is conservative. A pure-Rust WASM binary with dead code elimination (`wasm-opt -Os`) typically reaches 5–15 MB for a CLI this size. The threshold can be tightened once a real baseline is measured. To add `wasm-opt`, install `binaryen` via apt before the release build step.

### `check-no-persist-credentials` Compliance

The existing `push.yaml` includes a job that scans all workflows and fails if any `actions/checkout` step is missing `persist-credentials: false`. The new `wasi.yaml` must include this on every checkout step or the main push CI will fail.

### SHA Pinning

All action references must use pinned commit SHAs, not floating tags. This matches the pattern in `push.yaml`. Pins to use at authoring time:
- `actions/checkout` — use same SHA as push.yaml: `de0fac2e4500dabe0009e67214ff5f5447ce83dd`
- `Swatinem/rust-cache` — use same SHA as push.yaml: `779680da715d629ac1d338a641029a2f4372abb5`
- `actions/upload-artifact` — use same SHA as push.yaml: `b7c566a772e6b6bfb58ed0dc250532a479d7789f`
- `bytecodealliance/actions/wasmtime/setup@v1` — find current SHA via the action repo before pinning

### Full Workflow Skeleton

```yaml
name: "WASI"
on:
  push:
    branches:
      - feat/wasi
      - pr1/**
  pull_request:
    branches:
      - feat/wasi
      - pr1/**

env:
  RUST_BACKTRACE: full
  CARGO_TERM_COLOR: always

concurrency:
  group: ${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: true

permissions:
  contents: read
  pull-requests: read

jobs:
  wasi-build-and-test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@de0fac2e4500dabe0009e67214ff5f5447ce83dd # v6.0.2
        with:
          persist-credentials: false
      - name: Add wasm32-wasip2 target
        run: rustup target add wasm32-wasip2
      - name: Rust Cache
        uses: Swatinem/rust-cache@779680da715d629ac1d338a641029a2f4372abb5 # v2.8.2
        with:
          shared-key: wasi-build
          save-if: ${{ github.ref == 'refs/heads/feat/wasi' }}
      - name: Install Wasmtime
        uses: bytecodealliance/actions/wasmtime/setup@v1
        # TODO: pin to SHA once stable version confirmed
      - name: Build but.wasm (debug)
        run: cargo build -p but --no-default-features --features wasi --target wasm32-wasip2
      - name: Build but-wasi-host
        run: cargo build -p but-wasi-host
      - name: Run WASI integration tests
        run: |
          cargo test -p but-wasi-host --test '*'
          cargo test -p but --test 'wasi*'
      - name: Build but.wasm (release)
        run: cargo build -p but --no-default-features --features wasi --target wasm32-wasip2 --release
      - name: Check binary size
        run: |
          WASM_PATH="target/wasm32-wasip2/release/but.wasm"
          SIZE_BYTES=$(stat -c%s "$WASM_PATH")
          SIZE_MB=$(echo "scale=2; $SIZE_BYTES / 1048576" | bc)
          echo "but.wasm release size: ${SIZE_MB} MB"
          MAX_BYTES=20971520
          if [ "$SIZE_BYTES" -gt "$MAX_BYTES" ]; then
            echo "ERROR: but.wasm exceeds 20 MB (${SIZE_MB} MB)"
            exit 1
          fi
      - name: Upload but.wasm artifact
        uses: actions/upload-artifact@b7c566a772e6b6bfb58ed0dc250532a479d7789f # v6.0.0
        with:
          name: "but-wasm-${{ github.run_number }}"
          path: target/wasm32-wasip2/release/but.wasm
          if-no-files-found: error
          retention-days: 7
```

### Open Questions for Implementation

1. **`bytecodealliance/actions/wasmtime/setup` SHA:** The action's commit SHA must be looked up before authoring the final file. The `@v1` tag cannot be used bare — `check-no-persist-credentials` does not block this but it is inconsistent with the repo's pinning convention. Look up the current SHA for `wasmtime/setup@v1` at implementation time.

2. **Test command surface:** The exact `cargo test` invocations depend on how s15 structures its test binaries. The plan uses `--test '*'` and `--test 'wasi*'` as reasonable defaults, but may need adjustment once s15 is implemented.

3. **`but-wasi-host` crate name:** s14 defines the binary as `but-wasi` in the crate `but-wasi-host`. The cargo build command should use `-p but-wasi-host`; the binary name used in tests will be `but-wasi`. This is correct in the skeleton above.

4. **`wasm-opt` integration:** Adding `wasm-opt -Os` (binaryen) after the release build reduces binary size by 20–40%. It requires `apt-get install binaryen`. Defer to a follow-up unless the initial binary exceeds the 20 MB threshold without it.
