# s06: Gate Networking Dependencies for WASI

| Field     | Value                                              |
|-----------|----------------------------------------------------|
| **ID**    | s06                                                |
| **Branch**| `pr1/s01.s06/feat/wasi-gate-networking`            |
| **Anchor**| `pr1/s01.s05/feat/wasi-gate-process`               |
| **Deps**  | s01, s05                                           |
| **Size**  | M                                                  |
| **Commit**| `feat: gate networking dependencies (reqwest, ssh2, posthog) for WASI` |

## Status: plan-complete

## Overview

WASI targets cannot use `reqwest` (which requires OS sockets/TLS), `posthog-rs`
(which uses reqwest internally), `machine-uid` (used only for analytics), or
`async-openai` (which uses reqwest for HTTP). None of `ssh2` or `hyper` are
direct dependencies; `reqwest` is the single networking crate to eliminate.

The strategy is: make each networking crate optional in its own `Cargo.toml`
behind a new `network` feature, then make the relevant upstream crates
(`but-github`, `but-gitlab`, `but-update`, `but-llm`, `but-forge`) optional in
`crates/but/Cargo.toml`, activated only when the `native` feature is enabled.

`posthog-rs` and `machine-uid` are already identified by s05 as needing gating
in `crates/but/Cargo.toml` under the `native` feature. s06 coordinates with
s05 on those; the remaining work in s06 is gating the crates that carry
`reqwest` as a direct dependency.

---

## Crate-by-Crate Analysis

### 1. `crates/but-github`

**Direct networking deps:** `reqwest = { workspace = true, features = ["json"] }` (hard).
Dev-dep also has `reqwest = { workspace = true, features = ["blocking"] }` — dev-only, does not affect WASI compilation.

**What uses it:** `src/client.rs` — `reqwest::Client` for all GitHub REST/GraphQL calls.
`src/lib.rs` — re-exports types.

**Currently optional in `but`?** No — `but-github.workspace = true` (line 63 of `crates/but/Cargo.toml`) is a hard dep.

**Existing feature flags in `but-github`:** `export-ts`, `export-schema`. No `network` feature.

**Changes needed:**

- Add a `network` feature to `crates/but-github/Cargo.toml` that enables `dep:reqwest`.
- Make `reqwest` optional: `reqwest = { workspace = true, features = ["json"], optional = true }`.
- Wrap all `reqwest::` usage in `src/client.rs` with `#[cfg(feature = "network")]` or
  conditionally compile the entire module.
- In `crates/but/Cargo.toml`: make `but-github` optional
  (`but-github = { workspace = true, optional = true }`) and add it to the `native` feature.

**Alternatively (simpler):** Leave `but-github`'s `Cargo.toml` unchanged and just make
`but-github` itself optional in `crates/but/Cargo.toml`. Since `but-github`'s entire purpose
is networking, there is no WASI-useful subset. Making it optional in `but` is the right lever.

**Recommended approach:** Make `but-github` optional in `crates/but/Cargo.toml`.

---

### 2. `crates/but-gitlab`

**Direct networking deps:** `reqwest = { workspace = true, features = ["json"] }` (hard).
Dev-dep also has `reqwest = { workspace = true, features = ["blocking"] }`.

**What uses it:** `src/client.rs` — `reqwest::Client` for GitLab REST calls.

**Currently optional in `but`?** No — `but-gitlab.workspace = true` (line 64) is hard.

**Existing features:** `export-ts`, `export-schema`. No `network` feature.

**Recommended approach:** Make `but-gitlab` optional in `crates/but/Cargo.toml`.

---

### 3. `crates/but-forge`

**Direct networking deps:** None — `but-forge/Cargo.toml` does not list `reqwest` directly.
However it depends hard on `but-github` and `but-gitlab` (both pull `reqwest`).

**Currently optional in `but`?** No — `but-forge.workspace = true` (line 65) is hard.

**Existing features:** `export-ts`, `export-schema`. No `network` feature.

**Recommended approach:** Make `but-forge` optional in `crates/but/Cargo.toml`. Since
`but-forge` unconditionally depends on `but-github` and `but-gitlab`, gating `but-forge`
also removes the transitive `reqwest` paths through those two crates.

---

### 4. `crates/but-forge-storage`

**Direct networking deps:** None — `but-forge-storage/Cargo.toml` lists only `but-fs`,
`serde`, `anyhow`, `serde_json`. No `reqwest`, no networking at all.

**Currently optional in `but`?** No — `but-forge-storage.workspace = true` (line 66).

**Assessment:** `but-forge-storage` is WASI-safe as-is (no network deps). It does not
need to be made optional for networking reasons. However, if `but-forge` is made optional,
`but-forge-storage` can remain as a hard dep because it only does filesystem-based
credential/token storage.

**No changes needed for WASI networking.**

---

### 5. `crates/but-update`

**Direct networking deps:** `reqwest.workspace = true` (hard, line 19).
Also `machine-uid` (used to compute install fingerprint). `tokio` for the runtime.

**What uses it:** `src/check.rs` — synchronous HTTP request to `https://app.gitbutler.com/updates`.

**Currently optional in `but`?** No — `but-update.workspace = true` (line 57) is hard.

**Existing features:** None.

**Changes needed:**

Option A (crate-level feature): Add `network` feature to `but-update/Cargo.toml` that gates
`reqwest` and `machine-uid`. Wrap `check_status` and related types behind `#[cfg(feature = "network")]`.

Option B (make optional in `but`): Make `but-update` optional in `crates/but/Cargo.toml`
and activate only under `native`. If `but-update` is used in non-legacy code paths (e.g.,
`but update` subcommand), those call sites must also be gated.

**Recommended approach:** Make `but-update` optional in `crates/but/Cargo.toml` (Option B),
consistent with the pattern used for other network crates. Gate the `update` subcommand
call site with `#[cfg(feature = "native")]`.

Note: `machine-uid` in `but-update/Cargo.toml` is a separate instance from the one in
`crates/but/Cargo.toml`. Both need gating. s05 handles the one in `crates/but`; s06 handles
the one in `but-update` (via making the crate optional).

---

### 6. `crates/but-llm`

**Direct networking deps:** `reqwest.workspace = true` (hard, line 24). Also `async-openai`
(which uses `reqwest` internally via the `rustls` feature).

**What uses it:** `src/anthropic.rs` — `reqwest::Client` for direct Anthropic API calls.
`src/openai.rs` — uses `async-openai::Client` which is backed by `reqwest`.

**Currently optional in `but`?** No — `but-llm.workspace = true` (line 70) is hard.

**Existing features:** None.

**Recommended approach:** Make `but-llm` optional in `crates/but/Cargo.toml`, activated only
under `native`. Also gate `but-action` (which depends on `but-llm`) — but `but-action` is
already optional (under `legacy`). Check: `but-claude` (optional, under `legacy`) also
depends on `but-llm`.

One complication: `but-tools` (not optional in `but-llm`, but used by `but-llm`) also
pulls in `async-openai`. Since `but-llm` will be made optional, the `but-llm` → `but-tools`
→ `async-openai` chain is cut automatically when `but-llm` is excluded.

---

### 7. `crates/but-installer`

**Direct networking deps:** `curl = { version = "0.4.49", default-features = false }` — uses
libcurl (system library), NOT reqwest.

**Currently a dependency of `but`?** No — `but-installer` is a separate binary, not listed
in `crates/but/Cargo.toml` as a dependency. It is an independent binary crate.

**Assessment:** `but-installer` is not a problem for `but` CLI WASI compilation. No changes
needed in the context of s06.

---

### 8. `crates/but-secret`

**Direct networking deps:** None — uses `gix` (with `credentials` feature for git credential
helpers, which is local process invocation, not HTTP), `keyring` (OS keychain API).

**`keyring` on WASI?** `keyring` requires OS platform APIs (macOS Keychain, Linux Secret
Service, Windows Credential Store). These are not available on WASI. However, `but-secret`
is a dependency of `but-github`, `but-gitlab`, `but-update`, and `but-llm`. If those crates
are made optional, `but-secret` only remains as a transitive dep of other crates.

**Direct dep in `but`?** `but-secret.workspace = true` (line 56 of `crates/but/Cargo.toml`)
is a hard dep. `but-secret` itself uses `keyring` which is WASI-incompatible. This means
`but-secret` also needs to be gated or needs a WASI-stub feature.

**Assessment:** This is a secondary concern — `keyring` is not a networking dep, but it is
a platform-native dep that will fail on WASI. However, this may already be addressed by s01
(feature flags) or may be in scope for a separate sub-PR. For s06, note it as a blocker but
do not add it to scope; s06 focuses on HTTP/networking.

---

### 9. `gix` — does it pull reqwest transitively?

The workspace `gix` entry: `gix = { version = "0.80.0", ..., default-features = false, features = ["sha1"] }`.

`gix` with `default-features = false` does NOT enable its network client features. The
networking features in gix (`blocking-network-client`, `async-network-client`,
`blocking-http-transport-curl`, `blocking-http-transport-reqwest`, etc.) are all optional and
are NOT activated by any current usage in the codebase. The `but` crate uses
`features = ["tracing", "tracing-detail"]` — no network features.

**Assessment:** `gix` does NOT pull `reqwest` or any HTTP library transitively in this
codebase. No action needed.

---

### 10. `ssh2` — is it in the dep chain?

A grep across all `crates/*/Cargo.toml` found no match for `ssh2`. It is not a direct
dependency of any crate in this workspace.

`gix` with its current features does not enable SSH transport either. No git2 SSH features
are used (git2 uses `vendored-openssl` and `vendored-libgit2` but not `ssh` feature
explicitly).

**Assessment:** `ssh2` is not in the dependency chain. No action needed.

---

### 11. `async-openai` — reqwest usage

`async-openai` is configured with `features = ["rustls", "chat-completion"]` in the
workspace `Cargo.toml`. The `rustls` feature of `async-openai` enables `reqwest` with
`rustls-tls`. This means `async-openai` → `reqwest` is a transitive path.

`async-openai` appears in:
- `crates/but-llm/Cargo.toml` (which becomes optional via the `but-llm` gating)
- `crates/but-tools/Cargo.toml` (which is a dep of `but-llm`, and transitively optional)

---

## Summary of `reqwest` Paths into `but`

| Path | Via | Hard or Optional in `but` |
|------|-----|--------------------------|
| `reqwest` direct | `but-github` | Hard |
| `reqwest` direct | `but-gitlab` | Hard (via `but-forge`) |
| `reqwest` direct | `but-update` | Hard |
| `reqwest` direct | `but-llm` | Hard |
| `reqwest` via `async-openai` | `but-llm` → `but-tools` | Hard |
| `reqwest` via `async-openai` | `but-llm` (direct) | Hard |
| `posthog-rs` (uses reqwest) | `but` crate direct | Hard (s05 scope) |

No `ssh2` in dep tree. No `hyper` directly. No `ureq`. No networking in `gix` features.

---

## Changes Required

### `crates/but/Cargo.toml`

Make the following hard deps optional, and add them to the `native` feature activation list:

```toml
# Change from:
but-github.workspace = true
but-gitlab.workspace = true
but-forge.workspace = true
but-update.workspace = true
but-llm.workspace = true

# To:
but-github = { workspace = true, optional = true }
but-gitlab = { workspace = true, optional = true }
but-forge = { workspace = true, optional = true }
but-update = { workspace = true, optional = true }
but-llm = { workspace = true, optional = true }
```

Update the `native` feature (currently `native = []`) to activate all network crates plus
the process/metrics deps from s05:

```toml
native = [
    "dep:command-group",    # s05
    "dep:posthog-rs",       # s05
    "dep:machine-uid",      # s05
    "dep:but-github",       # s06
    "dep:but-gitlab",       # s06
    "dep:but-forge",        # s06
    "dep:but-update",       # s06
    "dep:but-llm",          # s06
]
```

Note: `but-forge-storage` stays as a hard dep (no network deps). `but-forge` being optional
means its hard dependency on `but-github` and `but-gitlab` is only active when `but-forge`
itself is active, which is already controlled by the `native` feature.

### `crates/but-github/Cargo.toml`

No internal changes needed. The entire crate becomes optional in `but`.

### `crates/but-gitlab/Cargo.toml`

No internal changes needed. The entire crate becomes optional in `but`.

### `crates/but-forge/Cargo.toml`

No internal changes needed. The entire crate becomes optional in `but`.

### `crates/but-update/Cargo.toml`

No internal changes needed if we gate the entire crate as optional in `but`. The crate's
own `reqwest` and `machine-uid` deps become irrelevant when the crate is not compiled.

### `crates/but-llm/Cargo.toml`

No internal changes needed if we gate the entire crate as optional in `but`.

---

## Source File Changes in `crates/but/src/`

All call sites in `crates/but/src/` that reference `but_github`, `but_gitlab`, `but_forge`,
`but_update`, or `but_llm` must be wrapped with `#[cfg(feature = "native")]`.

Key locations to audit:
- `src/args/forge/` — forge/PR subcommands, reference `but_forge`, `but_github`, `but_gitlab`
- `src/args/skill.rs` — `but_update::check_status` call
- `src/command/skill.rs` or `src/command/update.rs` — update check
- `src/args/mod.rs` or `src/lib.rs` — LLM integration via `but_llm`
- Any `use but_github::`, `use but_gitlab::`, `use but_forge::`, `use but_update::`,
  `use but_llm::` imports

All such imports and their corresponding `match` arms / command handlers need
`#[cfg(feature = "native")]` guards.

---

## What Does NOT Need Changes

- `crates/but-forge-storage` — no network deps, can remain a hard dep
- `gix` — network features not enabled, no reqwest path
- `ssh2` — not in dep tree at all
- `hyper`, `ureq` — not in dep tree
- `crates/but-installer` — separate binary, not a dep of `but`
- `crates/but-link` — no network deps
- `crates/but-gerrit` — no network deps (uses `gix` only)
- `crates/but-secret` — not network (platform keychain), separate concern

---

## Interaction with s05

s05 already plans to gate `posthog-rs`, `machine-uid`, and `command-group` behind `native`
in `crates/but/Cargo.toml`. s06 extends that same `native` feature to cover the five
network crates. The implementation must coordinate:

- s05 sets `native = ["dep:command-group", "dep:posthog-rs", "dep:machine-uid"]`
- s06 extends to: `native = ["dep:command-group", "dep:posthog-rs", "dep:machine-uid", "dep:but-github", "dep:but-gitlab", "dep:but-forge", "dep:but-update", "dep:but-llm"]`

If s05 lands first, s06 amends the `native` line. If s06 lands first, s05 must be rebased.
Since both branch from the s01 anchor, the coordinator should sequence s05 before s06 or
have s06 declare s05 as an additional dep.

---

## Files to Edit

| File | Change |
|------|--------|
| `crates/but/Cargo.toml` | Make 5 crates optional; extend `native` feature |
| `crates/but/src/` (various) | Gate `use` imports and call sites with `#[cfg(feature = "native")]` |

No changes to the individual library crates (`but-github`, `but-gitlab`, etc.).

---

## Acceptance Criteria

- [ ] `but-github` is optional in `crates/but/Cargo.toml`, activated by `native`
- [ ] `but-gitlab` is optional in `crates/but/Cargo.toml`, activated by `native`
- [ ] `but-forge` is optional in `crates/but/Cargo.toml`, activated by `native`
- [ ] `but-update` is optional in `crates/but/Cargo.toml`, activated by `native`
- [ ] `but-llm` is optional in `crates/but/Cargo.toml`, activated by `native`
- [ ] `native` feature in `crates/but/Cargo.toml` activates all five crates (plus s05 deps)
- [ ] All `use but_github::`, `use but_gitlab::`, `use but_forge::`, `use but_update::`, `use but_llm::` in `crates/but/src/` are gated `#[cfg(feature = "native")]`
- [ ] All forge/PR/LLM/update subcommand arms in `Subcommands` match are gated `#[cfg(feature = "native")]`
- [ ] No `reqwest` crate in WASI dep tree (`cargo tree -p but --no-default-features --features wasi -i reqwest 2>&1` produces no output or error)
- [ ] No `async-openai` in WASI dep tree
- [ ] `cargo check -p but` succeeds with default features (native build unaffected)
- [ ] `cargo test -p but` passes (existing tests unbroken)
- [ ] `but-forge-storage` remains a hard dep (no change needed)
- [ ] Note: `cargo check -p but --no-default-features --features wasi` may still fail due to `but-secret`/`keyring` (s07 scope) — this is expected
