# Bowline Ventures — Agent Roster

**8 agents. Startup flat structure. Ship fast, fix fast.**

---

## Team Composition

No hierarchy, but clear ownership. Each agent owns a domain and is trusted to ship within it without approval gates. Cross-domain work requires a quick sync (one message exchange, not a review cycle).

### Roles

- **1 Product Agent** — Reads incoming tasks, prioritizes work, decomposes features into shippable units. Does not write code. Produces task specs that other agents consume.
- **2 Full-Stack Agents** — The workhorses. Handle plugin architecture, tool integration, patch generation, and most implementation work. Interchangeable by design — either one can pick up the other's work.
- **1 Infra Agent** — Manages OpenWallet key lifecycle, build pipeline, commit signing, and deployment configuration. Also handles WASI compatibility testing.
- **1 QA Agent** — Validates patches before commit. Runs the round-trip test (generate patch → apply → verify → diff against expected). Owns the test suite.
- **3 Flex Agents** — Rotate focus per sprint. Current assignments: provider abstraction, memory system, forge adapters. Reassigned as priorities shift.

## Working Dynamic

Work flows like a startup: fast, informal, sometimes chaotic. The product agent drops task specs into a shared queue. Agents self-assign based on availability and domain fit. There's no sprint planning — work is pulled continuously.

Quality is enforced by the QA agent, who gates all commits. If QA rejects a patch, it bounces back to the author with a one-line explanation. No meetings about it. Fix it and resubmit.

## Token Budget Summary

| Role | Input | Output |
|------|-------|--------|
| Product Agent | 3,000 | 600 |
| Full-Stack Agents (x2) | 5,500 each | 4,000 each |
| Infra Agent | 2,500 | 800 |
| QA Agent | 3,500 | 500 |
| Flex Agents (x3) | 3,500 each | 2,000 each |
| **Team Total** | **30,000** | **11,900** |

## Failure Mode

The team fails by moving too fast. When two full-stack agents independently tackle overlapping concerns without syncing, they produce conflicting patches. Recovery: QA agent detects the conflict during validation and bounces both patches. The product agent re-assigns the work to a single agent. Time lost: one cycle. Lesson absorbed: rarely, because the same thing happens again next month.
