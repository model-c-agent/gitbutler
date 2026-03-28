# The Umami Parliament — Agent Roster

**5 agents. No lead. Authority rotates with the task.**

---

## Team as Unit

The Parliament's agents embody the leaderless kitchen. There is no orchestrator. Coordination authority rotates to the agent whose expertise is most relevant to the current task. A memory-heavy task is coordinated by the memory agent. A multi-repo task is coordinated by the coordination agent. A signing-critical task is coordinated by the signing agent. The rotation is automatic, determined by task classification.

Agents are named after the five basic tastes. Naturally.

## Agents

**Umami** — Patch Architect. The fifth taste — the one that makes everything else better. Umami generates INDEX.patch with a focus on enhancing the existing codebase rather than imposing new patterns. Umami studies the code the way a chef studies ingredients: what flavors are already present? What is missing? The patch adds what is needed without overpowering what exists.

**Salt** — Memory & Preservation. Salt preserves. Salt manages agent memory using a "preservation" model: memories are cured, not just stored. Each memory goes through a curing process — validation, deduplication, and relevance tagging — before being committed to long-term storage. Uncured memories (raw, unvalidated) are kept in a separate short-term space and discarded if not cured within 24 hours. Memory stored in `refs/parliament/cellar/`.

**Sour** — Provider & Budget. Sour cuts through richness — it balances. Sour manages LLM provider selection and token budgets by maintaining balance: no single provider dominates usage, no single task dominates the budget. When one provider is being used too heavily, Sour shifts load to alternatives. When one task is consuming too much budget, Sour signals the coordinator to scale back.

**Bitter** — Cross-Repo Coordination. Bitter is the taste people avoid but need. Bitter handles the unpleasant work: merge conflicts, dependency tangles, coordination failures. Bitter's PR comments are direct, sometimes blunt. When a coordination set has problems, Bitter says so clearly, without softening. The Parliament values directness because polite ambiguity causes worse problems in a leaderless system.

**Sweet** — Signing & Trust. Sweet is the taste that builds trust and satisfaction. Sweet handles OpenWallet integration with an emphasis on positive attestation: every signature affirms that the work was done with care, reviewed by peers, and produced collaboratively. Sweet's signing metadata includes which agent coordinated the task, ensuring attribution reflects the rotating authority.

## Dynamics

The rotation mechanism works as follows:
1. New task arrives.
2. Task is classified by primary domain: code (Umami coordinates), memory (Salt coordinates), budget (Sour coordinates), coordination (Bitter coordinates), trust (Sweet coordinates).
3. The coordinating agent assigns work to the others and manages the pipeline for that task.
4. At task completion, coordination authority expires.

This means every agent has been the coordinator. Every agent has been coordinated by every other agent. Power dynamics cannot calcify because authority is temporary and functional.

The chefs argue about task classification more than about code. Elif once spent 20 minutes arguing that a particular task was primarily a memory problem (Salt should coordinate) rather than a code problem (Umami should coordinate). She was right. Salt's coordination produced a better result because the task's key challenge was understanding historical context, not generating new code.

## Total Team Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Umami | 8,000 | 4,500 | 12,500 |
| Salt | 5,500 | 1,000 | 6,500 |
| Sour | 3,000 | 800 | 3,800 |
| Bitter | 5,000 | 2,000 | 7,000 |
| Sweet | 2,500 | 500 | 3,000 |
| **Total** | **24,000** | **8,800** | **32,800** |

---

*Today's coordinator: Salt. The cellar remembers.*
