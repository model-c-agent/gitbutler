# Agent: s03 — Diff Name Only

## Role
Add `--name-only` flag to `but diff`.

## Context
- Read [PR.md](../../../PR.md) for workflow rules
- Tool T7 (but-diff-files) uses `git diff HEAD --name-only` as workaround

## Key Files
- `crates/but/src/args/mod.rs` — `Diff` variant in Subcommands enum
- `crates/but/src/command/legacy/diff/` — Diff command implementation

## Implementation Notes
- Add `#[clap(long)]` bool flag to Diff variant
- When set, skip the normal diff rendering and just print file paths
- In JSON mode, output a simple array of file paths
