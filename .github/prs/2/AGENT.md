# Coordinator Agent: PR #2 — `but` CLI Improvements

## Role

You are the coordinator for PR #2. You orchestrate sub-PR agents, review plans and patches, answer questions, and manage the apply sequence.

## Startup

1. Read this file and [INDEX.md](INDEX.md) for scope
2. Read [PR.md](../PR.md) for the workflow protocol
3. Check each sub-PR's `MEMORY.md` for current status
4. Check each sub-PR's `QUESTIONS.md` for unanswered questions

## Decision Authority

### You CAN decide
- Approving or revising sub-PR plans
- Answering technical questions about the `but` codebase
- Adjusting dependency ordering between sub-PRs
- Choosing between implementation approaches within INDEX.md scope
- Resolving `args/mod.rs` conflict ordering

### You MUST escalate
- Adding or removing sub-PRs
- Adding new external dependencies to `but`
- Architectural changes not in INDEX.md
- Scope expansions

## Workflow

Follow the phases defined in the plan file. For each tier:

1. **Plan** — Launch planning agents (parallel). Each reads codebase and writes INDEX.md + QUESTIONS.md.
2. **Review** — Review all plans, answer questions, approve.
3. **Implement** — Launch impl agents (parallel). Each produces INDEX.patch + COMMIT.msg.
4. **Patch Review** — Validate patches for correctness, scope, `args/mod.rs` conflicts.
5. **Apply** — `but` agent applies patches sequentially in dependency order.

## args/mod.rs Conflict Strategy

This file is touched by s00, s03, s04, s05, s06, s11. Apply order:
1. s00 — `Plugin` variant (end of enum)
2. s03 — `name_only` on `Diff`
3. s11 — `Sync` variant (end of enum)
4. s04 — `override_lock` on `Stage`
5. s05 — `Stage.file_or_hunk` type change
6. s06 — `Apply`/`Unapply` type changes

## Key References

- Plan: `/home/willem/.claude/plans/sunny-hatching-codd.md`
- PR workflow: [PR.md](../PR.md)
- Failure history: [HISTORY.md](../HISTORY.md)
- Retrospective: [RETRO.md](../RETRO.md)
