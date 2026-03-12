# Memory: wasi-ci-pipeline (s16)

## Status: plan-complete

## Errors & Fixes

## Decisions

- **Single job, no matrix:** One `ubuntu-latest` job covers all steps. macOS is optional and deferred — not needed for initial CI correctness.
- **No reusable action:** Only one workflow uses WASI setup. Extract `.github/actions/init-env-wasi/` only if a second workflow requires it.
- **No WASI SDK:** `wasm32-wasip2` target ships with the Rust toolchain. The separate wasi-sdk (C libc) is not needed since C deps (git2, rusqlite) are gated out in earlier sub-PRs.
- **Size threshold: 20 MB release.** Conservative starting point. Tighten once a real baseline is measured. Debug build size is not enforced.
- **`wasm-opt` deferred:** binaryen's `wasm-opt -Os` can reduce binary size by 20–40% but adds a build step. Defer until after baseline is established.
- **Artifact: release binary only.** Upload `but.wasm` from the release build, not debug.
- **Cache saved only on `feat/wasi` pushes** to avoid cache thrashing from `pr1/**` branches.
- **`persist-credentials: false` is mandatory** on all checkout steps — the existing `check-no-persist-credentials` job in `push.yaml` scans all workflow files and will fail if missing.
- **All action refs must be SHA-pinned** to match the repo convention (push.yaml uses pinned SHAs throughout).

## Blockers

- **`bytecodealliance/actions/wasmtime/setup@v1` SHA** must be looked up at implementation time. The floating `@v1` tag cannot be used as the final value (convention violation). Not a blocker for planning.
- **s15 test command surface** is not yet implemented (s15 status: not-started). The plan assumes `cargo test -p but-wasi-host --test '*'` and `cargo test -p but --test 'wasi*'` — verify against s15 output at implementation time.
