# Franciscan Wildlife Trust Proposal — `but-ai` Plugin

**Submitted by:** The Franciscan Wildlife Trust
**Date:** 2026-03-28

---

## Vocation

This proposal treats the `but-ai` plugin as a tool of stewardship. It records. It protects. It does not presume to command. The work is sacred because the creatures are sacred.

---

## 3.1 Plugin Architecture

`but-ai` as a Rust binary. CLI mode: `but ai record` (produce patch), `but ai chapter` (request Guardian review), `but ai canticle` (generate daily summary with liturgical framing). MCP mode: `but ai mcp`, `ServerHandler`, ten workspace tools plus `SanctuaryStatus` (per-sanctuary overview) and `BreedingAdvisory` (generate non-binding recommendation).

WASI: The archive agent (Br. Paolo) operates fully in WASI — read-only access to sanctuary records. Write agents degrade to advisory mode.

Config: `[but-ai]` in Git config.

---

## 3.2 Provider-Agnostic AI Interface

`but-llm` only. Provider via `gitbutler.aiModelProvider`. The Trust adds an ethical constraint: providers that use training data from conservation organizations without consent are blacklisted. This is enforced by a `[but-ai "provider-ethics"]` config section maintained by Br. Amadeo.

New providers: config entry + ethical review + signed adapter.

---

## 3.3 The But Agent

Stewardship workflow:

1. **Observe** (Br. Paolo): Load relevant sanctuary records from memory
2. **Assess** (Sr. Margherita): Analyze the task, produce INDEX.patch + COMMIT.msg
3. **Discern** (Br. Amadeo): Review the patch. Approve, redirect, or request revision.
4. **Communicate** (Sr. Teresa): If cross-sanctuary, coordinate via PR

Patches only. COMMIT.msg begins with the species common name and sanctuary: `[Assisi/Iberian Lynx] update breeding record for individual LX-047`. Branch naming: `fwt/<sanctuary>/<species-code>/<task-id>`.

Budget: Amadeo allocates per chapter session. Margherita receives 50%, Paolo 25%, Teresa 20%, Amadeo 5%. At 85% budget, Margherita produces a minimal canonical record and defers advisory recommendations.

---

## 3.4 Polyrepo PR-Based Coordination

Seven sanctuary repos coordinated by Sr. Teresa. Schema:

```json
{
  "fwt_schema": "1.0",
  "type": "record-update|transfer-request|breeding-advisory|chapter-summary",
  "sanctuary": "assisi",
  "species": "Lynx pardinus",
  "from": "Sr. Teresa",
  "canticle": "Praised be my Lord for all those who pardon for love of Thee",
  "body": "...",
  "budget_remaining_pct": 62
}
```

The `canticle` field is required. It may seem wasteful (20-30 tokens), but the Trust considers it non-negotiable.

Forge adapter: `ForgeAdapter` trait with `letter` (PR), `postscript` (comment), `correspondence` (list), `seal` (merge). GitHub implementation.

Cross-repo: Animal transfers between sanctuaries are modeled as cross-repo dependencies. A transfer of LX-047 from Assisi to Kenya requires patches in both repos, coordinated by Teresa.

---

## 3.5 Agent Memory and Identity

Memory stored in `refs/fwt/memory/<sanctuary>/<species>/`. Organized by sanctuary and species, mirroring the paper logbook structure.

**Structure:**
```json
{
  "key": "LX-047-health-record",
  "sanctuary": "assisi",
  "species": "Lynx pardinus",
  "individual": "LX-047",
  "value": "Routine veterinary check, weight 11.2kg, condition: good",
  "recorded_by": "Sr. Margherita",
  "ttl": "never",
  "classification": "canonical"
}
```

**Relevance scoring:** Individual animal ID match is the highest-weight filter. Within an individual's records, recency is primary. Cross-individual queries use species match + tag BM25. Maximum 5 entries.

**Expiration:** Canonical records (veterinary, breeding, transfer) never expire. Advisory records expire after 180 days. Operational records (scheduling, logistics) expire after 30 days.

**Compaction survival:** Canonical records always survive. Advisory records survive if they relate to an active breeding program. Operational records are dropped.

**Identity:** At `refs/fwt/identity/<agent>`:
```json
{
  "name": "Sr. Margherita",
  "title": "Sister Infirmarian",
  "vocation": "veterinary-care",
  "sanctuaries": ["assisi", "kenya"],
  "signing_key_id": "fwt:margherita:2026"
}
```

**Unique insight:** Individual-indexed memory. Most systems index by topic or category. The Trust indexes by individual animal. Every animal in their care has a unique identifier, and every memory entry links to that identifier. This creates a complete, Git-native medical and behavioral record per animal — something the paper logbooks provided but no digital system had replicated until now.

---

## 3.6 Signed Commits via OpenWallet

Every commit signed. Commits to canonical records require Amadeo's countersignature.

**Authorization:**
```json
{
  "Sr. Margherita": { "allow": ["fwt/*/veterinary", "fwt/*/breeding"], "deny": ["main"] },
  "Br. Paolo": { "allow": ["refs/fwt/memory/*"], "deny": ["fwt/*/veterinary"] },
  "Sr. Teresa": { "allow": ["fwt/*/coordination"], "deny": ["fwt/*/veterinary", "fwt/*/breeding"] },
  "Br. Amadeo": { "allow": ["*"], "role": "guardian" }
}
```

Canonical records (veterinary, breeding) require dual signature: producer (Margherita) + approver (Amadeo).

**Key lifecycle:**
- Provisioning: at profession of vows (joining the community)
- Rotation: annual, at the feast of Saint Francis (October 4)
- Compromise: immediate revocation. All canonical records signed by the compromised key are re-verified by Amadeo.

---

## 3.7 Token Budget

| Component | Input | Output | Frequency | Notes |
|-----------|-------|--------|-----------|-------|
| System prompt | 2,400 | 0 | Once/session | Identity, tools, sanctuary context |
| Task ingestion | 1,800 | 300 | Once/task | Animal record, task scope |
| Observation (memory load) | 2,000 | 0 | Once/task | Individual animal history |
| Tool call (per call) | 1,000 | 500 | 3/task avg | Branch ops |
| Patch generation | 3,000 | 3,500 | Once/task | INDEX.patch (canonical record) |
| Commit message | 500 | 400 | Once/task | Species + sanctuary prefix |
| Memory retrieval | 1,500 | 200 | 2/task avg | Individual-indexed lookup |
| Discernment (review) | 1,000 | 400 | Once/task | Amadeo approval |
| Coordination | 1,500 | 700 | 0.5/task avg | Cross-sanctuary letter |
| **TOTAL (typical)** | **18,200** | **7,500** | -- | Single animal record update |

---

## Testing Strategy

1. **Individual isolation:** Verify LX-047 records never contaminate LX-048 queries.
2. **Canonical immutability:** Verify canonical records cannot be deleted, only superseded.
3. **Dual signature:** Verify canonical commits fail without Guardian countersignature.
4. **Cross-sanctuary transfer:** Simulate animal transfer between 2 repos. Verify record continuity.
5. **Canticle compliance:** Verify all PR comments include the canticle field.

---

## Git Config Keys

| Key | Default | Purpose |
|-----|---------|---------|
| `but-ai.agent.tokenBudget` | 40000 | Per-task budget |
| `but-ai.agent.memoryRef` | `refs/fwt/memory` | Memory prefix |
| `but-ai.agent.canonicalDualSign` | true | Require dual signature for canonical records |
| `but-ai.forge` | `github` | Forge adapter |
| `but-ai.agent.canticleRequired` | true | Require canticle in PR comments |
| `but-ai.agent.advisoryTTL` | `180d` | Advisory record expiration |

---

*"Praised be my Lord for all His creatures."*
