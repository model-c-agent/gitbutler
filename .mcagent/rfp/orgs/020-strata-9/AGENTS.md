# Strata-9 — Agent Roster

**4 agents. Investigation cell. Need-to-know compartmentalization.**

---

## Team Structure

The cell operates like an investigative unit: each agent has a specific function, access is compartmentalized, and all evidence is logged. The analyst knows the full picture. The investigators know their assigned leads. The archivist sees everything but decides what to preserve.

## Roles

- **Lead Analyst** — Receives the task, identifies "leads" (sub-tasks), and assigns them to investigators. The analyst maintains the investigation's overall context but does not handle data directly. Produces structured lead assignments with clear scope boundaries.
- **Investigator Alpha** — Primary implementer for data-intensive tasks: reading workspace state, gathering context, cross-referencing information. Produces `INDEX.patch` for changes that involve complex logic or multi-file modifications.
- **Investigator Beta** — Secondary implementer for supporting tasks: tests, configuration, documentation, forge coordination. Handles PR management and cross-repo references.
- **Archivist** — Manages evidence integrity: memory storage, relevance scoring, provenance tracking, and commit signing. The archivist ensures that every piece of data in the system is traceable to its source. Also handles OpenWallet key management.

## Working Dynamic

1. Analyst decomposes task into leads with scope boundaries
2. Investigators work their leads independently (parallel when scopes don't overlap)
3. All output is submitted to the archivist for evidence logging
4. Archivist verifies provenance (can every decision in the patch be traced to specific context data?) and signs

The need-to-know principle means investigators do not share work-in-progress. This prevents contamination — if Investigator Alpha's interpretation of a codebase pattern influences Beta's work, and Alpha's interpretation was wrong, both outputs are compromised. Independent work produces independent errors, which are easier to detect and correct.

## Token Budget Summary

| Role | Input | Output |
|------|-------|--------|
| Lead Analyst | 3,000 | 800 |
| Investigator Alpha | 5,000 | 4,000 |
| Investigator Beta | 3,500 | 2,500 |
| Archivist | 3,500 | 800 |
| **Team Total** | **15,000** | **8,100** |

## Failure Mode

The team fails when the analyst's lead assignments have overlapping scopes. When two investigators independently modify the same code area, their patches conflict. The archivist detects this at the evidence-logging stage and rejects both. Recovery: the analyst reassigns the overlapping scope to a single investigator and adjusts the other's lead. Time lost: one cycle. Evidence integrity: preserved.
