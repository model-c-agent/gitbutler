# Wingspan Atelier Proposal — `but-ai` Plugin

**Submitted by:** Wingspan Atelier
**Date:** 2026-03-28

---

## Guiding Principle

Every commit protects a species. Traceability from code change to conservation outcome is not a feature — it is the mission.

---

## 3.1 Plugin Architecture

`but-ai` as a Rust binary on PATH. CLI mode: `but ai commission` (start task), `but ai sketch` (draft patch), `but ai render` (finalize patch), `but ai certificate` (generate traceability record). MCP mode: `but ai mcp`, `ServerHandler`, ten workspace tools plus `CertificateTrace` (link commit to conservation outcome).

WASI: Read-only mode. Certificate queries work. Patch generation does not. Useful for audit and reporting.

Config: `[but-ai]` in Git config.

---

## 3.2 Provider-Agnostic AI Interface

`but-llm` exclusively. Provider via `gitbutler.aiModelProvider`. The Atelier adds a "visual summary" output mode: when `BUT_OUTPUT_FORMAT=human`, every structured output includes an ASCII-art summary of key metrics. This costs ~200 extra output tokens but makes agent output legible to non-technical studio members.

New providers: Git config + signed adapter library.

---

## 3.3 The But Agent

Commission-based workflow:

1. **Brief:** Solberg defines the commission — species, scope, dependencies
2. **Sketch:** Kai produces a draft INDEX.patch. First commit tagged `sketch`.
3. **Refine:** Kai revises based on Solberg's review. Tagged `refined`.
4. **Render:** Final INDEX.patch + COMMIT.msg. Tagged `rendered`. Includes species binomial.
5. **Certificate:** Priya generates a traceability record linking the commit to its conservation context.

Patches only. Branch naming: `atelier/<species-code>/<task-id>` — e.g., `atelier/rhino-sondaicus/cert-pipeline-fix`.

Budget: Solberg allocates per-commission. Kai gets 60% (patch production), Priya 20% (memory/certificates), Fen 15% (coordination), Solberg 5% (oversight). At 85% total budget, Kai enters "quick render" mode — produces best available output.

---

## 3.4 Polyrepo PR-Based Coordination

Fen manages all external communication. Schema:

```json
{
  "atelier_schema": "1.0",
  "type": "commission|update|request|delivery",
  "species": "Rhinoceros sondaicus",
  "from": "Fen",
  "partner": "sumatran-rhino-trust",
  "body": "...",
  "budget_pct_remaining": 68
}
```

Every coordination message names the species. This is non-negotiable — it keeps the mission in every message.

Forge adapter: `ForgeAdapter` trait with `commission` (create PR), `update` (comment), `gallery` (list PRs by species), `deliver` (merge). GitHub implementation.

Cross-repo: `depends-on: org/repo#N` in PR descriptions. Fen checks dependencies before marking a commission as deliverable.

---

## 3.5 Agent Memory and Identity

Memory stored in `refs/atelier/memory/<species-code>/`. Memory is organized by species — the fundamental unit of the Atelier's work.

**Structure:**
```json
{
  "key": "rhino-sondaicus-habitat-funding",
  "species": "Rhinoceros sondaicus",
  "value": "14,000 hectares protected via 2024 auction cycle",
  "ttl": "180d",
  "tags": ["funding", "habitat", "auction"],
  "certificate_ref": "cert-2024-037"
}
```

**Relevance scoring:** Species match is the primary filter — queries about Javan rhinos retrieve Javan rhino memories first. Within a species, BM25 on tags with recency decay (half-life: 30 days). Maximum 4 entries per retrieval.

**Compaction survival:** Memories linked to certificates always survive — they are the conservation record. Operational memories follow TTL.

**Identity:** At `refs/atelier/identity/<agent>`:
```json
{
  "name": "Kai",
  "role": "illustrator",
  "species_expertise": ["aves", "mammalia"],
  "signing_key_id": "atelier:kai:2026"
}
```

**Unique insight:** Species-indexed memory. Instead of generic topic-based retrieval, memory is primarily organized around the biological entity the work serves. This creates natural boundaries (rhino work never contaminates hawk-eagle work) and enables per-species audit trails that map directly to conservation certificates.

---

## 3.6 Signed Commits via OpenWallet

Every commit signed. Keys per agent. Each certificate includes the signing key that authorized the related commits.

**Authorization:**
```json
{
  "Kai": { "allow": ["atelier/*"], "deny": ["main"], "max_lines": 400 },
  "Priya": { "allow": ["refs/atelier/memory/*", "refs/atelier/certs/*"], "deny": [] },
  "Fen": { "allow": ["atelier/coordination/*"], "deny": ["atelier/*/implementation"] },
  "Solberg": { "allow": ["main", "atelier/*"], "role": "director" }
}
```

Only Solberg merges to main.

**Key lifecycle:**
- Provisioning: at onboarding
- Rotation: annual, timed to the Atelier's fiscal year
- Compromise: immediate revocation. Certificates linked to compromised-key commits are flagged for re-verification.

---

## 3.7 Token Budget

| Component | Input | Output | Frequency | Notes |
|-----------|-------|--------|-----------|-------|
| System prompt | 2,300 | 0 | Once/session | Identity, tools, current commissions |
| Task ingestion | 1,800 | 300 | Once/task | Commission brief |
| Sketch (draft patch) | 2,500 | 2,500 | Once/task | First-pass INDEX.patch |
| Refinement | 1,500 | 1,500 | Once/task | Revision pass |
| Tool call (per call) | 1,000 | 500 | 3/task avg | Branch ops |
| Commit message | 400 | 350 | Once/task | With species binomial |
| Memory retrieval | 1,200 | 200 | 2/task avg | Species-indexed lookup |
| Certificate generation | 800 | 400 | Once/task | Traceability record |
| Coordination event | 1,500 | 600 | 1/task avg | Partner communication |
| **TOTAL (typical)** | **16,500** | **7,850** | -- | 200-line feature, 3 files, 2 deps |

---

## Testing Strategy

1. **Species isolation:** Verify rhino memories never appear in hawk-eagle queries.
2. **Certificate traceability:** End-to-end: commit -> certificate -> conservation outcome. Verify chain integrity.
3. **Patch round-trip:** 25 fixtures with sketch/refine/render lifecycle.
4. **Cross-repo:** Mock forge with 2 conservation partner repos.
5. **Budget:** Verify quick-render mode at 85% threshold.

---

## Git Config Keys

| Key | Default | Purpose |
|-----|---------|---------|
| `but-ai.agent.tokenBudget` | 40000 | Per-commission budget |
| `but-ai.agent.memoryRef` | `refs/atelier/memory` | Memory prefix |
| `but-ai.agent.speciesRequired` | true | Require species in commit messages |
| `but-ai.forge` | `github` | Forge adapter |
| `but-ai.agent.maxIterations` | 3 | Sketch/refine cycle limit |
| `but-ai.agent.certificateRef` | `refs/atelier/certs` | Certificate storage |

---

*"Name the species. Trace the impact. Ship the patch."*
