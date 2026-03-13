## Q1: s05 sequencing — `native` feature ownership

**Context:** s05 plans to set `native = ["dep:command-group", "dep:posthog-rs", "dep:machine-uid"]`
in `crates/but/Cargo.toml`. s06 needs to extend that same line with five more
`dep:` entries. If s05 and s06 both edit that line independently, one will
create a merge conflict.

**Options:**
A) Sequence s05 strictly before s06; s06 rebases on s05 and simply appends to the `native` line.
B) Treat s06 as a dep of s05 (add s05 to s06's `Deps` field). The coordinator
   should not start s06 implementation until s05 is merged.
C) Combine s05 and s06 into one PR since they both edit `crates/but/Cargo.toml`
   extensively and have overlapping scope.

**Blocking:** yes — cannot finalize the `native` feature line in `Cargo.toml`
without knowing what s05 will write there first.

**Response:** 2026-03-13 — Option A. s05 is already implementing and will land first. s06 implementation must wait for s05 to complete, then stack on s05's branch (`pr1/s01.s05/feat/wasi-gate-process`) and extend the `native` feature line. Updated s06 deps to include s05.
**Source:** coordinator decision

---

## Q2: `but-secret` / `keyring` on WASI

**Context:** `crates/but/Cargo.toml` has `but-secret.workspace = true` as a hard dep
(line 56). `but-secret` depends on `keyring`, which requires OS-native secret store
APIs (macOS Keychain, Linux Secret Service, Windows DPAPI). These are not
available on WASI.

s06's scope is HTTP/networking. `keyring` is not a networking dep; it is a
platform-native dep. But it will cause a WASI compile failure just as much as
`reqwest`.

**Options:**
A) Expand s06 scope to also gate `but-secret` behind `native` (since it's needed
   by the network crates anyway and is also platform-native).
B) Create a separate sub-PR (e.g., s07) for platform-native non-network deps
   (`keyring`, `git2`, `keyring` in `gitbutler-user`).
C) The coordinator already has a sub-PR planned for `git2` and keyring — confirm
   which sub-PR owns `but-secret`.

**Blocking:** yes — if `but-secret` stays as a hard dep, `cargo check --features wasi`
will fail even after s06 is complete.

**Response:** 2026-03-13 — Option C. s07 ("Gate Platform-Specific Dependencies") already covers `keyring` in `but-secret`, `notify` in `but-settings`, `dirs` in `but-path`, `machine-uid`, and `open` in `but-api`. s06 scope stays on HTTP/networking only. Note: s06's acceptance criteria should check "no reqwest in the dep tree" (via `cargo tree -i reqwest`), NOT `cargo check --features wasi` which will still fail until s07 handles `but-secret`.
**Source:** coordinator — s07 INDEX.md scope

---

## Q3: `but-forge-storage` removal scope

**Context:** `but-forge-storage` has no network deps and is WASI-safe. However, it
is currently used by `but-github` and `but-gitlab` (which will be optional under
`native`). If `but-forge-storage` stays as a hard dep in `but`, it will be compiled
for WASI — but it itself is safe.

**Question:** Is there any code in `crates/but/src/` that uses `but-forge-storage`
directly (outside of forge/github/gitlab call paths), which would require its types to
be available unconditionally? If so, it should stay as a hard dep. If it's only
referenced through `but-forge`, it can also be made optional.

**Options:**
A) Leave `but-forge-storage` as a hard dep (safe, no networking).
B) Make it optional too, activated by `native`, since its types are only useful
   alongside `but-forge`/`but-github`/`but-gitlab`.

**Blocking:** no — option A is the safe default. Just needs a decision before implementation.
