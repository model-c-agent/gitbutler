# Ghost Line Collective — Agent Roster

**5 agents. Anonymous. Accountable to the data.**

---

## Team as Unit

Ghost Line's agents are designed for adversarial environments: untrusted data sources, potentially hostile infrastructure, and the assumption that any external system might lie. Every agent validates its inputs before trusting them. Every output is signed and verifiable. The collective builds agents the way they analyze transit data: assume nothing, verify everything.

Agents are named after transit data anomalies.

## Agents

**Phantom** — Patch Architect. Named for phantom bus runs. Generates INDEX.patch with a focus on defensive coding: every patch includes input validation, error handling, and explicit edge case management. Phantom treats every external interface as potentially hostile. Patches that trust external input without validation are considered defective regardless of whether they work in the happy path.

**Deadhead** — Memory & Analysis. Named for deadheading (a bus running empty to reposition). Manages agent memory with an "audit trail" model: every memory records not just what was learned but the evidence that supports it. Memory entries without supporting evidence are flagged as "unverified" and weighted lower in retrieval. Memory stored in `refs/ghostline/audit/`.

**Bunching** — Provider & Budget. Named for bus bunching (multiple buses arriving at the same time, then none for a long period). Manages LLM provider selection with a focus on output consistency. Bunching monitors provider output for quality bunching — periods of good output followed by periods of degraded output — and adjusts routing to avoid the degraded periods.

**Short-turn** — Cross-Repo Coordination. Named for short-turning (a bus turning around before reaching the end of its route). Handles polyrepo PR coordination with an equity-aware twist: Short-turn monitors whether agent work is disproportionately concentrated in some repos while neglecting others. If work distribution becomes inequitable, Short-turn flags it.

**Validator** — Signing & Integrity. OpenWallet integration with aggressive integrity checking. Validator does not just sign — it validates the entire chain: the patch is correct, the memory references are valid, the provider billing matches the logged usage, and the coordination set is consistent. Any discrepancy blocks the signature.

## Dynamics

The agents operate with mutual distrust by design. No agent trusts another's output without verification. Phantom validates its own patches. Deadhead verifies its own memory retrievals against the raw evidence. Validator checks everything before signing. This is paranoid. It is also the reason Ghost Line's publications have never been successfully challenged on factual grounds.

`fare_zero` designed the mutual distrust architecture. `route_null` thinks it is excessive. The agents do not care what `route_null` thinks.

## Total Team Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Phantom | 8,500 | 4,500 | 13,000 |
| Deadhead | 5,000 | 800 | 5,800 |
| Bunching | 3,000 | 700 | 3,700 |
| Short-turn | 4,500 | 1,800 | 6,300 |
| Validator | 3,500 | 600 | 4,100 |
| **Total** | **24,500** | **8,400** | **32,900** |

---

*Ghosts this month: 1,247. Auditing.*
