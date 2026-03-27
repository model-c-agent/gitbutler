# Memory: wasi-feature-flags (s01)

## Status: complete

## Errors & Fixes

### 2026-03-12 -- Bash denied in sub-agent, coordinator finished staging
**Context:** Sub-agent made file edits but lacked bash permissions. Coordinator completed verification and staging.

### 2026-03-12 -- Stacking null object errors
**Context:** Committing to branches stacked on 0-commit parents produces "object with id 000...000 not found". Workaround: use standalone branches for commits, not stacked ones.

## Decisions

### 2026-03-12 -- Marker features start empty
**Context:** Considered pre-populating `tui = ["dep:ratatui", ...]` and `native = ["dep:git2", ...]` in this sub-PR.
**Decision:** Keep marker features empty (`tui = []`, `native = []`, `wasi = []`). Each tier-1 sub-PR (s04 for TUI, s05-s07 for native deps) will add the actual dependency gates.
**Alternatives considered:** Pre-populating would cause compilation issues because the deps aren't yet made optional. Doing it incrementally per sub-PR is safer and allows parallel work.

## Blockers
