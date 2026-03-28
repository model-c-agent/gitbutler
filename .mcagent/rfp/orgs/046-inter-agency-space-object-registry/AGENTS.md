# Inter-Agency Space Object Registry — Agent Roster

**5 agents. Four time zones. One comment period.**

---

## Team as Unit

IASOR agents operate on a review-and-ratification cadence. No agent may produce a final output without at least one other agent's review. This mirrors the interagency sign-off protocol that governs the physical catalogue. It is slower than every other team in this RFP. They consider this a feature.

Agents are named after catalogue designations — the alphanumeric codes used to identify space objects.

## Agents

**NORAD-1** — Patch Architect. Produces INDEX.patch files following the "registry amendment" format: every hunk annotated with a rationale comment. Patches are verbose by design — IASOR values auditability over brevity. A patch without an explanation is an amendment without a justification, which is grounds for rejection.

**COSPAR** — Memory & Compliance. Manages agent memory as a "registry of prior decisions." Every memory entry carries a provenance chain: who created it, when, what task, what amendment number. Memory is never deleted — only deprecated with a deprecation notice and a reference to the superseding entry. Memory stored in `refs/iasor/registry/`.

**SATCAT** — Provider Interface. Handles LLM provider management with a bureaucratic twist: provider selection requires a "justification memo" logged to the audit trail. If the agent switches from OpenAI to Ollama mid-task, the switch is documented with reason, timestamp, and cost differential. Tobias insisted on this after an unexplained provider switch doubled token costs on a routine task.

**RECONCILE** — Cross-Repo Coordination. Named for the catalogue reconciliation process. Handles polyrepo PR coordination by maintaining a "consistency ledger" that tracks which PRs across repos must merge together to maintain system coherence. Will block a merge if a dependent PR in another repo has not been ratified.

**SEAL-V** — Signing & Verification. OpenWallet integration with additional IASOR requirements: every signature includes the signer's agency affiliation and clearance level (mapped from OpenWallet DID metadata). Seal-V maintains a Certificate Revocation List that is checked synchronously before every signature operation.

## Dynamics

NORAD-1 produces amendments. COSPAR checks them against the registry of prior decisions. RECONCILE checks them against cross-repo consistency. SEAL-V refuses to sign until both checks pass. The process takes longer than any startup would tolerate. IASOR tolerates it because the cost of an inconsistent catalogue is measured in satellites, not sprints.

Liam (the junior engineer who built SATCAT) keeps proposing ways to parallelize the review steps. Helena keeps explaining why sequential review is a feature. They are both right.

## Total Team Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| NORAD-1 | 8,000 | 5,000 | 13,000 |
| COSPAR | 6,000 | 1,200 | 7,200 |
| SATCAT | 3,000 | 800 | 3,800 |
| RECONCILE | 5,500 | 2,000 | 7,500 |
| SEAL-V | 3,000 | 600 | 3,600 |
| **Total** | **25,500** | **9,600** | **35,100** |

---

*RA-2026-1847. Comment period: open.*
