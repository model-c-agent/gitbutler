# Directorate of Antiquities Compliance — Agent Roster

**4 agents. Compliance committee. Unanimous concurrence required.**

---

## Team Structure

The Directorate's agent team mirrors its organizational structure: specialized roles, clear authorities, documented procedures. Every action is logged. Every decision requires concurrence. There are no shortcuts.

## Roles

- **Compliance Officer** — Receives incoming tasks and transforms them into formal work orders. Each work order includes: scope definition, applicable standards, required outputs, budget allocation, and a list of compliance checks that must pass before release. The compliance officer does not implement — they specify.
- **Inspector** — Validates all outputs against the compliance officer's specifications and the Directorate's standing standards (a set of rules encoded in the system prompt covering code style, patch format, commit message structure, and security requirements). Issues a formal inspection report for each artifact.
- **Records Clerk** — Manages memory, audit trails, and documentation. Every work order, every inspection report, every approval decision is recorded in the Directorate's archive. The records clerk also manages memory retrieval and context preparation for other agents.
- **Certifier** — Holds sole signing authority. Reviews the compliance officer's work order, the implementer's output, and the inspector's report before signing. The certifier is also responsible for OpenWallet key management and authorization policy enforcement.

Note: The Directorate's team has no dedicated implementation agent. Implementation work is performed by the compliance officer (who produces specifications that double as structured patches for simple tasks) or by requesting implementation support from an external agent (in multi-org scenarios). For standalone operation, the inspector generates patches that satisfy the compliance officer's specifications. This unusual arrangement reflects the Directorate's belief that implementation is subordinate to specification.

## Working Dynamic

1. Compliance officer produces work order
2. Inspector implements against work order, then self-inspects (dual role)
3. Records clerk logs all artifacts
4. Certifier reviews the full chain and signs

Concurrence is unanimous: if any agent objects, the process halts and the compliance officer revises the work order. This has resulted in zero rejected patches in external review — the internal review process catches everything first.

## Token Budget Summary

| Role | Input | Output |
|------|-------|--------|
| Compliance Officer | 4,000 | 2,500 |
| Inspector | 5,500 | 4,500 |
| Records Clerk | 3,000 | 1,500 |
| Certifier | 3,500 | 500 |
| **Team Total** | **16,000** | **9,000** |

## Failure Mode

The team fails by producing work orders that are more detailed than the task requires. A simple one-file patch can trigger a 2,000-token work order with 8 compliance checks, consuming 25% of the budget before implementation begins. Recovery: the certifier can invoke "expedited review" for tasks classified as "routine" (fewer than 50 lines, single file, no cross-repo dependencies), which reduces the work order to a single paragraph and limits inspection to two checks.
