# FGLA Proposal — `but-ai` Plugin

**Submitted by:** Federal Garment Labeling Authority
**Date:** 2026-03-28

---

## Legal Basis

This proposal implements the `but-ai` plugin as an enforcement instrument. Every agent action has a statutory basis. Every output is an administrative record subject to Freedom of Information Act disclosure and judicial review.

---

## 3.1 Plugin Architecture

`but-ai` as a Rust binary on PATH. CLI mode: `but ai intake` (parse complaint), `but ai determine` (produce enforcement report), `but ai audit` (compliance check), `but ai docket` (case status). MCP mode: `but ai mcp`, `ServerHandler`, ten workspace tools plus `FiberLookup` (query 16 CFR 303.7 fiber table) and `CitationChain` (validate regulatory citation completeness).

WASI: Read-only compliance checking. Useful for external auditors and Congressional oversight offices.

Config: `[but-ai]` in Git config. Regulatory reference data at `refs/fgla/standards/<cfr-section>/<version>`.

---

## 3.2 Provider-Agnostic AI Interface

`but-llm` only. Provider via `gitbutler.aiModelProvider`. The FGLA adds a data-handling classification:

```
[but-ai "provider-classification"]
    openai = commercial-acceptable
    anthropic = commercial-acceptable
    ollama = government-preferred
    lmstudio = government-preferred
```

Enforcement data involving trade secrets (unpublished fiber compositions) must use government-preferred (self-hosted) providers. Routine complaints can use commercial providers.

New providers: config + security classification + Chen approval.

---

## 3.3 The But Agent

Enforcement pipeline:

1. **Intake** (Alvarez): Parse complaint, assign priority, open case branch
2. **Determine** (Tanaka): Produce INDEX.patch with preliminary enforcement report, including all regulatory citations
3. **Audit** (Okafor): Verify citations, confirm procedural compliance
4. **Approve** (Chen): Sign off — transforms draft into official determination

INDEX.patch + COMMIT.msg only. COMMIT.msg format: `[16 CFR 303.7(d)(1)] determine(case-2026-0142): <description> — fiber:<actual> labeled:<claimed>`.

Branch naming: `fgla/<case-id>/<phase>` — e.g., `fgla/case-2026-0142/determination`.

Budget: Tanaka (determination) 45%, Alvarez (intake) 20%, Okafor (audit) 20%, Chen (approval) 15%. At 90% budget, Tanaka produces a abbreviated determination with `ABBREVIATED: budget constrained — full analysis required` flag.

---

## 3.4 Polyrepo PR-Based Coordination

The FGLA coordinates with state consumer protection offices and FTC regional offices. Schema:

```json
{
  "fgla_schema": "1.0",
  "type": "complaint-referral|determination|request-for-analysis|response",
  "cfr_basis": "16 CFR 303.7(d)(1)",
  "case_id": "case-2026-0142",
  "from_office": "FGLA-DC",
  "to_office": "CA-DCA",
  "body": "...",
  "response_deadline": "2026-05-28",
  "classification": "public|trade-secret"
}
```

Trade-secret classified messages use encrypted payload. Response deadlines are mandatory — driven by statutory timelines.

Forge adapter: trait with `refer` (PR), `append_evidence` (comment), `docket` (list by case), `close_case` (merge). GitHub implementation.

Cross-repo: Multi-state enforcement actions create cross-repo dependencies. Each state office has its own repo. The FGLA coordinates the federal response.

---

## 3.5 Agent Memory and Identity

Memory stored in `refs/fgla/memory/<cfr-section>/`. Organized by the CFR section that governs the memory — because enforcement precedent is organized by regulation.

**Structure:**
```json
{
  "key": "silk-adjacent-precedent-2023",
  "cfr_section": "16 CFR 303.7(d)",
  "value": "FGLA determined that 'silk-adjacent' has no legal definition under 16 CFR 303.7. Ruled misleading. Fine: $50,000. Case: FGLA-2023-0891.",
  "case_ref": "FGLA-2023-0891",
  "ttl": "never",
  "tags": ["silk", "misleading-label", "precedent", "terminology"]
}
```

**Relevance scoring:** CFR-section match is primary. Case precedent entries receive a 2x boost (they inform future determinations). BM25 on tags. Maximum 6 entries (enforcement requires more context than typical tasks).

**Expiration:** Precedent entries never expire. Active case entries persist for case duration + 3-year retention. Routine operational memories TTL 90 days.

**Compaction survival:** Precedent entries always survive. Active case entries survive. Everything else follows TTL.

**Identity:** At `refs/fgla/identity/<agent>`:
```json
{
  "name": "Tanaka",
  "role": "enforcement-analyst",
  "gs_level": "GS-13",
  "certifications": ["textile-fiber-identification", "enforcement-procedures"],
  "signing_key_id": "fgla:tanaka:2026"
}
```

**Unique insight:** Precedent-weighted memory. In an enforcement context, past decisions constrain future decisions. The FGLA's memory system gives precedent entries permanent status and a 2x relevance boost, ensuring that agents never make a determination that contradicts established precedent without explicitly acknowledging the departure.

---

## 3.6 Signed Commits via OpenWallet

Every commit signed. Determinations require dual signature: analyst (Tanaka) + approver (Chen).

**Authorization:**
```json
{
  "Tanaka": { "allow": ["fgla/*/determination"], "deny": ["main", "fgla/*/audit"] },
  "Alvarez": { "allow": ["fgla/*/intake"], "deny": ["fgla/*/determination"] },
  "Okafor": { "allow": ["fgla/*/audit"], "deny": ["fgla/*/determination"] },
  "Chen": { "allow": ["*"], "role": "director" }
}
```

Separation of duties: the analyst who makes the determination cannot audit it. The auditor cannot make determinations.

**Key lifecycle:**
- Provisioning: via PIV card system, bridged to OpenWallet
- Rotation: annual, federal fiscal year
- Compromise: governed by NIST SP 800-57. All determinations signed by compromised key are reviewed by Okafor.

---

## 3.7 Token Budget

| Component | Input | Output | Frequency | Notes |
|-----------|-------|--------|-----------|-------|
| System prompt | 2,800 | 0 | Once/session | Identity, tools, active CFR sections |
| Complaint intake | 2,000 | 500 | Once/task | Parse, categorize, prioritize |
| Regulatory lookup | 1,500 | 0 | Once/task | Load applicable standard version |
| Tool call (per call) | 1,000 | 500 | 4/task avg | Branch ops, fiber lookup |
| Determination | 3,500 | 4,500 | Once/task | Enforcement report INDEX.patch |
| Citation chain | 500 | 400 | Once/task | Regulatory references |
| Commit message | 600 | 500 | Once/task | Full citation in COMMIT.msg |
| Memory retrieval | 2,000 | 300 | 2/task avg | Precedent-weighted lookup |
| Audit | 1,500 | 800 | Once/task | Okafor compliance check |
| Coordination | 1,800 | 700 | 0.5/task avg | Inter-office referral |
| **TOTAL (typical)** | **21,200** | **10,200** | -- | Single enforcement action |

---

## Testing Strategy

1. **Citation validation:** Submit determinations with invalid CFR references. Verify CitationChain tool rejects them.
2. **Precedent consistency:** Generate a determination that contradicts established precedent. Verify the system flags the contradiction.
3. **Retroactive standard:** Verify that complaints from 2022 are evaluated against the 2022 standard, not the 2026 standard.
4. **Separation of duties:** Verify Tanaka cannot audit, Okafor cannot determine.
5. **Trade secret handling:** Verify trade-secret-classified data never reaches commercial providers.

---

## Git Config Keys

| Key | Default | Purpose |
|-----|---------|---------|
| `but-ai.agent.tokenBudget` | 50000 | Per-case budget |
| `but-ai.agent.memoryRef` | `refs/fgla/memory` | Memory prefix |
| `but-ai.agent.standardsRef` | `refs/fgla/standards` | Regulatory reference storage |
| `but-ai.forge` | `github` | Forge adapter |
| `but-ai.agent.citationRequired` | true | Require CFR citation in all commits |
| `but-ai.agent.precedentBoost` | 2.0 | Relevance multiplier for precedent entries |

---

*"Polyester is polyester. Silk is silk. The label makes the promise. We enforce it."*
