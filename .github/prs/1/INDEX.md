# PR #1: Compile `but` CLI to WASI

## Goal

Compile the GitButler `but` CLI to `wasm32-wasip2` so that agents running `but` are sandboxed by the WASI runtime. This provides:

1. **Filesystem isolation** — agents can only access preopened directories
2. **Network restriction** — no network access unless explicitly granted via wasi-http
3. **Portability** — a single `.wasm` binary runs on any platform with a WASI runtime
4. **Security** — capability-based access control via Wasmtime

## Motivation

When AI agents use `but` for git operations, they currently have unrestricted filesystem and network access. By compiling to WASI and running inside a Wasmtime sandbox (as implemented by [mcagent](https://github.com/willemneal/mcagent)), we constrain agent capabilities to exactly what's needed: access to the repository directory and nothing else.

## Technical Decisions

### WASI Target: `wasm32-wasip2`
- Component model support for composable modules
- `wasi-http` for controlled networking (future)
- Supported by Wasmtime 29+

### Feature Flag Strategy: `wasi` Cargo Feature
- **Not** `#[cfg(target_arch = "wasm32")]` — a cargo feature is testable on native
- `wasi` feature implies: no `legacy`, no TUI, no direct networking, no keyring
- Composes cleanly with the existing `legacy` feature flag

### Initial Command Scope
Non-legacy, non-TUI commands: `branch`, `config`, `alias`, `help`, `completions`, `skill` (read-only), `eval-hook`.

### git2 Approach: Feature-Gate, Don't Migrate
The project already uses `gix` (pure Rust) heavily. For WASI, gate `git2` out and accept reduced functionality.

### Runtime: Wasmtime via mcagent Model
New `but-wasi-host` crate using Wasmtime with capability-based FS, COW isolation, CLI arg passthrough.

## WASI Tools Goal

In addition to compiling `but` itself to WASI, this PR aims to build WASI-compatible tooling. The bash scripts in `scripts/bin/` serve as stand-ins — each will eventually be replaced by a WASI binary that agents can invoke inside the sandbox. This dogfoods the WASI compilation work.

**Current stand-in tools** (bash, in `scripts/bin/`):
- `but-apply-pattern`, `but-unapply-pattern` — branch management by regex
- `but-stage-all` — bulk staging
- `but-changes`, `but-branch-ids`, `but-branch-commits`, `but-diff-files` — status queries
- `but-setup-branches` — branch topology from JSON

**Target:** Replace with `crates/but-tools/` compiled to `wasm32-wasip2`.

## Workflow

### Sequential Agents, On-Demand Branches
Agents run one at a time. Each creates its branch when it starts work. The branch name encodes dependencies — if stacking is needed, the agent anchors on the dependency's branch. Otherwise it anchors on `feat/wasi`.

```bash
# Agent creates branch (anchor derived from dependency prefix in name)
but branch new pr1/<sub-pr-name> -a <anchor>

# Agent works, commits
but commit pr1/<sub-pr-name> -m "<summary>" --json --status-after

# Agent amends as work progresses
but absorb

# Commit message matches INDEX.md summary
but reword <commit-id> -m "<INDEX.md summary>"
```

Work stays on the agent's branch. The next agent stacks on top if it depends on this one, or branches from `feat/wasi` if it doesn't. The dependency graph determines ordering — the next agent to run is the next unblocked sub-PR.

## Sub-PR Breakdown (16 sub-PRs)

| ID | Folder | Branch | Description | Size |
|----|--------|--------|-------------|------|
| s01 | `s01/feat/wasi-feature-flags/` | `pr1/s01/feat/wasi-feature-flags` | Cargo `wasi` feature + build config | S |
| s02 | `s01.s02/feat/wasi-serde-objectid/` | `pr1/s01.s02/feat/wasi-serde-objectid` | but-serde: ObjectId abstraction (gate git2::Oid) | M |
| s03 | `s01.s03/feat/wasi-oxidize-noop/` | `pr1/s01.s03/feat/wasi-oxidize-noop` | but-oxidize: no-op under wasi | S |
| s04 | `s01.s04/feat/wasi-gate-tui/` | `pr1/s01.s04/feat/wasi-gate-tui` | Gate ratatui/crossterm/pager | M |
| s05 | `s01.s05/feat/wasi-gate-process/` | `pr1/s01.s05/feat/wasi-gate-process` | Gate std::process::Command | M |
| s06 | `s01.s06/feat/wasi-gate-networking/` | `pr1/s01.s06/feat/wasi-gate-networking` | Gate reqwest/HTTP deps | M |
| s07 | `s01.s07/feat/wasi-gate-platform/` | `pr1/s01.s07/feat/wasi-gate-platform` | Gate keyring/dirs/notify | M |
| s08 | `s01.s08/feat/wasi-gate-sqlite/` | `pr1/s01.s08/feat/wasi-gate-sqlite` | Gate or adapt rusqlite | M |
| s09 | `s02.s09/feat/wasi-gate-ctx/` | `pr1/s02.s09/feat/wasi-gate-ctx` | but-ctx: gate git2::Repository field | M |
| s10 | `s09.s10/feat/wasi-gate-core-checkout/` | `pr1/s09.s10/feat/wasi-gate-core-checkout` | but-core: gate worktree/checkout | S |
| s11 | `s05.s11/feat/wasi-tokio-singlethread/` | `pr1/s05.s11/feat/wasi-tokio-singlethread` | Single-threaded tokio runtime | M |
| s12 | `s02-s11.s12/feat/wasi-gix-features/` | `pr1/s02-s11.s12/feat/wasi-gix-features` | gix WASI feature audit + config | M |
| s13 | `s12.s13/feat/wasi-first-compile/` | `pr1/s12.s13/feat/wasi-first-compile` | Fix remaining compile errors | M |
| s14 | `s13.s14/feat/wasi-runtime-host/` | `pr1/s13.s14/feat/wasi-runtime-host` | but-wasi-host: wasmtime host binary | M |
| s15 | `s14.s15/test/wasi-testing/` | `pr1/s14.s15/test/wasi-testing` | WASI test harness + smoke tests | M |
| s16 | `s15.s16/feat/wasi-ci-pipeline/` | `pr1/s15.s16/feat/wasi-ci-pipeline` | CI workflow for WASI builds | S |

## Dependency Graph

```
s01 (feature flags) ✓
├── s02 (serde ObjectId) ✓    ─┐
│   └── s09 (gate ctx)         │
│       └── s10 (gate checkout) │
├── s03 (oxidize noop)         │── tier 1: all depend on s01/s02
├── s04 (gate TUI)             │
├── s05 (gate process)         │
│   ├── s06 (gate networking)  │   ← stacks on s05 (extends native feature)
│   │   └── s07 (gate platform)│   ← stacks on s06 (extends native further)
│   └── s11 (tokio)            │
├── s08 (gate sqlite)         ─┘
│
s12 (gix features)  ── depends on s02 through s11
└── s13 (first compile)
    └── s14 (runtime host)
        └── s15 (testing)
            └── s16 (CI)
```

### Implementation Order (after batch review)

Sequential implementation respects dependency graph + cross-cutting concerns (shared Cargo.toml lines). All plans s03-s16 are reviewed and approved.

| # | Sub-PR | Deps satisfied by | Key concern |
|---|--------|-------------------|-------------|
| 1 | s03 — oxidize noop | s01 ✓ | |
| 2 | s04 — gate TUI | s01 ✓ | |
| 3 | s05 — gate process | s01 ✓ | Adds `native` feature to `crates/but/Cargo.toml` |
| 4 | s08 — gate sqlite | s01 ✓ | Surfaces WASI SDK / rusqlite issues early |
| 5 | s09 — gate ctx | s02 ✓ | Adds `wasi`/`native` to `but-ctx/Cargo.toml` |
| 6 | s06 — gate networking | s05 (#3) | Extends `native` feature (stacks on s05) |
| 7 | s10 — gate core checkout | s09 (#5) | Adds `wasi`/`native` to `but-core/Cargo.toml` |
| 8 | s11 — tokio singlethread | s05 (#3) | Target-specific tokio deps |
| 9 | s07 — gate platform | s05+s06 (#3,#6) | Extends `native` further (secret, settings, path) |
| 10 | s12 — gix features | all s02-s11 | `but-secret` credentials gate; gix parallel removal |
| 11 | s13 — first compile | s12 (#10) | Catch-all: rmcp, config.rs git2, feature wiring |
| 12 | s14 — runtime host | s13 (#11) | New crate: but-wasi-host (wasmtime 29.0.1) |
| 13 | s15 — testing | s14 (#12) | WasiTestFixture harness + 4 test categories |
| 14 | s16 — CI pipeline | s15 (#13) | .github/workflows/wasi.yaml |

### Cross-Cutting Concerns (from batch review)

1. **`crates/but/Cargo.toml` `native` feature line** — s05 creates it, s06 extends it, s07 extends further. Implementation must follow this chain.
2. **`but-secret`** — s07 gates `keyring` (OS dep), s12 gates `gix::credentials` (gix feature). Complementary changes; s07 before s12.
3. **`config.rs`/`alias.rs` git2 usage** — s09 identified non-legacy `ctx.git2_repo.get()` calls. Scoped out of s09, handled by s13.
4. **`rmcp` dependency** — s13 identified as potential WASI blocker (may use tokio net). Not covered by prior sub-PRs.
5. **`gix-command` compile risk** — s12 noted `gix-merge` → `gix-command` → `std::process::Command`. May fail to compile on wasip2. Discovered by s13.

## Branch Creation

Branches are created **on demand** when an agent starts work, not upfront. The branch name encodes the dependency graph — the anchor is derived from it:

```bash
# No deps: anchor on feat/wasi
but branch new pr1/s01/feat/wasi-feature-flags -a feat/wasi

# s01.s02: depends on s01, anchor on s01's branch
but branch new pr1/s01.s02/feat/wasi-serde-objectid -a pr1/s01/feat/wasi-feature-flags

# s02.s09: depends on s02, anchor on s02's branch
but branch new pr1/s02.s09/feat/wasi-gate-ctx -a pr1/s01.s02/feat/wasi-serde-objectid

# s02-s11.s12: depends on s02 through s11, anchor on feat/wasi (all deps already committed)
but branch new pr1/s02-s11.s12/feat/wasi-gix-features -a feat/wasi
```

The dependency prefix in the branch name tells you where to stack. An agent reads the prefix, finds the corresponding branch, and anchors on it. If a dependency has no branch (its work was already committed to `feat/wasi`), anchor on `feat/wasi`.

## Risk Registry

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| `gix` WASI-incompatible features (threading, credentials) | Medium | High | s12 dedicated to this audit |
| `rusqlite` bundled C won't compile for wasip2 | Medium | High | Test early in s08; fallback to JSON-file backend |
| `but-serde` coupled to `git2::Oid` | High | Medium | s02 dedicated to ObjectId abstraction |
| WASM binary size too large | Low | Medium | Feature gating + `wasm-opt -Os` |
| `but-ctx` holds `git2::Repository` | High | High | s09 dedicated to this; s02 (ObjectId) lands first |

## Main Branch

**`feat/wasi`** — umbrella. All sub-PRs merge here. Once complete, `feat/wasi` merges to `master`.

## Acceptance Criteria

- [ ] `cargo build -p but --no-default-features --features wasi --target wasm32-wasip2` succeeds
- [ ] `wasmtime but.wasm -- --help` produces output
- [ ] Core commands (branch, config, help) produce correct output
- [ ] `but-wasi-host` loads and runs the binary with restricted filesystem
- [ ] CI builds and tests the WASI target on every push
- [ ] No regressions to the native build
