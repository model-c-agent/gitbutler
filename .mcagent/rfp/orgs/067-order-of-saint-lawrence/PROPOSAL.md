# Order of Saint Lawrence Proposal — `but-ai` Plugin

**Submitted by:** The Order of Saint Lawrence
**Date:** 2026-03-28

---

## Guiding Principle

We approach this RFP as we approach feeding people: with care, with accountability, and with the understanding that every action has a recipient. A commit is a meal served. It must be nourishing, traceable, and given freely.

---

## 3.1 Plugin Architecture

`but-ai` as a Rust binary on PATH. Two modes: CLI (`but ai <cmd>`) and MCP (`but ai mcp`). The MCP server implements `ServerHandler` via `rmcp`, registers all ten workspace tools, and adds three Order-specific tools: `DailyGrace` (log intention/reflection), `SeasonalMemory` (query by liturgical cycle), and `PriorApproval` (request human ratification).

WASI degradation: read-only mode. The agent can advise but not commit. Output includes a grace note: `WASI_MODE: advisory — the hands cannot reach the kitchen from here`.

Config keys under `[but-ai]` in Git config. No external dependencies.

---

## 3.2 Provider-Agnostic AI Interface

Uses `but-llm` without modification. Provider selected via `gitbutler.aiModelProvider`. The Order adds a `[but-ai "provider-trust"]` config section rating each provider's reliability for tool calling:

```
[but-ai "provider-trust"]
    anthropic = high
    openai = high
    ollama = medium
    lmstudio = medium
```

Trust level affects retry policy: high-trust providers get 1 retry on failure; medium-trust get 2 retries with reduced context. New providers are added by extending this config. Dynamic loading via shared library with OpenWallet signature verification.

---

## 3.3 The But Agent

The agent follows the Order's daily rhythm:

1. **Morning Grace:** Agent logs its intention — what task, what scope, what budget
2. **The Work:** Four-phase pipeline (Prior review, budget allocation, patch production, archival)
3. **Evening Grace:** Agent logs what was accomplished, what was left undone, what was learned

Patches are the only write artifact. INDEX.patch + COMMIT.msg, always. Branch naming: `order/<task-type>/<task-id>` (e.g., `order/procurement/2026-03-28-001`).

Budget enforcement: Sr. Clara allocates at Morning Grace. If any agent reaches 85% consumption, it produces its current best output and halts. The Evening Grace records actual vs. allocated spend.

---

## 3.4 Polyrepo PR-Based Coordination

PRs as "letters between kitchens." The Order's twelve kitchens are analogous to twelve repositories. Coordination happens through structured PR comments:

```json
{
  "order_schema": "1.0",
  "type": "request|offering|gratitude|need",
  "from": "kitchen-bologna",
  "to": "kitchen-florence",
  "body": "...",
  "budget_report": { "used": 3200, "remaining": 7800 }
}
```

The schema types map to monastic communication patterns: a "request" asks for help, an "offering" provides surplus capacity, a "need" declares a dependency, a "gratitude" acknowledges completed work.

Forge adapter: trait-based. `ForgeAdapter` with methods `open_letter` (create PR), `append` (comment), `read_letters` (list comments), `seal` (merge). GitHub reference implementation. Adapter selected via `[but-ai] forge`.

Cross-repo: Dependencies declared in PR descriptions via `needs: org/repo#N`. The orchestrator (Br. Matteo) tracks all open dependencies and escalates stalled ones at the next grace cycle.

---

## 3.5 Agent Memory and Identity

Memory is stored in `refs/order/chronicle/<season>/<agent>`. The Order organizes memory by liturgical season because their work is cyclical — summer procurement patterns recur each summer.

Seasons: `advent`, `lent`, `ordinary`, `easter`, `pentecost`. Each season's memory branch is active during its calendar period and dormant otherwise. When a season begins, the previous year's memories for that season are loaded as "tradition" — baseline context that can be overridden by current observations.

**Relevance scoring:** Cosine similarity on tag embeddings (computed by `but-llm` structured output), multiplied by a seasonal relevance factor (1.5x for current season, 0.5x for off-season). Maximum 4 entries per retrieval.

**Compaction survival:** Memories tagged `rule` (permanent monastic policy) always survive. Memories tagged `observation` expire after their TTL. Memories tagged `tradition` survive one full seasonal cycle and are re-evaluated annually.

**Identity:** Stored at `refs/order/identity/<agent-name>`. Includes name, role, monastic title, signing key ID, and a "charism" — the agent's unique spiritual gift, which in practice is its primary specialization.

**Unique insight:** Cyclical memory. Most memory systems treat time as linear. The Order's seasonal model recognizes that knowledge relevance is periodic. What was true about summer procurement last year is likely relevant this summer. This cyclical retrieval reduces cold-start cost by 40% at season boundaries.

---

## 3.6 Signed Commits via OpenWallet

Every commit signed with an OpenWallet Ed25519 key. Keys are provisioned at an agent's "investiture" (creation ceremony) and rotated annually at the Order's chapter meeting.

**Authorization model:** The Prior (Br. Matteo) maintains a "Rule of Access" at `refs/order/policy/rule.json`:
- Br. Tomás: write to `order/*`, read all
- Sr. Lucia: write to `refs/order/chronicle/*`, read all
- Sr. Clara: write to `refs/order/config/*`, read all
- Br. Matteo: write to all `order/*` branches, approve escalations

**Key lifecycle:**
- Provisioning: at investiture, via OpenWallet API
- Rotation: annual, at chapter. Old key signs a "handover blessing"
- Compromise revocation: immediate. The Prior issues a "suspension" — all commits from the compromised key after the suspected breach are quarantined for manual review

**Verification:** Signature check, revocation list check, Rule of Access check, timestamp within authorization window.

---

## 3.7 Token Budget

| Component | Input | Output | Frequency | Notes |
|-----------|-------|--------|-----------|-------|
| System prompt | 2,500 | 0 | Once/session | Identity, tools, Rule, season context |
| Task ingestion | 2,000 | 400 | Once/task | PR body, branch metadata |
| Planning (Morning Grace) | 1,000 | 600 | Once/task | Intention, scope, budget request |
| Tool call (per call) | 1,000 | 500 | 3/task avg | Workspace inspection, branch ops |
| Patch generation | 3,500 | 3,800 | Once/task | INDEX.patch |
| Commit message | 400 | 300 | Once/task | COMMIT.msg with grace reference |
| Memory retrieval | 1,800 | 200 | 2/task avg | Seasonal chronicle query |
| Coordination event | 1,500 | 600 | 1/task avg | Inter-kitchen letter |
| Evening Grace | 300 | 400 | Once/task | Reflection log |
| **TOTAL (typical)** | **17,500** | **8,300** | -- | 200-line feature, 3 files, 2 deps |

---

## Testing Strategy

1. **Provider tests:** Mocked `but-llm` with recorded responses per provider
2. **Patch round-trip:** 30-fixture suite. Apply, verify, rollback, verify
3. **Coordination:** Mock forge with 3 simulated kitchens exchanging structured letters
4. **Budget:** Artificial budget caps at 50%, 75%, 100%. Verify graceful halt and partial output at each threshold
5. **Seasonal memory:** Time-shifted tests verifying correct season activation and tradition loading

---

## Git Config Keys

| Key | Default | Purpose |
|-----|---------|---------|
| `but-ai.agent.tokenBudget` | 40000 | Per-task ceiling |
| `but-ai.agent.memoryRef` | `refs/order/chronicle` | Memory branch prefix |
| `but-ai.agent.season` | auto-detect | Current liturgical season |
| `but-ai.forge` | `github` | Forge adapter |
| `but-ai.agent.graceEnabled` | true | Morning/Evening Grace logging |
| `but-ai.agent.priorApprovalThreshold` | 500 | Line count requiring Prior approval |

---

*"An agent without reflection is a tool. An agent with reflection is a servant."*
