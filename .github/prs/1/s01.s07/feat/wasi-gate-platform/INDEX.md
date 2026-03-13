# s07: Gate Platform-Specific Dependencies for WASI

| Field     | Value                                              |
|-----------|----------------------------------------------------|
| **ID**    | s07                                                |
| **Branch**| `pr1/s01.s07/feat/wasi-gate-platform`              |
| **Anchor**| `pr1/s01.s06/feat/wasi-gate-networking`             |
| **Deps**  | s01, s05, s06                                      |
| **Size**  | M                                                  |
| **Commit**| `feat: gate platform-specific deps (keyring, dirs, notify) for WASI` |

## Scope

- Gate `keyring` in `but-secret`; provide env-var auth fallback for WASI
- Gate `notify` in `but-settings` (inotify/fsevents C bindings don't compile for WASI)
- Gate `dirs` in `but-path` (no home directory concept in WASI)
- Gate `dirs` direct usage in `but`, `but-claude`, `but-installer`
- Gate `machine-uid` in `crates/but` and `crates/but-update`
- Gate `open` in `but-api` and `but-path`
- Gate `posthog-rs` + metrics entirely under `native` feature

## Scope Boundaries (Avoid Overlap)

**Already handled by s05:** `command-group`, `posthog-rs`, `machine-uid` in `crates/but/Cargo.toml`; all process-spawning gates in `metrics.rs` and `setup.rs`.

**Already handled by s06:** `but-github`, `but-gitlab`, `but-forge`, `but-update`, `but-llm` made optional in `crates/but/Cargo.toml` under `native`. This also removes `but-update`'s `machine-uid` from the WASI tree.

**s07 unique scope:**
- `keyring` in `but-secret` (+ env-var fallback implementation)
- `notify` in `but-settings`
- `dirs` in `but-path` (+ env-var fallback)
- `open` in `but-path` and `but-api`
- `dirs` in `but-installer` (make crate optional in `but`)
- `git2` in `crates/but/Cargo.toml` (make optional under `native`/`legacy`)
- `dirs` direct usage in `crates/but/src/command/skill.rs`

**NOT s07 scope:** `posthog-rs`, `machine-uid` in `crates/but`, `but-update` optionality, `but-llm` optionality.

## Crate-by-Crate Analysis

### 1. `crates/but-secret` — `keyring`

**What it does:** Wraps the platform OS keychain (Keychain on macOS, libsecret/DBus on Linux,
Windows Credential Manager on Windows) to store and retrieve named secrets.

**Public API surface:**
- `secret::persist(handle, secret, namespace)` — writes to keychain
- `secret::retrieve(handle, namespace)` — reads from keychain, returns `Option`
- `secret::delete(handle, namespace)` — deletes from keychain
- `secret::set_application_namespace(identifier)` — sets global namespace prefix
- `secret::git_credentials::setup()` — installs a `git credential`-backed keyring provider

**Callers of `secret::retrieve` / `secret::persist` / `secret::delete`:**
| Caller crate | Keys retrieved/persisted | Usage |
|---|---|---|
| `gitbutler-user/src/user.rs` | `gitbutler_access_token`, `github_access_token` | User login tokens |
| `but-llm/src/openai.rs` | `gitbutler_access_token`, `aiOpenAIKey` | LLM API keys |
| `but-llm/src/anthropic.rs` | `gitbutler_access_token`, `aiAnthropicKey` | LLM API keys |
| `but-github/src/token.rs` | `github_oauth_*`, `github_pat_*`, `github_enterprise_*` | GitHub tokens |
| `but-gitlab/src/token.rs` | `gitlab_pat_*`, `gitlab_selfhosted_*` | GitLab tokens |
| `but-update/src/check.rs` | `gitbutler_access_token` | Update check auth |
| `but-api/src/legacy/secret.rs` | any `handle` | Generic Tauri secret API |
| `but-api/src/github.rs`, `but-api/src/gitlab.rs` | (delegate to above) | Forge token wrappers |

**WASI problem:** `keyring = "3.6.3"` uses features `["apple-native", "windows-native",
"linux-native-sync-persistent", "crypto-rust"]`. All three native backends require OS-specific
C libraries or system calls unavailable under WASI.

**Fallback strategy for WASI:** Provide a WASI implementation of `secret::retrieve` / `secret::persist`
/ `secret::delete` that reads from / writes to environment variables using a naming scheme:
`GITBUTLER_SECRET_<NAMESPACE>_<HANDLE>` (uppercased, dashes replaced with underscores).
WASI processes are given a pre-configured environment by the host, so env vars are the
idiomatic credential injection point.

**Env-var naming scheme:**
- Namespace `BuildKind` → prefix from `set_application_namespace`, defaulting to `development`:
  e.g., `GITBUTLER_SECRET_COM_GITBUTLER_APP_GITBUTLER_ACCESS_TOKEN`
- Namespace `Global` → prefix `GITBUTLER`:
  e.g., `GITBUTLER_SECRET_AIOPENAIKEY`
- Algorithm: uppercase the concatenated `{namespace_prefix}_{handle}`, replacing `.` and `-` with `_`.

Under WASI the secrets store is read-only from env vars; `persist` and `delete` return `Ok(())`
as no-ops (credentials are injected by the host, not written at runtime).

**Cargo.toml changes needed:**
```toml
# crates/but-secret/Cargo.toml
[features]
native = ["dep:keyring"]

[dependencies]
keyring = { workspace = true, optional = true }
```

The `keyring` feature must be enabled by default for native builds and absent for WASI.
All callers of `but-secret` remain unchanged; the implementation switches via `#[cfg(feature = "native")]`.

**`gix::credentials` shelling out:** The `git_credentials::setup()` function in `but-secret` invokes
git credential helpers via `gix::credentials::helper::Cascade::invoke()`. Under WASI this path is
unreachable: `git_credentials::setup()` is only called from `set_application_namespace()` when
`cfg!(debug_assertions) && cfg!(target_os = "macos")`. Since WASI has neither macOS semantics nor
debug-assertion popups, this branch is dead code. No additional gating is required for `gix::credentials`,
but the `git_credentials` module itself (which depends on `keyring` types) must be gated behind
`#[cfg(feature = "native")]`.

---

### 2. `crates/but-settings` — `notify`

**What it does:** `AppSettingsWithDiskSync::watch_in_background()` in `src/watch.rs` uses `notify`
to watch `settings.json` for changes and reload settings when the file is modified. This is a
background daemon capability.

**WASI problem:** `notify = "8.2.0"` uses `RecommendedWatcher`, which compiles to inotify on Linux,
kqueue/FSEvents on macOS, and ReadDirectoryChangesW on Windows. All require OS kernel APIs
unavailable in WASI.

**Callers of `watch_in_background`:**
- `gitbutler-tauri/src/settings.rs` — Tauri app subscribes to live reload (not in WASI dep tree)
- `but-server/src/lib.rs` — `but-server` subscribes (not in WASI dep tree)
- `but-api/src/legacy/settings.rs` — called by legacy settings API
- `crates/but/src/command/config.rs` — CLI `config watch` subcommand

For WASI, `watch_in_background` should be a no-op stub: the host environment does not provide
file-system notification events, and `but` CLI running as a WASI component does not need
live settings reload (each invocation is stateless).

**Cargo.toml changes needed:**
```toml
# crates/but-settings/Cargo.toml
[features]
native = ["dep:notify"]

[dependencies]
notify = { workspace = true, optional = true }
```

`AppSettingsWithDiskSync::watch_in_background` gets a `#[cfg(feature = "native")]` body vs. a
no-op stub returning `Ok(())`.

---

### 3. `crates/but-path` — `dirs` and `open`

**What it does:**
- `dirs` is used in all four path functions: `app_data_dir()`, `app_log_dir()`, `app_config_dir()`,
  `app_cache_dir()`. Each already has an early-return escape hatch via `E2E_TEST_APP_DATA_DIR`.
- `open` is used in `AppChannel::open()` to launch the GitButler GUI via a deep-link URL.

**WASI problem:**
- `dirs = "6.0.0"` uses OS-specific APIs (`$HOME`, `XDG_*`, `CSIDL_*`, `NSSearchPathDirectory`, etc.).
  Under WASI there is no home directory.
- `open = "5.3.2"` calls `xdg-open`, `open`, `start`, etc. — subprocesses unavailable from WASI.

**Fallback strategy for WASI dirs:**
All four functions can be rewritten to also check a new env-var `GITBUTLER_APP_DIR` (or per-function
variants) before consulting `dirs`. For WASI, the host sets these env vars to preconfigured paths
inside the WASI sandbox's pre-opened directories. Suggested env vars:
- `GITBUTLER_DATA_DIR` → replaces `dirs::data_dir()`
- `GITBUTLER_LOG_DIR` → replaces `dirs::data_local_dir()` / `dirs::home_dir()` for logs
- `GITBUTLER_CONFIG_DIR` → replaces `dirs::config_dir()`
- `GITBUTLER_CACHE_DIR` → replaces `dirs::cache_dir()`

Each function checks the env var first; if set, returns it directly (without appending the identifier
suffix since the host can be specific). Falls back to `dirs::*` when the feature is native.

**Fallback for `open` / `AppChannel::open()`:**
Under WASI `AppChannel::open()` should return an `Err` immediately with a clear message:
"Cannot open browser from WASI context." The callers of `AppChannel::open()` are inside the
`but` CLI's skill/config commands, which are `legacy`-gated and will not be present in the WASI build.

**Cargo.toml changes needed:**
```toml
# crates/but-path/Cargo.toml
[features]
native = ["dep:dirs", "dep:open"]

[dependencies]
dirs = { version = "6.0.0", optional = true }
open = { workspace = true, optional = true }
```

---

### 4. `crates/but` — `dirs`, `machine-uid`, `posthog-rs`, `git2`

**`dirs` in `crates/but/src/command/skill.rs`:**
Used in 5+ call sites to locate `~/.claude/` and `~` for skill file management.
All usages are in `skill.rs` commands that are part of the `but` CLI's tooling, not the WASI MCP
server. The skill command is reachable from `but` subcommands. Under WASI (MCP mode), these skill
commands must be disabled. They can be gated behind `cfg(feature = "native")` in `Cargo.toml`.

**`machine-uid` in `crates/but/src/utils/metrics.rs`:**
Used in `fn machine() -> String` which hashes machine ID + "gitbutler" to create a stable device
identifier for PostHog telemetry. Under WASI there is no machine identity concept.

**`posthog-rs` in `crates/but/src/utils/metrics.rs`:**
Telemetry is meaningless for a server-side WASI component. The entire metrics module can be gated
behind `cfg(feature = "native")`. Callers of `capture_event_blocking` must also be gated.

**`git2` in `crates/but/Cargo.toml`:**
`git2` is an unconditional dependency in `crates/but/Cargo.toml` (used for libgit2 operations in
legacy commands). For WASI, `git2` requires C compilation of libgit2 which is not WASI-compatible.
It must be made optional under the `native` feature.

**Cargo.toml changes needed:**
```toml
# crates/but/Cargo.toml
[features]
native = [
    "dep:machine-uid",
    "dep:posthog-rs",
    "dep:git2",
    "but-secret/native",
    "but-settings/native",
    "but-path/native",
    "but-update/native",
    "but-installer/native",
]

[dependencies]
machine-uid = { version = "0.5.4", optional = true }
posthog-rs = { ..., optional = true }
git2 = { workspace = true, optional = true }
dirs = { workspace = true, optional = true }
```

---

### 5. `crates/but-update` — `machine-uid`

**What it does:** `fn install() -> Option<String>` in `src/check.rs` calls `machine_uid::get()`
to create a stable install identifier hashed into a hex string. This is sent to the GitButler
update server as telemetry.

**WASI problem:** `machine-uid = "0.5.4"` reads machine-specific IDs from `/etc/machine-id`,
Windows registry, or IOKit on macOS — all unavailable in WASI.

**Fallback for WASI:** `fn install()` returns `None` when `machine-uid` is not available.
The function already returns `Option<String>`, so the WASI stub simply returns `None`.
Additionally, the `but_secret::secret::retrieve("gitbutler_access_token", ...)` call in
`check.rs` must route through the WASI env-var backend (handled by s07 gating in `but-secret`).

**Cargo.toml changes needed:**
```toml
# crates/but-update/Cargo.toml
[features]
native = ["dep:machine-uid"]

[dependencies]
machine-uid = { version = "0.5.4", optional = true }
```

---

### 6. `crates/but-api` — `open`

**What it does:** `src/legacy/open.rs` contains `open_url()`, `open_in_terminal()`, and
`show_in_finder()`. All three call `open::commands()` or spawn platform-specific processes.
`open_that()` is an internal helper.

**WASI problem:** The `open` crate shells out to `xdg-open` / macOS `open` / Windows `start`.
Spawning subprocesses from WASI is generally not supported without explicit host capabilities.
Even if process spawning were available, there is no display environment to open a browser/terminal in.

**`open_in_terminal` and `show_in_finder`:** these use `cfg!(target_os = "macos")`,
`cfg!(target_os = "linux")`, `cfg!(windows)` — all will evaluate to false under WASI, falling
through to `bail!("Unsupported platform")` or similar. However `open::commands()` still prevents
compilation.

**Fallback for WASI:** The entire `src/legacy/open.rs` module is gated behind the `legacy` feature,
which itself requires `native` features in the WASI build. When `legacy` is disabled (WASI mode),
`open` is never compiled in.

However, `open` is also an unconditional dependency in `but-api/Cargo.toml` (line 132) and
`but-path/Cargo.toml`. The `but-api` unconditional use must be made optional.

**Cargo.toml changes needed:**
```toml
# crates/but-api/Cargo.toml
[dependencies]
open = { workspace = true, optional = true }

[features]
legacy = [
    ...,
    "dep:open",   # open::commands() only used in legacy/open.rs
]
```

---

### 7. `crates/but-claude` — `dirs`

**What it does:** Uses `dirs::home_dir()` in four places: `src/lib.rs` (global claude dir),
`src/claude_transcript.rs` (session transcripts), `src/claude_mcp.rs` (`.claude.json`),
`src/claude_sub_agents.rs` (`.claude/agents/`), and `src/claude_settings.rs` (`.claude/settings.json`).

**WASI scope:** `but-claude` is an optional dependency of `but` under the `legacy` feature, which
is excluded in WASI builds. Therefore `but-claude`'s `dirs` usage does not affect WASI compilation
directly. No gating is needed in this sub-PR.

---

### 8. `crates/but-installer` — `dirs`

**What it does:** `src/config.rs` uses `dirs::home_dir()` to determine where to install the binary
(`~/.local/bin/` on Linux, etc.), and `env::consts::OS` / `env::consts::ARCH` to detect platform.

**WASI scope:** `but-installer` is an unconditional dependency of `crates/but`. The update install
subcommand is already gated `#[cfg(unix)]`. Under WASI (which is not unix), the install path is
unreachable. However, `but-installer` still compiles as a dependency, and `dirs::home_dir()` in
`config.rs` will fail to resolve at compile time if `dirs` is not available.

**Cargo.toml changes needed:**
```toml
# crates/but-installer/Cargo.toml
[features]
native = ["dep:dirs"]

[dependencies]
dirs = { workspace = true, optional = true }
```

And `config.rs` wraps home_dir in `#[cfg(feature = "native")]`.

---

### 9. `gitbutler-user` — `keyring` (indirect, via `but-secret`)

`gitbutler-user` depends on `keyring` in dev-dependencies and directly uses `but-secret`'s
`secret::retrieve`. Since `gitbutler-user` is in the `legacy` feature tree and excluded from WASI
builds, no additional gating is needed in this crate itself. However, `gitbutler-user/Cargo.toml`
line 29 lists `keyring.workspace = true` as a direct (non-dev) dependency — this may need to
be made optional or removed if it's only used in tests.

---

### 10. `gitbutler-testsupport` and `but-testing` — `keyring` / `dirs-next`

Both are test-support crates (`gitbutler-testsupport/Cargo.toml:34` and `but-testing/Cargo.toml:51`)
that reference `keyring` and `dirs-next`. These are not compiled into the WASI binary; they are only
used in `[dev-dependencies]` or test crates. No changes needed for this sub-PR.

---

### 11. `gitbutler-cli` — `dirs-next`

`crates/gitbutler-cli/Cargo.toml:36` uses `dirs-next = "2.0.0"`. This is the legacy Tauri CLI
crate, not part of the WASI dep tree. No changes needed.

---

### 12. `gix::credentials` — shell-out risk

`gix::credentials::helper::Cascade::invoke()` in `but-secret/src/secret.rs` calls into git
credential helpers, which shell out via `std::process::Command`. This code path is only reached
from `git_credentials::setup()`, which is only called when `cfg!(debug_assertions) && cfg!(target_os = "macos")`.
Under WASI neither condition is true, so the git_credentials module is dead code. The module
references `keyring` types throughout, so it must be gated inside `#[cfg(feature = "native")]`.

---

## Cargo.toml Change Summary

| Crate | Dep | Change |
|---|---|---|
| `but-secret` | `keyring` | `optional = true`, new `native` feature |
| `but-settings` | `notify` | `optional = true`, new `native` feature |
| `but-path` | `dirs`, `open` | `optional = true`, new `native` feature |
| `but-update` | `machine-uid` | `optional = true`, new `native` feature |
| `but-api` | `open` | `optional = true`, add to `legacy` feature |
| `but-installer` | `dirs` | `optional = true`, new `native` feature |
| `crates/but` | `machine-uid`, `posthog-rs`, `git2`, `dirs` | `optional = true`, all under `native` feature |
| `crates/but` | `native` feature | propagates `but-secret/native`, `but-path/native`, `but-settings/native`, `but-update/native`, `but-installer/native` |

## Env-Var Fallback Naming Convention

All WASI secret env vars follow: `GITBUTLER_SECRET_{NAMESPACE}_{HANDLE}`

Where:
- `{NAMESPACE}` is the namespace prefix (e.g., `COM_GITBUTLER_APP`, `GITBUTLER` for global)
- `{HANDLE}` is the secret handle with `.` and `-` replaced by `_`, uppercased

Examples:
| Secret handle | Namespace | Env var |
|---|---|---|
| `gitbutler_access_token` | BuildKind (`com.gitbutler.app`) | `GITBUTLER_SECRET_COM_GITBUTLER_APP_GITBUTLER_ACCESS_TOKEN` |
| `github_access_token` | BuildKind | `GITBUTLER_SECRET_COM_GITBUTLER_APP_GITHUB_ACCESS_TOKEN` |
| `aiOpenAIKey` | Global | `GITBUTLER_SECRET_GITBUTLER_AIOPENAIKEY` |
| `aiAnthropicKey` | Global | `GITBUTLER_SECRET_GITBUTLER_AIANTHROPICKEY` |
| `github_oauth_<username>` | BuildKind | `GITBUTLER_SECRET_COM_GITBUTLER_APP_GITHUB_OAUTH_<USERNAME>` |

WASI `persist()` and `delete()` are no-ops returning `Ok(())`.

## Dir Env-Var Fallbacks

| Function | Env var override | Fallback (native only) |
|---|---|---|
| `app_data_dir()` | `GITBUTLER_DATA_DIR` | `dirs::data_dir()` + identifier |
| `app_log_dir()` | `GITBUTLER_LOG_DIR` | platform-specific via `dirs::*` |
| `app_config_dir()` | `GITBUTLER_CONFIG_DIR` | `dirs::config_dir()` + "gitbutler" |
| `app_cache_dir()` | `GITBUTLER_CACHE_DIR` | `dirs::cache_dir()` + identifier |

Note: `E2E_TEST_APP_DATA_DIR` already handles the test override pattern — the WASI env vars follow
the same idiom.

## Acceptance Criteria

- [ ] `keyring` is optional in `but-secret`; gated by `native` feature
- [ ] `but-secret::secret` module compiles for `wasm32-wasip2` with env-var fallback
- [ ] `but-secret::git_credentials` module gated behind `#[cfg(feature = "native")]`
- [ ] `notify` is optional in `but-settings`; `watch_in_background` is a no-op stub under WASI
- [ ] `dirs` is optional in `but-path`; all four path functions fall back to env vars under WASI
- [ ] `open` is optional in `but-path` and `but-api`; gated via `native`/`legacy` features
- [ ] `machine-uid` is optional in `but-update` and `crates/but`; `install()` returns `None` in WASI
- [ ] `posthog-rs` is optional in `crates/but`; metrics module disabled under WASI
- [ ] `git2` is optional in `crates/but`; gated under `native`
- [ ] `crates/but` with `--no-default-features --features wasi` compiles for `wasm32-wasip2`
- [ ] Native build with `--features native` compiles and passes existing tests unmodified
- [ ] WASI secrets env-var naming scheme documented and verified by an integration test
