## Q1: Should `but-update/check.rs` be excluded entirely from WASI rather than stubbed?

**Context:** `check_status()` in `but-update` makes an HTTP request to the GitButler update server
and uses `machine_uid::get()` to generate an install ID. Under WASI, `but` runs as an MCP server
process, not an end-user CLI. Update checking is not meaningful in that context. The `but-update`
crate also depends on `but-secret` (to read the access token for the update auth header) and
`but-settings` (to check if updates are enabled). Both of those crates require native gating.
It would be simpler to make `but-update` itself optional under `native`, removing it from the
WASI dep tree entirely — but this may require changes to how `crates/but` calls into it.

**Options:**
A) Gate `but-update` as optional under `native` in `crates/but`, removing it from WASI entirely.
B) Keep `but-update` in the WASI dep tree; stub `machine-uid` and the update-check network call.

**Blocking:** No — implementation can proceed with option A as the safer choice, revisiting if needed.

**Response:** 2026-03-13 — Option A, but note: s06 already plans to make `but-update` optional in `crates/but/Cargo.toml` under `native`. s07 does NOT need to handle `but-update` — it's s06's scope. s07 only needs to handle `machine-uid` inside `but-update/Cargo.toml` if `but-update` needs internal gating, but since s06 makes the whole crate optional, that's moot.
**Source:** coordinator — s06 plan

---

## Q2: Env-var secret naming: flat vs. namespaced?

**Context:** The proposed scheme `GITBUTLER_SECRET_{NAMESPACE}_{HANDLE}` uses the full application
identifier (e.g., `COM_GITBUTLER_APP`) in the env var name. This is verbose. An alternative is
a flat scheme: `GITBUTLER_{HANDLE_UPPER}` (e.g., `GITBUTLER_ACCESS_TOKEN`), relying on the fact
that the WASI process is isolated and namespace collisions between build kinds are irrelevant in a
container/sandbox context.

**Options:**
A) Full namespaced scheme: `GITBUTLER_SECRET_COM_GITBUTLER_APP_GITBUTLER_ACCESS_TOKEN`
B) Flat scheme: `GITBUTLER_ACCESS_TOKEN` (handle only, uppercased)
C) Abbreviated namespace: `GITBUTLER_<BUILDKIND>_<HANDLE>` where buildkind is `dev|nightly|release`

**Blocking:** Yes — the env-var scheme must be settled before implementation to avoid breaking
changes to the host-side configuration.

**Response:** 2026-03-13 — Option B (flat scheme). The WASI process is sandboxed and isolated — namespace collisions between build kinds are irrelevant inside a container. `GITBUTLER_ACCESS_TOKEN` is far more ergonomic than `GITBUTLER_SECRET_COM_GITBUTLER_APP_GITBUTLER_ACCESS_TOKEN`. The host configures env vars for the sandbox, so simplicity wins. Use `GITBUTLER_{HANDLE_UPPER}` with dashes/dots replaced by underscores.
**Source:** coordinator decision

---

## Q3: Should `but-installer` be excluded from the WASI dep tree?

**Context:** `but-installer` is an unconditional dependency of `crates/but`. It requires `dirs`
for home directory detection and `env::consts::OS/ARCH` for platform detection. The installer
functionality (`but update install`) is already gated `#[cfg(unix)]` at the callsite. However,
the crate itself still compiles. Under WASI, `env::consts::OS` returns `"wasi"` and `env::consts::ARCH`
returns `"wasm32"`, which would cause `config.rs` to bail early — but only at runtime. The compile-time
issue is `dirs::home_dir()` if `dirs` is not available.

**Options:**
A) Make `but-installer` an optional dep of `crates/but` under `native` (cleanest).
B) Keep `but-installer` in the WASI dep tree; gate `dirs` as optional inside `but-installer`.

**Blocking:** No — option B is implementable as described in the plan.

**Response:** 2026-03-13 — Option A. Make `but-installer` optional in `crates/but/Cargo.toml` under `native`. The installer has no purpose in WASI.
**Source:** coordinator decision
