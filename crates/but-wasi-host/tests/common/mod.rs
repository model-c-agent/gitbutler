use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::LazyLock;

/// Resolved path to the `but.wasm` component module.
///
/// Checks `WASI_WASM_PATH` first, then falls back to
/// `<workspace>/target/wasm32-wasip2/debug/but.wasm`.
static WASM_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    if let Ok(p) = std::env::var("WASI_WASM_PATH") {
        return PathBuf::from(p);
    }
    workspace_root().join("target/wasm32-wasip2/debug/but.wasm")
});

/// Returns `true` when the `.wasm` component exists on disk.
pub fn wasm_available() -> bool {
    WASM_PATH.exists()
}

/// Returns the resolved path to `but.wasm`.
pub fn wasm_path() -> &'static Path {
    &WASM_PATH
}

/// Approximate workspace root by walking up from `CARGO_MANIFEST_DIR`.
fn workspace_root() -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // crates/but-wasi-host -> ../../
    manifest
        .parent()
        .and_then(|p| p.parent())
        .expect("unable to determine workspace root")
        .to_path_buf()
}

/// Captured output from running the `but-wasi` binary.
#[derive(Debug)]
pub struct WasiOutput {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

impl WasiOutput {
    /// Panics when the process did not exit successfully (code 0).
    #[track_caller]
    pub fn assert_success(&self) {
        assert_eq!(
            self.exit_code, 0,
            "expected exit code 0, got {}\nstderr: {}",
            self.exit_code, self.stderr
        );
    }

    /// Panics when the process exited with code 0.
    #[track_caller]
    pub fn assert_failure(&self) {
        assert_ne!(
            self.exit_code, 0,
            "expected non-zero exit code\nstdout: {}",
            self.stdout
        );
    }

    /// Parse stdout as JSON.
    #[allow(dead_code)]
    pub fn stdout_json(&self) -> serde_json::Value {
        serde_json::from_str(&self.stdout).unwrap_or_else(|err| {
            panic!(
                "failed to parse stdout as JSON: {err}\nstdout: {}",
                self.stdout
            )
        })
    }
}

impl From<Output> for WasiOutput {
    fn from(output: Output) -> Self {
        Self {
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
            exit_code: output.status.code().unwrap_or(-1),
        }
    }
}

/// A test fixture that provides a temporary git repository and a helper to
/// run the `but-wasi` binary against it.
pub struct WasiTestFixture {
    pub repo_dir: tempfile::TempDir,
}

impl WasiTestFixture {
    /// Create a new fixture with a bare `git init` repository.
    pub fn new() -> Self {
        let repo_dir = tempfile::tempdir().expect("failed to create temp dir");

        // Initialise a minimal git repository so the guest `but` binary
        // finds a valid repo.
        let status = Command::new("git")
            .args(["init", "--initial-branch=main"])
            .current_dir(repo_dir.path())
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .env("GIT_CONFIG_SYSTEM", "/dev/null")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .expect("failed to run git init");
        assert!(status.success(), "git init failed");

        Self { repo_dir }
    }

    /// Run `but-wasi` with the given arguments forwarded to the guest.
    ///
    /// The `--repo` and `--module` flags are filled in automatically.
    /// `but_args` are the arguments passed *after* `--` to the guest binary.
    pub fn run(&self, but_args: &[&str]) -> WasiOutput {
        self.run_with_extra_host_args(&[], but_args)
    }

    /// Like [`run`](Self::run) but allows passing extra host-level flags
    /// (e.g. `--no-cache`).
    pub fn run_with_extra_host_args(&self, host_args: &[&str], but_args: &[&str]) -> WasiOutput {
        let bin = env!("CARGO_BIN_EXE_but-wasi");

        let mut cmd = Command::new(bin);
        cmd.arg("--repo")
            .arg(self.repo_dir.path())
            .arg("--module")
            .arg(wasm_path());

        for arg in host_args {
            cmd.arg(arg);
        }

        if !but_args.is_empty() {
            cmd.arg("--");
            for arg in but_args {
                cmd.arg(arg);
            }
        }

        let output = cmd.output().expect("failed to execute but-wasi binary");
        WasiOutput::from(output)
    }
}

/// Run `but-wasi` without a repo directory (only host-level flags).
///
/// Useful for tests like `--help` that don't need `--repo`.
pub fn run_host_only(args: &[&str]) -> WasiOutput {
    let bin = env!("CARGO_BIN_EXE_but-wasi");
    let output = Command::new(bin)
        .args(args)
        .output()
        .expect("failed to execute but-wasi binary");
    WasiOutput::from(output)
}
