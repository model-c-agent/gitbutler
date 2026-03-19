# PR #2: `but` CLI Improvements

## Motivation

During PR #1 (WASI compilation), the `but` CLI exhibited 6 failure modes (F1-F6) and required 10 bespoke workaround tools (T1-T10). These are documented in [HISTORY.md](../HISTORY.md) and [RETRO.md](../RETRO.md). PR #2 addresses these issues directly in the `but` CLI Rust source code.

## Goals

1. **Plugin system** — Cargo-style `but-<name>` plugin discovery makes all existing shell tools first-class subcommands
2. **Bug fixes** — Silent null commit IDs (F1, F3), commit ID instability (F6)
3. **Missing commands** — `but diff --name-only`, `but branch rename`, `but sync pause/resume`
4. **Batch operations** — Multi-ID staging, multi-branch apply/unapply
5. **UX improvements** — Human-readable status, branch list with pattern/format, branch delete by pattern
6. **Safety** — Hunk lock override flag for explicit reassignment

## Sub-PR Breakdown

### Tier 0: Root (no dependencies)

| ID | Folder | Description | Size | Fixes |
|----|--------|-------------|------|-------|
| s00 | `s00/feat/plugin-system` | Cargo-style plugin system: `but-<name>` on PATH → `but <name>` | M | T1-T10 |
| s01 | `s01/fix/commit-null-id-error` | Fix silent null commit IDs — error instead of "unknown" | S | F1, F3 |
| s02 | `s02/feat/mutation-return-new-ids` | Return new commit IDs from amend/reword/squash/absorb | M | F6, T8 |
| s03 | `s03/feat/diff-name-only` | Add `but diff --name-only` flag | S | T7 |
| s11 | `s11/feat/sync-pause-resume` | Add `but sync pause`/`resume` subcommand | M | F4 |

### Tier 1: Depends on Tier 0

| ID | Folder | Description | Size | Fixes |
|----|--------|-------------|------|-------|
| s04 | `s01.s04/feat/stage-override-lock` | Add `--override-lock` to `but stage` | M | F2 |
| s05 | `s02.s05/feat/batch-stage` | Accept comma-separated IDs in `but stage` | M | T3 |
| s06 | `s02.s06/feat/batch-apply-unapply` | Accept multiple branches in `but apply`/`unapply` | M | T1, T2 |

### Tier 2: Depends on Tier 1

| ID | Folder | Description | Size | Fixes |
|----|--------|-------------|------|-------|
| s07 | `s03.s07/feat/branch-rename` | Add `but branch rename <old> <new>` | S | F5 |
| s08 | `s04.s05.s08/feat/human-status-improvements` | Summary header + file counts in `but status` | L | T4-T6, T9 |

### Tier 3: Depends on Tier 2

| ID | Folder | Description | Size | Fixes |
|----|--------|-------------|------|-------|
| s09 | `s07.s09/feat/branch-list-pattern-format` | `--pattern` and `--format table` for `but branch list` | M | T5 |
| s10 | `s08.s10/feat/branch-delete-pattern` | `but branch delete --pattern <regex>` | S | T10 |

## Dependency Graph

```
s00 (plugin)   s01 (null commit)   s02 (mutation IDs)   s03 (diff)   s11 (sync)
                 │                   │  │                  │
                 └── s04             │  │                  │
                                     │  │                  │
                              s05 ◄──┘  │                  │
                                     │  │                  │
                              s06 ◄──┘  │                  │
                                        │                  │
                           s08 ◄── s04, s05                │
                                        │                  │
                                 s07 ◄──┘                  │
                                        │                  │
                                 s09 ◄──┘                  │
                                        │                  │
                                 s10 ◄──┘                  │
```

## Acceptance Criteria

- All 12 sub-PRs pass `cargo check -p but`, `cargo clippy -p but -- -D warnings`, `cargo test -p but`
- Plugin system discovers existing `scripts/bin/but-*` tools
- Null commit IDs produce errors instead of silent "unknown"
- Mutation commands return new commit IDs in JSON output
