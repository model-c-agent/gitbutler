# DeepRoute Syndicate — Agent Roster

**4 agents. Pseudonymous. Trust-minimized coordination.**

---

## Team Structure

The syndicate operates on a trust-minimized model: each agent knows only what it needs to know. The controller knows the full task; the collector knows what data to gather; the analyst knows the specification; the auditor knows the output. No single agent has complete visibility. This is not just operational security — it is a design pattern for preventing any single compromised agent from leaking the full context of a task.

## Roles

- **Controller (`ctrl`)** — Receives tasks, decomposes them, and distributes specifications to other agents. The controller sees the full picture but produces no technical artifacts. Its output is structured task assignments.
- **Collector (`coll`)** — Gathers data. Calls workspace tools (`GetProjectStatus`, `GetBranchChanges`, `GetCommitDetails`), retrieves memory, and pre-fetches forge data. Produces a "dossier" — a structured context package — that is passed to the analyst.
- **Analyst (`anls`)** — The only agent that produces patches. Takes the controller's specification and the collector's dossier, then generates `INDEX.patch` + `COMMIT.msg`. Has no direct access to forge APIs or memory writes — only reads.
- **Auditor (`audt`)** — Verifies every output. Reviews the analyst's patch against the controller's specification. Checks signatures, authorization scopes, and budget compliance. Manages OpenWallet key operations. The auditor is the only agent with signing authority.

## Working Dynamic

The workflow is a pipeline with firewalls between stages:

1. `ctrl` produces task spec → passes to `coll` and `anls`
2. `coll` gathers context → passes dossier to `anls`
3. `anls` produces patch → passes to `audt`
4. `audt` verifies and signs → commit finalized

Agents communicate via signed messages. An unsigned message from any agent is ignored by all others. This prevents injection attacks where a compromised external system attempts to feed false instructions into the pipeline.

## Token Budget Summary

| Role | Input | Output |
|------|-------|--------|
| Controller | 3,000 | 800 |
| Collector | 4,000 | 500 |
| Analyst | 4,500 | 4,500 |
| Auditor | 4,000 | 600 |
| **Team Total** | **15,500** | **6,400** |

## Failure Mode

The team fails when the collector's dossier is incomplete. The analyst, lacking sufficient context, produces a patch based on assumptions rather than data. The auditor cannot detect this because the specification was satisfied — the problem is that the specification was met with incorrect assumptions.

Recovery: the auditor performs a "provenance check" — tracing each significant decision in the patch back to a specific data point in the dossier. If a decision cannot be traced, the patch is flagged as `UNGROUNDED` and the cycle restarts with an expanded collection scope.
