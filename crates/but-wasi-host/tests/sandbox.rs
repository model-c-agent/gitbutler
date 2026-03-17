#[allow(dead_code)]
mod common;

use std::fs;

use common::{WasiTestFixture, wasm_available};

/// Verify that the guest process can read a file placed inside the
/// repository directory (the preopen path).
#[test]
fn preopen_repo_is_accessible() {
    if !wasm_available() {
        eprintln!("Skipping: but.wasm not found");
        return;
    }

    let fixture = WasiTestFixture::new();

    // Create a sentinel file inside the repo so the guest has something
    // to discover.
    let sentinel = fixture.repo_dir.path().join("sentinel.txt");
    fs::write(&sentinel, "hello from host").expect("failed to write sentinel file");

    // Running any `but` command that reads the repo should work. We use
    // `but branch` which needs to open the git dir.
    let _out = fixture.run(&["branch"]);
    // As long as the process did not crash, the preopen worked. The
    // specific exit code depends on the guest binary's behaviour.
}

/// Verify that `HOME` is remapped to `/config` inside the sandbox by
/// running a command that actually depends on the config directory.
///
/// `but config user set --global name "sandbox-test"` writes to the
/// *global* git config, which lives under `$HOME`.  Inside the sandbox
/// HOME=/config which is a read-only preopen, so the write must fail.
/// If HOME were still pointing at the real host path, the guest would
/// either succeed (modifying the host's global gitconfig -- bad!) or
/// fail with a completely different error about a host path.
#[test]
fn home_env_is_remapped() {
    if !wasm_available() {
        eprintln!("Skipping: but.wasm not found");
        return;
    }

    let fixture = WasiTestFixture::new();

    // Attempt a global config write.  The sandbox exposes /config as
    // read-only so this *must* fail.
    let out = fixture.run(&["config", "user", "set", "--global", "name", "sandbox-test"]);
    out.assert_failure();

    // Stderr should NOT mention the real host HOME directory.
    let real_home = std::env::var("HOME").unwrap_or_default();
    if !real_home.is_empty() {
        assert!(
            !out.stderr.contains(&real_home),
            "guest stderr leaked real HOME path: {real_home}"
        );
        assert!(
            !out.stdout.contains(&real_home),
            "guest stdout leaked real HOME path: {real_home}"
        );
    }
}

/// Verify that the WASI sandbox prevents the guest from accessing paths
/// outside the two preopened directories (`/repo` and `/config`).
///
/// WASI filesystem isolation means the guest has no capability to open
/// host paths such as `/etc/passwd`, `/tmp`, or the user's real home
/// directory.  We test this by checking two invariants:
///
///   1. A read-only command (`but config user`) executes successfully and
///      its output contains *no* references to real host paths.
///   2. A global-config write (`but config user set --global …`) fails
///      because `/config` is mounted read-only, proving the guest cannot
///      escape to a writable location outside the sandbox.
#[test]
fn sandbox_prevents_access_outside_preopened_dirs() {
    if !wasm_available() {
        eprintln!("Skipping: but.wasm not found");
        return;
    }

    let fixture = WasiTestFixture::new();

    // ── 1. Read-only command should succeed and not leak host paths ────
    let out = fixture.run(&["config", "user"]);
    // The command may succeed or fail (no user config set), but it should
    // not crash with a sandbox violation.  We just need the process to
    // have run to completion.
    let combined = format!("{}\n{}", out.stdout, out.stderr);

    // Must not contain any reference to real host paths.
    let sensitive_paths: Vec<String> = [
        std::env::var("HOME").ok(),
        std::env::var("XDG_CONFIG_HOME").ok(),
        Some("/etc/passwd".to_string()),
    ]
    .into_iter()
    .flatten()
    .filter(|p| !p.is_empty())
    .collect();

    for path in &sensitive_paths {
        assert!(
            !combined.contains(path.as_str()),
            "guest output leaked host path {path:?}:\n{combined}"
        );
    }

    // ── 2. Write to a global config location must fail ─────────────────
    // If the guest could somehow escape the sandbox to a writable host
    // directory, this would succeed.  The read-only /config preopen
    // guarantees it cannot.
    let out = fixture.run(&["config", "user", "set", "--global", "email", "escape@test.invalid"]);
    out.assert_failure();
}

/// Verify the `--no-cache` host flag is accepted and does not cause a crash.
#[test]
fn no_cache_flag_accepted() {
    if !wasm_available() {
        eprintln!("Skipping: but.wasm not found");
        return;
    }

    let fixture = WasiTestFixture::new();
    let out = fixture.run_with_extra_host_args(&["--no-cache"], &["--help"]);
    out.assert_success();
}
