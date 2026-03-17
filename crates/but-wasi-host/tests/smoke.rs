#[allow(dead_code)]
mod common;

use common::{WasiTestFixture, run_host_only, wasm_available};

#[test]
fn host_help_exits_zero() {
    // The host binary itself should respond to --help regardless of .wasm
    let out = run_host_only(&["--help"]);
    assert_eq!(out.exit_code, 0, "stderr: {}", out.stderr);
    assert!(
        out.stdout.contains("but-wasi") || out.stdout.contains("WASI"),
        "expected help text to mention but-wasi, got:\n{}",
        out.stdout
    );
}

#[test]
fn host_rejects_missing_repo() {
    // When --repo points to a non-existent directory, the host should fail
    // before even touching the .wasm module.
    let out = run_host_only(&["--repo", "/tmp/nonexistent-repo-path-12345"]);
    assert_ne!(out.exit_code, 0);
    assert!(
        out.stderr.contains("error")
            || out.stderr.contains("not exist")
            || out.stderr.contains("required"),
        "expected an error message, got stderr:\n{}",
        out.stderr
    );
}

#[test]
fn host_rejects_missing_module() {
    // Valid repo but missing .wasm module.
    let fixture = WasiTestFixture::new();
    let out = fixture.run_with_extra_host_args(&["--module", "/tmp/nonexistent-module.wasm"], &[]);
    assert_ne!(out.exit_code, 0);
    assert!(
        out.stderr.contains("error") || out.stderr.contains("not found"),
        "expected an error about missing module, got stderr:\n{}",
        out.stderr
    );
}

#[test]
fn wasm_exits_zero_on_help() {
    if !wasm_available() {
        eprintln!("Skipping: but.wasm not found");
        return;
    }

    let fixture = WasiTestFixture::new();
    let out = fixture.run(&["--help"]);
    out.assert_success();
}

#[test]
fn wasm_help_contains_expected_text() {
    if !wasm_available() {
        eprintln!("Skipping: but.wasm not found");
        return;
    }

    let fixture = WasiTestFixture::new();
    let out = fixture.run(&["--help"]);
    out.assert_success();
    assert!(
        out.stdout.contains("Usage") || out.stdout.contains("usage") || out.stdout.contains("but"),
        "expected help output to contain usage info, got:\n{}",
        out.stdout
    );
}

#[test]
fn wasm_exits_nonzero_on_unknown_command() {
    if !wasm_available() {
        eprintln!("Skipping: but.wasm not found");
        return;
    }

    let fixture = WasiTestFixture::new();
    let out = fixture.run(&["this-subcommand-does-not-exist"]);
    out.assert_failure();
}

#[test]
fn wasm_version_flag() {
    if !wasm_available() {
        eprintln!("Skipping: but.wasm not found");
        return;
    }

    let fixture = WasiTestFixture::new();
    let out = fixture.run(&["--version"]);
    out.assert_success();
}
