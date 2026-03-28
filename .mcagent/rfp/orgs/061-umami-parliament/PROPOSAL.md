# The Umami Parliament — Technical Proposal for `but-ai`

**RFP Response | Tier 2 Abbreviated**

---

## Executive Summary

The Umami Parliament proposes a `but-ai` implementation with rotating coordination authority: no permanent orchestrator, no fixed hierarchy. Coordination responsibility shifts to the agent best suited for each task. Our domain expertise in running a leaderless Michelin-starred kitchen translates to agents that self-organize around competence rather than assignment.

## Requirement 1: PATH-Based Plugin Architecture

Rust binary at `$PATH` as `but-tool-ai`. The binary includes a `--classify` subcommand that analyzes a task description and outputs the recommended coordinating agent (code, memory, budget, coordination, or trust primary). This allows the human operator to override the automatic rotation if needed.

Configuration via `$XDG_CONFIG_HOME/but-ai/config.toml`. The rotation weights (which task types map to which coordinating agent) are configurable. Defaults follow the five-domain classification. Custom domains can be defined.

## Requirement 2: Provider-Agnostic LLM Interface

Four-provider backend. Sour manages provider selection with a balance constraint: no single provider may serve more than 60% of calls in a rolling 24-hour window. This prevents provider lock-in and ensures the system is tested against multiple backends continuously.

Sour maintains a "palate" — a per-provider quality profile built from recent calls. When the palate shows that a provider excels at certain task types (e.g., Anthropic for nuanced reasoning, Ollama for fast simple completions), Sour routes accordingly.

**Domain Insight:** In cooking, relying on a single ingredient — no matter how good — produces monotony. The best dishes balance multiple flavors. Sour applies this to providers: a system that routes everything to the "best" provider is fragile. A system that balances across providers is resilient, adaptable, and continuously benchmarked.

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Umami generates patches through a "tasting" process:
1. **Read the ingredients** — Survey the codebase: file structure, patterns, conventions.
2. **Taste the dish** — Read the specific files in scope and understand their existing flavor (style, patterns, conventions).
3. **Season carefully** — Generate INDEX.patch that enhances without overpowering. The change should feel like a natural extension of what exists.
4. **Taste again** — Self-review: does the patch match the dish? If the style deviates from the surrounding code, regenerate.

COMMIT.msg:
```
feat(rotation): add weighted authority selection

The rotation scheduler now weights authority assignments by
agent competence scores, addressing the 2-year argument.

Agent: Umami | Coordinator: Salt | Authority: black garlic
Tokens: 2,400/1,600 | Cellar-refs: salt/preserve-2026-0091
```

## Requirement 4: Polyrepo PR Coordination

Bitter handles cross-repo coordination with characteristic directness. Coordination sets are tracked in `refs/parliament/coord/`. Bitter's PR comments are structured but blunt:

```
COORDINATION STATUS: blocked
BLOCKER: repo-b/feat-branch has failing checks
REQUIRED ACTION: fix checks in repo-b before repo-a can merge
ESTIMATED DELAY: unknown until checks pass
```

No softening. No hedging. When a coordination set has a problem, Bitter names the problem and the required action. The Parliament learned from kitchen communication: in a high-pressure, leaderless environment, ambiguous status reports cause cascading failures.

Forge adapters (GitHub, GitLab, Gitea) implement a minimal trait.

## Requirement 5: Agent Memory in Git Branches

Salt manages memory using a "curing" model:

**Short-term (uncured):** Raw memory entries stored in `refs/parliament/raw/`. TTL: 24 hours. These are observations, impressions, and provisional learnings. If not cured within 24 hours, they expire.

**Long-term (cured):** Validated, deduplicated, relevance-tagged entries stored in `refs/parliament/cellar/`. TTL: 30-90 days based on relevance score. Curing requires:
- Validation (the memory is factually correct based on available evidence).
- Deduplication (the memory is not redundant with existing cellar entries).
- Tagging (relevance tags for future retrieval).

| Memory Type | Location | TTL | Curing Required |
|-------------|----------|-----|-----------------|
| Raw | `refs/parliament/raw/` | 24h | No (auto-expires) |
| Cured | `refs/parliament/cellar/` | 30-90 days | Yes |
| Foundational | `refs/parliament/cellar/foundation/` | 180 days | Yes + 2 agents agree |

Retrieval searches the cellar. Raw memories are never returned to other agents — they are Salt's working space only.

## Requirement 6: Signed Commits via OpenWallet

Sweet handles signing with collaborative attestation. The signing metadata includes:
- The coordinating agent for the task (the rotating authority).
- The contributing agents and their roles.
- A collaborative hash (a hash of all agents' outputs concatenated), proving that the signed commit reflects the collective's work, not any single agent's.

Key rotation every 30 days. Sweet maintains a "trust ledger" that records each agent's signing history. Agents with long, clean signing records are flagged as "trusted peers."

## Token Budget

| Agent | Role | Input/task | Output/task | Total |
|-------|------|-----------|-------------|-------|
| Umami | Patch generation | 8,000 | 4,500 | 12,500 |
| Salt | Memory curing | 5,500 | 1,000 | 6,500 |
| Sour | Provider balance | 3,000 | 800 | 3,800 |
| Bitter | Coordination (direct) | 5,000 | 2,000 | 7,000 |
| Sweet | Signing & trust | 2,500 | 500 | 3,000 |
| **Per-task total** | | **24,000** | **8,800** | **32,800** |

## Unique Domain Insight

Six years of running a leaderless kitchen taught us that the biggest risk in self-organizing systems is not chaos — it is invisible hierarchy. When no one is officially in charge, informal power structures emerge: the loudest voice, the most experienced person, the one who happens to be standing closest to the stove. These invisible hierarchies are worse than formal ones because they cannot be challenged or rotated.

Our proposal prevents invisible hierarchy through enforced rotation. The coordination role is assigned algorithmically based on task classification, not on agent initiative. An agent cannot seize coordination authority — it is granted by the system and revoked at task completion. This is not true anarchy (the Parliament does not pretend to be anarchist — it is parliamentary). It is structured egalitarianism: everyone leads, everyone follows, and the structure ensures neither role calcifies.

---

*Today's authority: black garlic. The Parliament is in session.*
