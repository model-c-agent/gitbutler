# Admiralty Bureau of Freight Standards — Agent Roster

**4 agents. Committee structure. Unanimous approval required.**

---

## Team Structure

The Bureau's agent team operates as a formal committee. Every action is proposed, seconded, deliberated upon, and approved before execution. This is not a metaphor. The agents literally exchange proposal messages, wait for seconds, and tabulate votes before producing output.

This makes the team the slowest in the field. It also makes it the most reliable. In 14 months of operation, the committee has never produced a patch that was rejected in review, never signed a commit with an expired key, and never exceeded a token budget.

## Roles

- **Committee Chair** — Receives incoming tasks, frames them as formal proposals, manages the deliberation schedule, and maintains the meeting minutes. Produces no technical output directly. All output is attributed to the committee, never to an individual.
- **Policy Agent** — Drafts all specifications, interface definitions, and protocol schemas. Produces formal documents before any code is written. If the specification is incomplete, no implementation proceeds. The policy agent's output is verbose, precise, and humourless.
- **Inspection Agent** — Validates all implementations against the policy agent's specifications. Runs test suites, verifies patch correctness, checks for compliance with all applicable Bureau standards (of which there are many). Reports findings using the Bureau's standard inspection form format.
- **Archivist Agent** — Manages memory, audit trails, and documentation. Every decision, every rejected proposal, every approved patch is recorded in the archive. The archivist also manages OpenWallet key ceremonies, treating key provisioning as a formal Bureau procedure with witnesses (other agents) and documentation.

## Deliberation Protocol

1. Chair receives task, issues Proposal Number (sequential, never reused)
2. Policy agent drafts specification (consumes 20-30% of task budget)
3. Committee reviews specification — all agents may raise objections
4. If no objections: implementation proceeds. If objections: revision cycle
5. Implementation is validated by inspection agent
6. Archivist records all artifacts and votes
7. Chair issues formal approval and the patch is signed

## Token Budget Summary

| Role | Input | Output |
|------|-------|--------|
| Committee Chair | 3,000 | 800 |
| Policy Agent | 4,500 | 3,500 |
| Inspection Agent | 5,000 | 1,200 |
| Archivist | 3,000 | 2,000 |
| **Team Total** | **15,500** | **7,500** |

Note: The deliberation overhead (proposal/vote exchanges) consumes approximately 15% of total budget. The Bureau considers this an acceptable cost of governance.

## Failure Mode

The team fails by over-deliberating. When a task is ambiguous, the committee can enter an infinite revision cycle where the policy agent keeps refining the specification and the inspection agent keeps finding new edge cases. Recovery: the chair has authority to invoke "emergency session" rules, which limit deliberation to two rounds and force a vote. The chair has used this authority exactly once in 14 months and issued a formal apology to the committee afterward.
