# Agent: s06 — Batch Apply/Unapply

## Role
Add multi-branch support to `but apply` and `but unapply`.

## Context
- Read [PR.md](../../../../PR.md) for workflow rules
- Tools T1 (but-apply-pattern) and T2 (but-unapply-pattern) are workarounds

## Key Files
- `crates/but/src/args/mod.rs` — `Apply` and `Unapply` variants
- `crates/but/src/command/legacy/branch/apply.rs`
- `crates/but/src/command/legacy/unapply.rs`
