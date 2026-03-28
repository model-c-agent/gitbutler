# Bureau of Automated Cultivation -- Technical Proposal

**Document:** BAC-DSMU-2026-012-P
**RFP:** `but-ai` Plugin for GitButler
**Classification:** Public
**Date:** 2026-03-28

---

## 1. Purpose

This proposal describes the DSMU's recommended approach to implementing the `but-ai` plugin, with emphasis on auditability, certification readiness, and safety traceability. The Bureau's contribution is not primarily a faster or cheaper implementation. It is an implementation designed to survive regulatory scrutiny.

---

## 2. PATH-Based Plugin Architecture

The plugin binary is installed to PATH. Configuration in TOML with a schema document for validation. The plugin validates its configuration at startup and logs the validation result to the audit trail.

The plugin responds to `but-ai audit` with a summary of all agent actions since the last audit query. This is the Bureau's primary contribution to the plugin interface: a built-in audit query command that produces structured, machine-readable output suitable for compliance review.

No daemon process. Federal systems prefer stateless operation where possible.

---

## 3. Provider-Agnostic AI

Four providers. Provider selection is explicit, logged, and auditable. Each provider invocation is recorded in the audit trail with: provider ID, model version, prompt hash, response hash, token count, and latency.

No automatic fallback. Provider switches are logged as operational decisions with justification.

The Bureau requires that provider terms of service permit use in safety-critical workflows. The configuration includes a `safety_cleared` boolean per provider. Providers without safety clearance cannot be used for tasks with `Safety-Impact: medium` or higher.

---

## 4. But Agent (INDEX.patch + COMMIT.msg)

CERT generates patches through a process that produces certification-ready artifacts:

1. Read task and relevant context
2. Classify the task's safety impact (`none`/`low`/`medium`/`high`)
3. Generate INDEX.patch
4. Generate COMMIT.msg with trailers:
   ```
   Safety-Impact: low
   Certification-Ref: BAC-2026-0044
   Audit-Trail: refs/but-ai/audit/2026-03-28/task-0012
   Reviewed-By: REG
   ```
5. Validate and submit for review

### Safety Classification

The safety classification is automatic, based on which files are modified. Files in directories matching configurable patterns (e.g., `**/safety/**`, `**/control/**`) automatically elevate the safety impact. The classification can be overridden with justification.

---

## 5. Polyrepo PR Coordination

Structured PR comments with Bureau-standard metadata:

```json
{
  "schema": "bac/coord/v1",
  "source": "repo-a@branch",
  "target": "repo-b@branch",
  "dependency": "blocks",
  "safety_impact": "low",
  "certification_ref": "BAC-2026-0044"
}
```

Every coordination comment includes a safety impact and certification reference. This ensures that cross-repo dependencies are traceable to the certification record.

Forge trait: four methods (`create_pr`, `post_comment`, `read_comments`, `pr_status`). GitHub implementation.

---

## 6. Agent Memory in Git Branches

Memory in `refs/but-ai/memory/<namespace>/<key>`. The Bureau adds a retention classification to each entry:

| Retention Class | TTL | Description |
|----------------|-----|-------------|
| Operational | 7 days | Task context, transient state |
| Reference | 90 days | Code patterns, architectural decisions |
| Certification | 7 years | Anything cited in a certification record |
| Permanent | None | Audit trail entries |

The retention classification is assigned by CERT at creation time. Memory entries cited in certification records are automatically upgraded to `Certification` retention.

### Audit Memory

The audit trail is a special memory namespace with permanent retention. Each entry records: agent, action, timestamp, input hash, output hash, safety impact, and compliance flags. The audit trail is append-only -- entries cannot be modified or deleted.

---

## 7. Signed Commits via OpenWallet

All commits signed. The signing credential includes the agent's identity and the Bureau's organizational DID. Commits to certification-tracked branches require a second signature from REG (attestation that the patch passed compliance review).

Key rotation: 24 hours. Compromised keys trigger a certification hold on all commits signed with the compromised key until re-verification is complete.

---

## 8. Token Budget

| Agent | Input | Output | Total | Role |
|-------|-------|--------|-------|------|
| CERT | 9,000 | 5,000 | 14,000 | Architecture & patches |
| REG | 5,500 | 2,500 | 8,000 | Review & registry |
| AUDIT | 3,500 | 1,000 | 4,500 | Audit trail |
| BUDGET | 3,000 | 1,500 | 4,500 | Cost control & signing |
| **DSMU** | **21,000** | **10,000** | **31,000** | |

Audit trail overhead: approximately 14% of total budget. The Bureau considers this the cost of accountability.

---

## 9. Unique Insight: Certification as Continuous Integration

The Bureau certifies agricultural software the way software teams certify builds: by running a suite of checks against defined criteria and producing a pass/fail report. The difference is that the Bureau's checks are manual, slow, and conducted by humans reading code.

Our insight is that `but-ai`'s audit trail can serve as input to automated certification checks. If every agent action is logged with safety classifications, decision provenance, and reviewer attestations, then the certification process can query the audit trail instead of re-reading the code. The certifier's question changes from "What does this code do?" to "What decisions were made, by whom, with what justification, and were they reviewed?"

This does not automate certification. It automates the evidence collection that certification requires. In our pilot project, evidence collection was 60% of total certification time. Automating it reduced a 14-month process to 4 months.

We propose embedding certification evidence collection into the `but-ai` plugin's audit system, so that every `but-ai`-managed project produces certification-ready evidence as a byproduct of normal operation. For projects that never need certification, the audit trail is free documentation. For projects that do, it saves months.

---

*BAC-DSMU-2026-012-P. Filed per Administrative Procedure 12.1.*
*Approved: Dr. Amara Osei. Noted: Thomas Hoffman.*
