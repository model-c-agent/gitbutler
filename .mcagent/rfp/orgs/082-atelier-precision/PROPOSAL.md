# Atelier Precision Proposal — `but-ai` Plugin

**Submitted by:** Atelier Precision
**Date:** 2026-03-28

---

## Doctrine

If it is not measured, it is not done. Every output has a specification. Every specification has a tolerance. Every commit reports both.

---

## 3.1 Plugin Architecture

`but-ai` as a Rust binary. CLI mode: `but ai spec` (load specification), `but ai produce` (generate patch), `but ai inspect` (validate against spec), `but ai report` (compliance summary). MCP mode: `but ai mcp`, `ServerHandler`, ten workspace tools plus `SpecVersion` (resolve current specification) and `MeasurementReport` (structured compliance output).

WASI: Inspection-only mode. Run compliance checks in a sandboxed WASI environment. No write capability — read the garment, measure it, report findings.

Config: `[but-ai]` in Git config. Specifications stored at `refs/precision/specs/<spec-id>/<version>`.

---

## 3.2 Provider-Agnostic AI Interface

`but-llm` only. Provider via `gitbutler.aiModelProvider`. Atelier Precision adds a "determinism score" per provider — how consistent are the provider's outputs for identical inputs? Providers with determinism scores below 0.95 are rejected for specification-critical tasks.

```
[but-ai "provider-determinism"]
    anthropic = 0.97
    openai = 0.95
    ollama = 0.92
    lmstudio = 0.91
```

Determinism scores are measured monthly via calibration tasks. New providers: config + calibration + Voronova approval.

---

## 3.3 The But Agent

Inspection-grade workflow:

1. **Load spec** (Petrov): Resolve the current specification version for the task
2. **Produce** (Sala): Generate INDEX.patch against the loaded specification
3. **Inspect** (Chen): Validate patch against specification tolerances
4. **Approve** (Voronova): Sign off or return for revision
5. **Coordinate** (DaSilva): If multi-client, handle PR communication

Every COMMIT.msg includes: spec version, tolerance specified, tolerance achieved, pass/fail. Example: `[SPEC:jacket-v2.3] seam(shoulder) — spec:0.3mm achieved:0.18mm PASS`.

Branch naming: `precision/<project-id>/<checkpoint>` — e.g., `precision/jacket-0247/checkpoint-12`.

Budget: Sala 40%, Chen 25%, Petrov 15%, DaSilva 15%, Voronova 5%. At 85% budget, Sala produces minimal patch. Inspection remains mandatory regardless of budget.

---

## 3.4 Polyrepo PR-Based Coordination

DaSilva coordinates across client repos and supplier repos. Schema:

```json
{
  "precision_schema": "1.0",
  "type": "specification|inspection-report|deviation-request|coordination",
  "project": "jacket-0247",
  "checkpoint": 12,
  "spec_version": "jacket-v2.3",
  "measurement": { "spec": "0.3mm", "achieved": "0.18mm", "verdict": "PASS" },
  "from": "DaSilva",
  "body": "..."
}
```

Every coordination message includes the measurement. Messages without measurements are flagged as incomplete.

Forge adapter: trait with `dispatch` (PR), `annotate` (comment), `inspection_log` (list by project/checkpoint), `certify` (merge). GitHub implementation.

Cross-repo: Supplier repos provide fabric specification data. Client repos receive inspection reports. Dependencies are one-directional.

---

## 3.5 Agent Memory and Identity

Memory stored in `refs/precision/memory/<spec-id>/`. Organized by specification — the fundamental unit of the Atelier's work.

**Structure:**
```json
{
  "key": "jacket-v2.3-shoulder-tolerance",
  "spec_id": "jacket-v2.3",
  "value": "Shoulder seam tolerance tightened from 0.3mm to 0.2mm in v2.3. Reason: stress accumulation observed at 0.3mm under repeated motion testing.",
  "measurement_history": [0.31, 0.28, 0.22, 0.18],
  "ttl": "never",
  "tags": ["shoulder", "tolerance", "stress"]
}
```

**Relevance scoring:** Spec-ID match is mandatory. Within a specification, checkpoint match provides a 2x boost. BM25 on tags. Measurement trend data (improving or degrading?) provides a small boost for entries showing improvement. Maximum 5 entries.

**Expiration:** Specification memories never expire while the specification is active. When a specification is superseded, its memories are archived with a 2-year retention. Historical measurement data is retained indefinitely (it informs future specifications).

**Compaction survival:** Active specification memories always survive. Measurement history is compressed (kept as statistical summary rather than raw data) to save tokens.

**Identity:** At `refs/precision/identity/<agent>`:
```json
{
  "name": "Sala",
  "role": "head-cutter",
  "rank": "operations-chief",
  "certifications": ["mil-std-1916", "iso-9001"],
  "signing_key_id": "precision:sala:2026"
}
```

**Unique insight:** Measurement-embedded memory. Every memory entry includes quantitative measurement data, not just qualitative descriptions. This enables agents to track tolerance trends over time — is the system getting more precise or less? — and to set specifications informed by historical measurement distributions rather than arbitrary targets.

---

## 3.6 Signed Commits via OpenWallet

Every commit signed. Inspection results require dual signature: inspector (Chen) + approver (Voronova).

**Authorization:**
```json
{
  "Sala": { "allow": ["precision/*/production"], "deny": ["main", "precision/*/inspection"] },
  "Chen": { "allow": ["precision/*/inspection"], "deny": ["precision/*/production"] },
  "Petrov": { "allow": ["refs/precision/specs/*", "refs/precision/memory/*"], "deny": [] },
  "DaSilva": { "allow": ["precision/*/coordination"], "deny": ["precision/*/production"] },
  "Voronova": { "allow": ["*"], "role": "commander" }
}
```

Production and inspection are separated at the authorization level. The agent that produces the garment cannot inspect it.

**Key lifecycle:**
- Provisioning: at onboarding, with certification verification
- Rotation: annual
- Compromise: immediate revocation. All inspection reports signed by the compromised key are re-inspected. No exceptions.

---

## 3.7 Token Budget

| Component | Input | Output | Frequency | Notes |
|-----------|-------|--------|-----------|-------|
| System prompt | 2,500 | 0 | Once/session | Identity, tools, active specifications |
| Spec loading | 1,500 | 0 | Once/task | Current spec version |
| Task ingestion | 1,800 | 400 | Once/task | Project, checkpoint |
| Tool call (per call) | 1,000 | 500 | 4/task avg | Branch ops, spec queries |
| Patch generation | 3,000 | 3,500 | Once/task | INDEX.patch |
| Measurement report | 500 | 800 | Once/task | Structured inspection output |
| Commit message | 500 | 400 | Once/task | With measurement data |
| Memory retrieval | 1,500 | 200 | 2/task avg | Spec-indexed lookup |
| Coordination | 1,500 | 600 | 1/task avg | Client/supplier comms |
| **TOTAL (typical)** | **18,300** | **8,400** | -- | Single checkpoint, one garment |

---

## Testing Strategy

1. **Specification enforcement:** Generate patches that violate tolerances. Verify Chen rejects them.
2. **Determinism:** Run identical tasks against multiple providers. Verify output consistency.
3. **Separation of duties:** Verify production agent cannot sign inspection reports and vice versa.
4. **Measurement precision:** Verify COMMIT.msg includes correct measurement data.
5. **Spec versioning:** Update a specification mid-project. Verify in-progress work is re-evaluated.

---

## Git Config Keys

| Key | Default | Purpose |
|-----|---------|---------|
| `but-ai.agent.tokenBudget` | 45000 | Per-checkpoint budget |
| `but-ai.agent.memoryRef` | `refs/precision/memory` | Memory prefix |
| `but-ai.agent.specRef` | `refs/precision/specs` | Specification storage |
| `but-ai.forge` | `github` | Forge adapter |
| `but-ai.agent.measurementRequired` | true | Require measurements in commits |
| `but-ai.agent.determinismThreshold` | 0.95 | Minimum provider determinism |

---

*"0.18mm achieved on a 0.3mm spec. That is not over-engineering. That is margin."*
