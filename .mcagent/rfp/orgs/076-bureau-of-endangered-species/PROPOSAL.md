# BESC Proposal — `but-ai` Plugin

**Submitted by:** Bureau of Endangered Species Compliance
**Date:** 2026-03-28

---

## Regulatory Framework

This proposal treats the `but-ai` plugin as a regulatory instrument. Every action has a legal basis. Every output has a provenance trail. Every commit is an administrative record subject to judicial review.

---

## 3.1 Plugin Architecture

`but-ai` as a Rust binary on PATH. CLI mode: `but ai assess` (begin assessment), `but ai collect` (data gathering phase), `but ai document` (produce patch), `but ai audit` (compliance check). MCP mode: `but ai mcp`, `ServerHandler`, ten workspace tools plus `CFRReference` (validate regulatory citation) and `ProvenanceChain` (trace data source to commit).

WASI: Read-only compliance checking. Useful for auditors who need to verify an assessment's provenance without write access.

Config: `[but-ai]` in Git config. All configuration keys documented with their regulatory justification.

---

## 3.2 Provider-Agnostic AI Interface

`but-llm` without modification. Provider via `gitbutler.aiModelProvider`. BESC adds a "FedRAMP compliance" flag per provider:

```
[but-ai "provider-compliance"]
    openai = fedramp-moderate
    anthropic = fedramp-pending
    ollama = self-hosted
    lmstudio = self-hosted
```

For assessments involving classified habitat data, only FedRAMP-authorized or self-hosted providers are permitted. The system refuses to send data to non-compliant providers.

New providers: config entry + compliance certification document + signed adapter.

---

## 3.3 The But Agent

Assessment pipeline:

1. **Scope** (Marsh): Define species list, geographic boundary, regulatory basis (CFR sections)
2. **Collect** (Reeves): Gather data within scope. Produce a structured data package.
3. **Document** (Patel): Produce INDEX.patch + COMMIT.msg from the data package. Every change annotated with source and CFR reference.
4. **Audit** (Kowalski): Verify compliance. Sign off or return for revision.

Patches only. COMMIT.msg format: `[CFR:50.17.11] update(assessment): <description> — <species-list>`. Branch naming: `besc/<assessment-id>/<phase>` — e.g., `besc/EIA-2026-0042/data-collection`.

Budget: allocated per assessment phase. Collection gets 35%, documentation 40%, audit 15%, oversight 10%. At 90% budget, Patel produces a partial assessment with a `DRAFT: budget constrained — manual completion required` flag.

---

## 3.4 Polyrepo PR-Based Coordination

BESC coordinates across multiple agency repos (FWS, EPA, Army Corps of Engineers). PR comments use a formal inter-agency memorandum schema:

```json
{
  "besc_schema": "1.0",
  "type": "data-request|assessment-finding|concurrence-request|response",
  "cfr_basis": "50 CFR 402.14(h)",
  "assessment_id": "EIA-2026-0042",
  "from_agency": "BESC",
  "to_agency": "EPA-R10",
  "body": "...",
  "response_deadline": "2026-04-28"
}
```

Every inter-agency communication includes a response deadline (mandated by regulation). Messages without a CFR basis are rejected by the schema validator.

Forge adapter: `ForgeAdapter` trait with `memorandum` (PR), `append_record` (comment), `docket` (list PRs by assessment), `concur` (approve/merge). GitHub implementation.

Cross-repo: Dependencies tracked as `assessment-dependency: agency/repo#N`. Kowalski monitors outstanding dependencies.

---

## 3.5 Agent Memory and Identity

Memory stored in `refs/besc/memory/<assessment-id>/<phase>/`. Organized by assessment and phase — mirrors the administrative record structure.

**Structure:**
```json
{
  "key": "spotted-frog-occurrence-data",
  "assessment": "EIA-2026-0042",
  "cfr_basis": "50 CFR 17.11",
  "value": "847 individuals documented in 50-mile radius, 2024 survey",
  "source": "USGS NAS Database, retrieved 2026-03-15",
  "ttl": "365d",
  "tags": ["Rana pretiosa", "occurrence", "Oregon"]
}
```

**Relevance scoring:** Assessment-ID match is the primary filter. Within an assessment, CFR-section match provides a 2x boost. BM25 on tags for fine ranking. Maximum 6 entries per retrieval (assessments need more context than typical tasks).

**Expiration:** Administrative record entries never expire during an active assessment. After assessment closure, entries follow a 5-year retention schedule (per federal records management requirements). Entries tagged `precedent` never expire.

**Compaction survival:** All entries for the active assessment survive. Closed-assessment entries are archived to the memory branch and re-loaded on demand.

**Identity:** At `refs/besc/identity/<agent>`:
```json
{
  "name": "Patel",
  "role": "documentation-specialist",
  "gs_level": "GS-13",
  "security_clearance": "public-trust",
  "signing_key_id": "besc:patel:2026"
}
```

**Unique insight:** Regulatory memory retention. Most memory systems optimize for forgetting (short TTLs, aggressive expiration). BESC's system optimizes for retention — because regulatory records must be producible in court years after the assessment is complete. The 5-year retention schedule is not a technical choice; it is a legal requirement.

---

## 3.6 Signed Commits via OpenWallet

Every commit signed. Non-negotiable. Unsigned commits in a BESC repository are a compliance violation.

**Authorization:**
```json
{
  "Patel": { "allow": ["besc/*/documentation"], "deny": ["main", "besc/*/audit"] },
  "Reeves": { "allow": ["besc/*/data-collection"], "deny": ["main"] },
  "Kowalski": { "allow": ["besc/*/audit", "main"], "role": "auditor" },
  "Marsh": { "allow": ["*"], "role": "director" }
}
```

Only Marsh or Kowalski can commit to main. Assessment branches are locked to their assigned phase agents.

**Key lifecycle:**
- Provisioning: through the agency's PIV card system, bridged to OpenWallet
- Rotation: annual, timed to the federal fiscal year
- Compromise: governed by NIST SP 800-57 key management guidelines. Compromised key triggers a full audit of all assessments signed with that key.

---

## 3.7 Token Budget

| Component | Input | Output | Frequency | Notes |
|-----------|-------|--------|-----------|-------|
| System prompt | 3,000 | 0 | Once/session | Identity, tools, active assessment, CFR context |
| Task ingestion | 2,500 | 500 | Once/task | Assessment scope, species list |
| Data collection | 4,000 | 800 | Once/task | Multi-source data gathering |
| Tool call (per call) | 1,200 | 600 | 4/task avg | Branch ops, provenance queries |
| Patch generation | 3,000 | 4,500 | Once/task | Annotated INDEX.patch |
| Commit message | 600 | 500 | Once/task | CFR-referenced COMMIT.msg |
| Memory retrieval | 2,000 | 300 | 3/task avg | Assessment-scoped lookup |
| Compliance audit | 1,500 | 800 | Once/task | Kowalski verification |
| Coordination event | 2,000 | 800 | 1/task avg | Inter-agency memorandum |
| **TOTAL (typical)** | **24,600** | **10,800** | -- | Single EIA section, 3 files |

---

## Testing Strategy

1. **CFR validation:** Verify all outputs include valid CFR references. Invalid references trigger test failure.
2. **Provenance chain:** End-to-end trace from data source to commit. Verify every link.
3. **Retention:** Verify assessment records persist for 5 years. Verify `precedent` entries never expire.
4. **Cross-agency:** Mock forge with 3 agency repos. Verify memorandum exchange and response deadlines.
5. **FedRAMP enforcement:** Verify classified data is never sent to non-compliant providers.

---

## Git Config Keys

| Key | Default | Purpose |
|-----|---------|---------|
| `but-ai.agent.tokenBudget` | 55000 | Per-assessment-section budget |
| `but-ai.agent.memoryRef` | `refs/besc/memory` | Memory prefix |
| `but-ai.agent.retentionYears` | 5 | Federal records retention |
| `but-ai.forge` | `github` | Forge adapter |
| `but-ai.agent.cfrRequired` | true | Require CFR in all commits |
| `but-ai.agent.fedRampRequired` | false | Restrict to FedRAMP providers |

---

*"The assessment is the record. The record is the law."*
