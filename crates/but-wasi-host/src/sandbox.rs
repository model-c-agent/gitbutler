use std::path::PathBuf;

use anyhow::{Context, Result};
use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store,
};
use wasmtime_wasi::{DirPerms, FilePerms, WasiCtx, WasiCtxBuilder, WasiView};

/// Env vars that are safe to forward into the sandbox.
/// Exact names are matched literally; prefixes are matched with `starts_with`.
/// We explicitly exclude any variable whose name contains `_TOKEN`, `_KEY`, or
/// `_SECRET` to avoid leaking credentials even if a new `GITBUTLER_*` var is
/// introduced on the host.
const ALLOWED_ENV_EXACT: &[&str] = &["RUST_LOG"];
const ALLOWED_ENV_PREFIXES: &[&str] = &[
    "GITBUTLER_DATA_DIR",
    "GITBUTLER_CONFIG_DIR",
    "GITBUTLER_LOG_DIR",
    "GITBUTLER_LOG_",
    "GITBUTLER_APP_",
];
/// Suffixes that must never be forwarded, even if a prefix matches.
const DENIED_ENV_SUFFIXES: &[&str] = &["_TOKEN", "_KEY", "_SECRET", "_PASSWORD", "_CREDENTIAL"];

/// Options controlling the WASI sandbox environment.
pub struct SandboxOptions {
    pub repo_path: PathBuf,
    pub config_path: PathBuf,
    pub module_path: PathBuf,
    pub cache_dir: Option<PathBuf>,
    pub no_cache: bool,
}

/// Internal host state handed to the Wasmtime store.
struct HostState {
    ctx: WasiCtx,
    table: wasmtime::component::ResourceTable,
}

impl WasiView for HostState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
    fn table(&mut self) -> &mut wasmtime::component::ResourceTable {
        &mut self.table
    }
}

/// Run the `but.wasm` component inside a Wasmtime WASI sandbox.
///
/// Returns the guest process exit code.
pub fn run(opts: SandboxOptions, but_args: Vec<String>) -> Result<i32> {
    // ── Engine ──────────────────────────────────────────────────────────
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.cranelift_opt_level(wasmtime::OptLevel::Speed);

    let engine = Engine::new(&config).context("failed to create wasmtime engine")?;

    // ── Component (with optional AOT cache) ────────────────────────────
    let component = load_component(&engine, &opts)?;

    // ── WASI context ───────────────────────────────────────────────────
    let mut builder = WasiCtxBuilder::new();

    // Argv: guest sees `but <args…>`
    let mut argv = vec!["but".to_string()];
    argv.extend(but_args);
    builder.args(&argv);

    // Preopened directories
    builder
        .preopened_dir(&opts.repo_path, "/repo", DirPerms::all(), FilePerms::all())
        .context("failed to preopen repo directory")?;
    builder
        .preopened_dir(
            &opts.config_path,
            "/config",
            DirPerms::READ,
            FilePerms::READ,
        )
        .context("failed to preopen config directory")?;

    // Inherit I/O
    builder.inherit_stdout();
    builder.inherit_stderr();

    // Environment variables: forward only explicitly allowed vars into the sandbox.
    // Secret-bearing variables (_TOKEN, _KEY, _SECRET, etc.) are always excluded.
    for (key, value) in std::env::vars() {
        if is_env_allowed(&key) {
            builder.env(&key, &value);
        }
    }
    builder.env("HOME", "/config");

    let wasi_ctx = builder.build();

    // ── Store + linker ─────────────────────────────────────────────────
    let mut store = Store::new(
        &engine,
        HostState {
            ctx: wasi_ctx,
            table: wasmtime::component::ResourceTable::new(),
        },
    );

    let mut linker: Linker<HostState> = Linker::new(&engine);
    wasmtime_wasi::add_to_linker_sync(&mut linker).context("failed to add WASI to linker")?;

    // ── Instantiate and run ────────────────────────────────────────────
    let command = wasmtime_wasi::bindings::sync::Command::instantiate(&mut store, &component, &linker)
        .context("failed to instantiate WASI command component")?;

    let result = command
        .wasi_cli_run()
        .call_run(&mut store)
        .context("failed to call wasi:cli/run")?;

    match result {
        Ok(()) => Ok(0),
        Err(()) => Ok(1),
    }
}

/// Load a WASM component, using an AOT `.cwasm` cache when available.
fn load_component(engine: &Engine, opts: &SandboxOptions) -> Result<Component> {
    let wasm_path = &opts.module_path;
    let cwasm_path = match &opts.cache_dir {
        Some(dir) => {
            let file_name = wasm_path
                .file_stem()
                .unwrap_or_else(|| std::ffi::OsStr::new("but"))
                .to_owned();
            let mut name = file_name;
            name.push(".cwasm");
            dir.join(name)
        }
        None => wasm_path.with_extension("cwasm"),
    };

    // Try AOT cache first (unless --no-cache)
    if !opts.no_cache && cwasm_path.exists() {
        // Only use cache if the .cwasm is newer than the .wasm
        let wasm_mtime = std::fs::metadata(wasm_path)
            .and_then(|m| m.modified())
            .ok();
        let cwasm_mtime = std::fs::metadata(&cwasm_path)
            .and_then(|m| m.modified())
            .ok();

        if let (Some(wasm_t), Some(cwasm_t)) = (wasm_mtime, cwasm_mtime)
            && cwasm_t >= wasm_t
        {
            tracing::info!("loading AOT-cached component from {}", cwasm_path.display());
            // SAFETY: We trust the .cwasm file was produced by this engine version.
            // A version mismatch will surface as an error rather than UB.
            let component = unsafe { Component::deserialize_file(engine, &cwasm_path) }
                .context("failed to deserialize cached .cwasm (try --no-cache)")?;
            return Ok(component);
        }
    }

    // Load from .wasm source
    tracing::info!("loading WASM component from {}", wasm_path.display());
    let wasm_bytes =
        std::fs::read(wasm_path).with_context(|| format!("failed to read {}", wasm_path.display()))?;

    // Pre-compile and cache (unless --no-cache)
    if !opts.no_cache {
        // Ensure the cache directory exists when using a custom cache dir
        if let Some(parent) = cwasm_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        match engine.precompile_component(&wasm_bytes) {
            Ok(serialized) => {
                if let Err(err) = std::fs::write(&cwasm_path, &serialized) {
                    tracing::warn!(
                        "failed to write AOT cache {}: {err}",
                        cwasm_path.display()
                    );
                } else {
                    tracing::info!("wrote AOT cache to {}", cwasm_path.display());
                }
            }
            Err(err) => {
                tracing::warn!("AOT pre-compilation failed, loading interpreted: {err}");
            }
        }
    }

    let component =
        Component::new(engine, &wasm_bytes).context("failed to compile WASM component")?;
    Ok(component)
}

/// Returns `true` if the env var `key` is safe to forward into the sandbox.
fn is_env_allowed(key: &str) -> bool {
    let upper = key.to_uppercase();

    // Never forward anything that looks like a secret.
    for suffix in DENIED_ENV_SUFFIXES {
        if upper.ends_with(suffix) {
            return false;
        }
    }

    // Check exact matches.
    if ALLOWED_ENV_EXACT.contains(&key) {
        return true;
    }

    // Check prefix matches.
    for prefix in ALLOWED_ENV_PREFIXES {
        if key.starts_with(prefix) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allowed_exact_match() {
        assert!(is_env_allowed("RUST_LOG"));
    }

    #[test]
    fn allowed_prefix_match() {
        assert!(is_env_allowed("GITBUTLER_DATA_DIR"));
        assert!(is_env_allowed("GITBUTLER_CONFIG_DIR"));
        assert!(is_env_allowed("GITBUTLER_LOG_DIR"));
        assert!(is_env_allowed("GITBUTLER_LOG_LEVEL"));
        assert!(is_env_allowed("GITBUTLER_APP_VERSION"));
    }

    #[test]
    fn denied_secrets() {
        assert!(!is_env_allowed("GITBUTLER_APP_TOKEN"));
        assert!(!is_env_allowed("GITBUTLER_API_KEY"));
        assert!(!is_env_allowed("GITBUTLER_DATA_SECRET"));
        assert!(!is_env_allowed("SOME_PASSWORD"));
        assert!(!is_env_allowed("MY_CREDENTIAL"));
    }

    #[test]
    fn denied_unknown_vars() {
        assert!(!is_env_allowed("HOME"));
        assert!(!is_env_allowed("PATH"));
        assert!(!is_env_allowed("AWS_ACCESS_KEY_ID"));
        assert!(!is_env_allowed("GITBUTLER_UNKNOWN"));
    }
}
