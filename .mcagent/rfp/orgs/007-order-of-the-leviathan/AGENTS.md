# Order of the Leviathan — Agent Roster

**3 agents. Sequential operation. No concurrency. Minimal footprint.**

---

## Team Structure

The Order assigns three agents to every task, corresponding to the three phases of their Rule: Observe, Act, Verify. The agents operate in strict sequence. The second agent does not begin until the first has finished. The third does not begin until the second has finished. This is slower than parallel execution. The Order considers speed a false economy.

## Roles

- **The Reader** — Observes. Reads the task description, retrieves relevant memory, examines the workspace state, and produces a written specification of what must be done. The specification is terse — the Reader uses as few tokens as possible. Surplus context is waste.
- **The Scribe** — Acts. Takes the Reader's specification and produces `INDEX.patch` + `COMMIT.msg`. The Scribe does not read the original task — only the Reader's specification. This enforces a clean separation: the Reader interprets; the Scribe implements.
- **The Warden** — Verifies. Reviews the Scribe's patch against the Reader's specification. Checks for correctness, compliance with branch authorization policy, and adherence to the Order's frugality standards (patch is not unnecessarily large, commit message is not unnecessarily verbose). If verification passes, the Warden signs the commit via OpenWallet.

## Working Dynamic

Communication between agents is unidirectional: Reader → Scribe → Warden. There is no back-channel. If the Warden rejects a patch, the entire cycle restarts from the Reader. The Order believes that revision is costlier than restarting with fresh context — a rejected patch means the Reader's specification was inadequate, not that the Scribe made a typo.

This has been challenged internally. Critics note that restarting costs 100% of the budget already spent. The Order responds that a specification good enough to produce a correct patch on the first attempt costs less than a specification that requires three revision cycles.

## Token Budget Summary

| Role | Input | Output |
|------|-------|--------|
| Reader | 4,500 | 1,200 |
| Scribe | 3,000 | 4,500 |
| Warden | 4,000 | 600 |
| **Team Total** | **11,500** | **6,300** |

This is the leanest budget among all proposing organizations. The Order considers this a point of pride.

## Failure Mode

The team fails when the Reader produces an ambiguous specification. The Scribe interprets ambiguity in the most literal way possible, which sometimes means producing a technically correct patch that misses the task's intent. The Warden, comparing patch against specification, finds no fault — because the specification was the problem, not the implementation.

Recovery: the Warden has a "sense check" step where it compares the patch against the original task description (consuming additional tokens). If the patch satisfies the specification but not the task, the Warden flags it as `SPECIFICATION_MISMATCH` and the cycle restarts.
