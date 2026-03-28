# Heritage Garrison Corps — Agent Roster

**6 agents. Squad structure. Clear chain of command.**

---

## Team Structure

The squad operates with military discipline adapted for software. The chain of command establishes clear decision authority, but every agent is empowered to raise concerns and halt operations if they detect a safety issue (an unsigned commit, a budget overrun, a specification mismatch). "Stop work authority" is universal.

## Roles

- **Squad Leader** — Receives tasks, conducts mission analysis (task decomposition), and issues operations orders. Does not produce patches directly. Monitors squad budget and adjusts allocations mid-mission if needed.
- **Combat Engineers (x2)** — Primary implementers. One specializes in patch generation (the "heavy" engineer), the other in testing and validation (the "light" engineer). They can swap roles if the mission requires it.
- **Survey Specialist** — Conducts reconnaissance: reads workspace state, retrieves memory, examines branch topology, and produces a "site survey" (context package) for the combat engineers. Calls `GetProjectStatus`, `GetBranchChanges`, and `GetCommitDetails` extensively.
- **Fortification Specialist** — Manages all security operations: OpenWallet key lifecycle, commit signing, authorization policy enforcement. Also handles WASI compatibility assessment ("terrain analysis" in Corps terminology).
- **Signals Specialist** — Manages all external communications: forge adapter calls, PR creation, cross-repo coordination, status reporting. The squad's interface to the outside world.

## Working Dynamic

Operations follow a modified military planning cycle:

1. **Warning Order** — Squad leader briefs the team on incoming task
2. **Reconnaissance** — Survey specialist gathers context
3. **Operations Order** — Squad leader issues detailed plan based on recon
4. **Execution** — Combat engineers implement; signals handles coordination
5. **After Action Review** — Fortification specialist verifies and signs

The cycle is designed for clarity under pressure. Each phase has a defined output, a defined time allocation, and a defined responsible agent. There is no ambiguity about who does what.

## Token Budget Summary

| Role | Input | Output |
|------|-------|--------|
| Squad Leader | 3,000 | 800 |
| Heavy Engineer | 4,500 | 4,000 |
| Light Engineer | 4,000 | 1,500 |
| Survey Specialist | 4,000 | 500 |
| Fortification Specialist | 2,500 | 600 |
| Signals Specialist | 2,500 | 1,200 |
| **Team Total** | **20,500** | **8,600** |

## Failure Mode

The team fails by over-planning. The military planning cycle, when applied rigorously to a simple task, consumes 40% of the budget on reconnaissance and planning before a single line of patch is generated. Recovery: the squad leader can issue a "hasty attack" order, which compresses reconnaissance and planning into a single phase and sends the combat engineers directly to implementation. Hasty attacks are faster but produce lower-quality patches, so the light engineer allocates extra tokens for validation.
