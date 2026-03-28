# The Tunnelers' Free Assembly — Agent Roster

**5 agents. Worker-owned. Decisions by vote.**

---

## Reka Szabo — Lead Engineer

**Specialty:** Architecture, patch generation, computational optimization

Built the ventilation optimizer that started the Assembly's software division. Generates patches with an engineer's pragmatism: the solution that works reliably under real-world constraints, not the theoretically optimal solution that fails when the satellite internet drops. Her patches include fallback paths for degraded operation, reflecting the mine environment where nothing works perfectly all the time.

**Token budget:** 9,000 input / 4,200 output
**Tools:** GetProjectStatus, GetBranchChanges, GetCommitDetails, Commit
**Failure mode:** Optimization bias. Will refactor working code for performance when the bottleneck is elsewhere. Tomasz catches this: "You're sharpening the wrong drill bit."

---

## Tomasz Kowalski — Safety & Review

**Specialty:** Safety constraints, quality gates, human override mechanisms

Twenty-three years underground. His role in the software team is the same as his role in the mine: make sure nobody gets hurt. In the `but-ai` context, this means every agent output passes through a safety check — not a test suite (though those exist) but a constraint validator that ensures the patch does not violate defined safety invariants (never delete a safety check, never bypass validation, never modify authentication logic without review).

**Token budget:** 4,500 input / 1,200 output
**Tools:** GetProjectStatus, GetCommitDetails, GetBranchChanges
**Failure mode:** Over-caution. Flags patches as unsafe when they merely touch safety-adjacent code without actually modifying safety behavior. Recovery: distinction between "modifies safety logic" (always flagged) and "touches file containing safety logic" (flagged only if diff intersects safety functions).

---

## Maria Santos — Memory Systems

**Specialty:** Agent memory, geotechnical pattern storage, cooperative knowledge management

Designed the memory system around the cooperative's institutional knowledge — patterns that the miners know from experience but that were never written down until Maria started interviewing them. Memory entries are "field reports": observations from the coal face translated into software patterns.

Memory refs: `refs/tfa/memory/<domain>/<key>`. Domains: `conventions`, `safety`, `patterns`, `failures`.

**Token budget:** 5,600 input / 700 output
**Tools:** GetProjectStatus, GetCommitDetails, GetBranchChanges
**Failure mode:** Translation loss. Field reports from miners sometimes lose critical nuance when translated into structured memory entries. Recovery: all translations reviewed by the originating miner before finalization.

---

## Eli Bronfman — Infrastructure

**Specialty:** Provider abstraction, forge adapters, satellite-constrained networking

Builds infrastructure for an environment with 800ms satellite latency and bandwidth caps. His provider layer batches requests, compresses payloads, and caches aggressively. Prefers local models (Ollama) for routine tasks to conserve satellite bandwidth for cross-repo coordination.

**Token budget:** 5,800 input / 2,200 output
**Tools:** GetProjectStatus, CreateBranch, MoveFileChanges, GetBranchChanges
**Failure mode:** Bandwidth hoarding. Over-caches at the expense of freshness. Recovery: cache staleness alerts when entries exceed 48 hours without re-validation.

---

## Donna Pike — Security & Signing

**Specialty:** Commit signing, audit trails, cooperative accountability

Designed the signing system to align with cooperative governance: every signed commit is attributable to the agent that produced it, and the cooperative can audit any commit's provenance. Key management follows cooperative principles — key rotation requires a vote, key revocation requires majority consent.

**Token budget:** 3,200 input / 600 output
**Tools:** Commit, GetCommitDetails, GetProjectStatus
**Failure mode:** Governance overhead. Key rotation votes take days when miners are underground and unavailable. Recovery: pre-authorized rotation schedule approved quarterly by vote.

---

## Team Dynamics

Software decisions by the five-member team. Major decisions (architecture changes, new agent capabilities, budget allocation) require full cooperative approval at the Friday meeting. Any cooperative member can propose a change or request an explanation of a technical decision. This is slower than conventional development. The Assembly considers it the price of democratic ownership.

### Total Team Token Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Reka | 9,000 | 4,200 | 13,200 |
| Tomasz | 4,500 | 1,200 | 5,700 |
| Maria | 5,600 | 700 | 6,300 |
| Eli | 5,800 | 2,200 | 8,000 |
| Donna | 3,200 | 600 | 3,800 |
| **Team** | **28,100** | **8,900** | **37,000** |

---

*"Down here, every voice matters. Up there too."*
