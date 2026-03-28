# Kapoor Conservation Legacy Proposal — `but-ai` Plugin

**Submitted by:** Kapoor Conservation Legacy
**Date:** 2026-03-28

---

## Principle

Every tiger is an individual. The system must know them as individuals, not as records.

---

## 3.1 Plugin Architecture

`but-ai` as a Rust binary. CLI mode: `but ai dossier` (query/update tiger records), `but ai assess` (behavioral readiness scoring), `but ai consult` (query Priya-II archive). MCP mode: `but ai mcp`, `ServerHandler`, ten workspace tools plus `TigerProfile` (individual animal query) and `ReadinessScore` (compute behavioral readiness).

WASI: Priya-II (archive) runs fully in WASI. Write agents degrade to scoring-only mode (can compute but not commit).

Config: `[but-ai]` in Git config.

---

## 3.2 Provider-Agnostic AI Interface

`but-llm` only. Provider via `gitbutler.aiModelProvider`. The Centre adds a "behavioral accuracy" metric per provider — how well does the provider's structured output match veterinary terminology and behavioral science conventions? Providers that consistently misclassify behavioral indicators are deprioritized.

New providers: config + behavioral accuracy calibration.

---

## 3.3 The But Agent

Dossier-driven workflow:

1. **Consult** (Priya-II): Load the relevant tiger's dossier from the archive
2. **Assess** (Meera + Arjun): Produce patches — Meera for veterinary records, Arjun for behavioral scores
3. **Review** (Vikram-ji): Approve or trigger the 30-day reassessment
4. **Record:** INDEX.patch + COMMIT.msg with tiger name

Branch naming: `kapoor/<tiger-name>/<data-type>/<task-id>` — e.g., `kapoor/kanha/veterinary/health-check-2026-03`. The tiger name is always in the branch.

Budget: Arjun allocates. Meera 40%, Arjun 30%, archive queries 20%, oversight 10%. At 80% budget, produce minimal record update and defer scoring to next session.

---

## 3.4 Polyrepo PR-Based Coordination

Multiple repos: one per partner reserve (4 reserves use the Kapoor methodology). Schema:

```json
{
  "kapoor_schema": "1.0",
  "type": "dossier-update|transfer|advisory|readiness-report",
  "tiger": { "id": "TG-023", "name": "Kanha" },
  "from_reserve": "kanha-centre",
  "body": "...",
  "readiness_score": 0.78,
  "budget_remaining_pct": 55
}
```

Every message names the tiger. Every message includes the readiness score if applicable.

Forge adapter: trait with `dossier_update` (PR), `annotate` (comment), `lineage` (list PRs by tiger), `release_approval` (merge). GitHub implementation.

Cross-repo: Tiger transfers between reserves are modeled as cross-repo dependencies. The tiger's dossier must be fully synchronized before physical transfer.

---

## 3.5 Agent Memory and Identity

Memory stored in `refs/kapoor/memory/<tiger-id>/`. Every tiger has its own memory namespace.

**Structure:**
```json
{
  "key": "TG-023-kanha-stress-pattern",
  "tiger": { "id": "TG-023", "name": "Kanha" },
  "category": "behavioral|veterinary|genetic|tracking",
  "value": "Exhibits elevated cortisol during monsoon season. Pattern consistent across 3 years.",
  "ttl": "never",
  "tags": ["stress", "seasonal", "cortisol"],
  "source": "behavioral-observation-2024"
}
```

**Relevance scoring:** Tiger-ID match is mandatory — you never retrieve another tiger's data when working on a specific individual. Within a tiger's namespace, category match provides a 1.5x boost. BM25 on tags. Maximum 5 entries.

**Expiration:** Behavioral and veterinary records never expire (they are the tiger's medical history). Tracking data expires after 90 days (GPS coordinates become irrelevant). Operational memories (scheduling, logistics) expire after 30 days.

**Compaction survival:** All records for the currently-active tiger survive. Records for other tigers are dropped and re-loaded from the memory branch on demand.

**Identity:** At `refs/kapoor/identity/<agent>`:
```json
{
  "name": "Meera",
  "role": "veterinarian",
  "generation": "G3",
  "tiger_expertise": ["Panthera tigris tigris"],
  "signing_key_id": "kapoor:meera:2026"
}
```

**Unique insight:** Per-individual memory namespaces. In a system that manages 47 individuals, each with decades of data, organizing memory per individual prevents cross-contamination. Kanha's cortisol patterns are never confused with Surya's dietary preferences. This is obvious for tiger rehabilitation but novel for agent memory systems, which typically organize by topic rather than subject.

---

## 3.6 Signed Commits via OpenWallet

Every commit signed. Dossier updates require dual signature: producer (Meera or Arjun) + reviewer (Vikram-ji).

**Authorization:**
```json
{
  "Meera": { "allow": ["kapoor/*/veterinary/*"], "deny": ["main"] },
  "Arjun": { "allow": ["kapoor/*/behavioral/*", "kapoor/*/tracking/*"], "deny": ["main"] },
  "Vikram-ji": { "allow": ["*"], "role": "senior-advisor" },
  "Priya-II": { "allow": [], "deny": ["*"], "role": "read-only" }
}
```

Priya-II has no signing key. She cannot commit. This is by design.

**Key lifecycle:**
- Provisioning: at joining the Centre
- Rotation: annual, on the anniversary of the Centre's founding (March 15, 1991)
- Compromise: immediate revocation. All dossier entries signed by the compromised key are reviewed by Vikram-ji.

---

## 3.7 Token Budget

| Component | Input | Output | Frequency | Notes |
|-----------|-------|--------|-----------|-------|
| System prompt | 2,500 | 0 | Once/session | Identity, tools, active tigers |
| Dossier load (Priya-II) | 2,500 | 0 | Once/task | Individual tiger history |
| Task ingestion | 1,500 | 300 | Once/task | Task scope, tiger selection |
| Tool call (per call) | 1,000 | 500 | 3/task avg | Branch ops |
| Patch generation | 3,000 | 3,500 | Once/task | INDEX.patch (dossier update) |
| Readiness scoring | 1,500 | 800 | 0.5/task avg | Behavioral assessment |
| Commit message | 500 | 400 | Once/task | With tiger name |
| Memory retrieval | 1,500 | 200 | 2/task avg | Per-individual lookup |
| Review (Vikram-ji) | 1,000 | 400 | Once/task | Approval or 30-day flag |
| Coordination | 1,500 | 600 | 0.5/task avg | Cross-reserve |
| **TOTAL (typical)** | **19,500** | **8,200** | -- | Single tiger dossier update |

---

## Testing Strategy

1. **Individual isolation:** Verify TG-023 queries never return TG-024 data.
2. **Dossier integrity:** Update veterinary and behavioral records simultaneously on separate branches. Verify clean merge.
3. **30-day rule:** Simulate Vikram-ji/score disagreement. Verify decision deferral.
4. **Cross-reserve transfer:** Simulate dossier synchronization between 2 repos.
5. **Archive gaps:** Query Priya-II for pre-digital records. Verify `ARCHIVE_GAP` response.

---

## Git Config Keys

| Key | Default | Purpose |
|-----|---------|---------|
| `but-ai.agent.tokenBudget` | 45000 | Per-task budget |
| `but-ai.agent.memoryRef` | `refs/kapoor/memory` | Memory prefix |
| `but-ai.agent.tigerNameRequired` | true | Require tiger name in commits |
| `but-ai.forge` | `github` | Forge adapter |
| `but-ai.agent.dualSignDossier` | true | Require dual signature for dossier updates |
| `but-ai.agent.reassessmentDays` | 30 | Disagreement deferral period |

---

*"Papa says Kanha is ready. The score says 0.78. We wait thirty days."*
