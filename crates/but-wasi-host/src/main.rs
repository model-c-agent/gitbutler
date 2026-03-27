mod sandbox;

use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;

/// WASI host for running `but.wasm` inside a Wasmtime sandbox.
#[derive(Parser, Debug)]
#[command(name = "but-wasi", about = "Run but CLI inside a WASI sandbox")]
struct Cli {
    /// Path to the git repository to expose to the guest.
    #[arg(long, env = "BUT_WASI_REPO")]
    repo: PathBuf,

    /// Path to the config directory exposed read-only to the guest.
    /// Defaults to `<config_dir>/gitbutler`.
    #[arg(long, env = "BUT_WASI_CONFIG")]
    config: Option<PathBuf>,

    /// Path to the `.wasm` component module.
    /// Defaults to `but.wasm` next to this binary.
    #[arg(long, env = "BUT_WASI_MODULE")]
    module: Option<PathBuf>,

    /// Directory used for caching pre-compiled (`.cwasm`) artifacts.
    #[arg(long, env = "BUT_WASI_CACHE_DIR")]
    cache_dir: Option<PathBuf>,

    /// Reserved for future network access support (not yet implemented).
    #[arg(long, default_value_t = false)]
    allow_network: bool,

    /// Disable AOT pre-compilation cache.
    #[arg(long, default_value_t = false)]
    no_cache: bool,

    /// Arguments forwarded to the guest `but` binary (pass after `--`).
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    but_args: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn")),
        )
        .with_writer(std::io::stderr)
        .init();

    match run(cli) {
        Ok(code) => std::process::exit(code),
        Err(err) => {
            eprintln!("error: {err:#}");
            std::process::exit(1);
        }
    }
}

fn run(cli: Cli) -> Result<i32> {
    if cli.allow_network {
        anyhow::bail!(
            "--allow-network is not yet implemented; network access support is planned for a future release"
        );
    }

    let repo_path = &cli.repo;
    if !repo_path.exists() {
        anyhow::bail!(
            "repository path does not exist: {}",
            repo_path.display()
        );
    }

    // Canonicalize the repo path to resolve symlinks, preventing symlink
    // traversal attacks where a symlink could preopen unintended directories.
    let repo_path = std::fs::canonicalize(repo_path).with_context(|| {
        format!(
            "failed to canonicalize repository path: {}",
            repo_path.display()
        )
    })?;

    let config_path = match cli.config {
        Some(path) => path,
        None => dirs::config_dir()
            .context("unable to determine platform config directory")?
            .join("gitbutler"),
    };

    let module_path = cli.module.unwrap_or_else(|| {
        let exe = std::env::current_exe().expect("unable to determine current executable path");
        exe.parent()
            .expect("executable has no parent directory")
            .join("but.wasm")
    });

    if !module_path.exists() {
        anyhow::bail!(
            "WASM module not found: {}\n\
             hint: build it with `cargo build -p but --target wasm32-wasip2` or pass --module",
            module_path.display()
        );
    }

    let opts = sandbox::SandboxOptions {
        repo_path,
        config_path,
        module_path,
        cache_dir: cli.cache_dir,
        no_cache: cli.no_cache,
    };

    sandbox::run(opts, cli.but_args)
}
