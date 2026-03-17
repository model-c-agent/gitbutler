#[allow(dead_code)]
mod common;

use common::{wasm_available, WasiTestFixture};

#[test]
fn branch_list_empty_repo() {
    if !wasm_available() {
        eprintln!("Skipping: but.wasm not found");
        return;
    }

    let fixture = WasiTestFixture::new();
    // Running `but branch` in a freshly-init'd repo should not crash.
    // The exact output/exit-code depends on whether the guest can handle
    // an empty repo, but it must not panic or SIGABRT.
    let _out = fixture.run(&["branch"]);
    // We accept any exit code here -- the important thing is that the
    // process terminated normally (which is asserted by `output()` not
    // returning an Err).
}

#[test]
fn config_read() {
    if !wasm_available() {
        eprintln!("Skipping: but.wasm not found");
        return;
    }

    let fixture = WasiTestFixture::new();
    let _out = fixture.run(&["config", "--json"]);
    // The guest may fail if the config directory isn't fully set up,
    // but it must not panic or SIGABRT. We accept any exit code.
}
