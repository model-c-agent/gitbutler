# The Pilgrims' Route Cooperative — Agent Roster

**4 agents. Service-first. Humble by design.**

---

## Team as Unit

The Cooperative's agents are designed to be invisible. A good shuttle is one where the rider thinks about their destination, not the vehicle. A good agent is one where the developer thinks about their code, not the tool. The agents are deliberately simple, resource-efficient, and forgiving of imperfect conditions (unreliable networks, limited compute, budget constraints).

Four agents. Brother Thomas argued for fewer: "Complexity is a form of selfishness — it demands attention that should go to the work." Edwin compromised at four.

Agents are named after Swahili words related to service.

## Agents

**Huduma** (Service) — Patch Architect. Generates INDEX.patch with a focus on correctness and clarity. Huduma does not optimize for speed or elegance — it optimizes for reliability. Every patch must work on the first application. There is no revision cycle. The Cooperative's compute budget does not allow for iterative refinement. Huduma reads carefully, generates once, validates, and submits.

**Kumbuka** (Remember) — Memory & Context. Manages agent memory with extreme frugality. Memory entries are plain text (no embeddings, no vector search) to minimize compute requirements. Retrieval is keyword-based with recency weighting. Memory stored in `refs/pilgrims/memory/`. Kumbuka stores only what is necessary: decisions made, patterns learned, errors encountered. No verbose context. No aesthetic metadata. Information only.

**Daraja** (Bridge) — Provider, Budget, & Coordination. Combined role. Handles LLM provider selection (with a strong preference for local/free providers — Ollama first, cloud APIs only when local models cannot handle the task), token budget management, and cross-repo PR coordination. Daraja is the Cooperative's response to budget constraints: one agent does the work of two, at the cost of sophistication.

**Muhuri** (Seal) — Signing & Trust. OpenWallet integration. Simple, reliable signing. No complex ceremony. No extended metadata. Muhuri signs what Huduma produces, logs the signature, and moves on. Key rotation every 60 days.

## Dynamics

Sequential pipeline: Kumbuka retrieves context. Huduma generates the patch. Daraja manages the budget and coordinates. Muhuri signs. Simple. Predictable. Affordable.

The Cooperative runs all agents on a single modest machine — Edwin's refurbished ThinkPad that also serves as the dispatch server. The agents are designed to work within these constraints. Brother Thomas says this is a feature: "If the agent cannot run on a ThinkPad, it cannot serve the communities that need it most."

## Total Team Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Huduma | 6,000 | 3,000 | 9,000 |
| Kumbuka | 3,500 | 500 | 4,000 |
| Daraja | 5,000 | 1,500 | 6,500 |
| Muhuri | 2,000 | 400 | 2,400 |
| **Total** | **16,500** | **5,400** | **21,900** |

Smallest budget in this RFP. By design.

---

*Riders: 184,207. The van leaves at 7.*
