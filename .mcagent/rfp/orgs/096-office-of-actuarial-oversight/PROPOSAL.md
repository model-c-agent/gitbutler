# Office of Actuarial Oversight & Compliance -- Technical Proposal

**RFP Response: `but-ai` Plugin for GitButler**

---

## Executive Summary

This proposal has been reviewed and approved by the Office of Actuarial Oversight & Compliance under authorization reference OAOC-2026-RFP-0047. All technical decisions described herein comply with the Office's Internal Software Development Framework (ISDF v3.2, 2025). The proposal is submitted in accordance with procurement regulation EU/2024/1789 and has been countersigned by Director van der Berg.

---

## Requirement 1: PATH-Based Plugin Architecture

The `but-ai` binary shall be deployed to `$PATH` in accordance with the Office's Software Deployment Standard (SDS-4.1). The binary requires security clearance level OAOC-B (standard internal tool) and must pass the Office's binary audit process before deployment to production workstations.

**Deployment process:**
1. Binary built from audited source code in the Office's CI environment
2. Binary hash recorded in the Software Asset Registry (Form 3A-IT)
3. Binary deployed to a staging workstation for 14-day evaluation
4. Evaluation report submitted (Form 9B-AO)
5. Director approval
6. Production deployment

Estimated time from build to production: 21 days. This is acceptable.

---

## Requirement 2: Provider-Agnostic AI

`Completer` trait with approved providers only. The Office maintains a Provider Approval Register. A provider must complete the Office's AI Provider Assessment (Form 14-AI, 28 pages) before it may be configured.

**Currently approved:** Anthropic (assessment completed 2025-11-14), OpenAI (assessment pending re-approval, due 2026-06-01). Ollama and LMStudio are classified as "local deployment" and require a separate IT Security Assessment (Form 6C-IT).

**Data residency:** All provider calls must comply with EU data residency requirements. Providers that cannot guarantee EU data processing are excluded regardless of technical merit.

---

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Agents produce INDEX.patch and COMMIT.msg. Every patch undergoes a three-stage review:

1. **Technical review** (Pieter): Does the patch work?
2. **Compliance review** (Annelies): Does the patch comply with ISDF v3.2?
3. **Director approval** (van der Berg): Is the patch authorized?

All three approvals are recorded in the audit trail before the commit is executed. COMMIT.msg includes `Approved-by:` trailers for each reviewer.

**Expedited review:** Pieter has proposed a single-stage review for patches under 20 lines that do not modify compliance-sensitive code. This proposal is under review (Form 7B-AO, submitted 2026-02-15, status: pending).

---

## Requirement 4: Polyrepo PR Coordination (Forge-Agnostic)

Forge adapter trait. The Office uses GitHub Enterprise (approved 2024). Cross-repo coordination generates audit entries for each PR interaction.

**Inter-departmental coordination:** When the audit tooling repo and reporting repo require coordinated changes, the coordination follows the Office's Inter-Departmental Change Protocol (IDCP v2.0):
1. Initiating department submits Form 11-IDCP
2. Receiving department acknowledges within 5 business days
3. Coordinated review proceeds in parallel
4. Merge requires sign-off from both department heads

---

## Requirement 5: Agent Memory in Git Branches

Memory stored in `refs/oaoc/audit/<date>/`. Entries are append-only audit records. Nothing is deleted. Retention period: seven years (regulatory minimum).

**Memory schema:**
```json
{
  "key": "audit-finding-2026-0047-003",
  "value": "...",
  "classification": "OFFICIAL",
  "retention_years": 7,
  "created": "2026-03-28T10:00:00Z",
  "created_by": "pieter.bakker@oaoc.gov.nl",
  "approved_by": "e.vanderberg@oaoc.gov.nl",
  "form_reference": "7B-AO-2026-0389"
}
```

**No TTL:** Memory entries in a regulatory context do not expire. They are retained for the legally mandated period and then archived (not deleted) to cold storage.

---

## Requirement 6: Signed Commits via OpenWallet

All commits signed. Henrik manages the key infrastructure. Key provisioning requires Form 8A-IT (IT Security Key Request). Key rotation is scheduled quarterly (not 90 days -- quarters align with the fiscal calendar). Emergency revocation follows the Office's Incident Response Protocol (IRP v1.3).

**Signature chain:** Every commit in the audit tooling repo must form an unbroken signature chain. A gap in the chain (an unsigned commit between signed commits) triggers an automatic compliance alert.

---

## Token Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Dir. van der Berg | 2,000 | 400 | 2,400 |
| Pieter | 8,500 | 4,000 | 12,500 |
| Annelies | 5,000 | 1,500 | 6,500 |
| Mads | 5,200 | 800 | 6,000 |
| Fatima | 5,500 | 2,200 | 7,700 |
| Henrik | 3,800 | 900 | 4,700 |
| **Total** | **30,000** | **9,800** | **39,800** |

Budget allocation is a fiscal year line item (budget code OAOC-2026-SW-AI-001). Supplementary requests via Form 12C-AO.

---

## Unique Insight: The Compliance Review as Automated Gate

Most proposals treat human review as a bottleneck to optimize away. We treat it as a mandatory gate that can be made more efficient without being eliminated.

Our three-stage review (technical, compliance, director) is slow. But each stage catches different classes of error. In the Office's first year of software development, 23% of patches that passed technical review were flagged during compliance review. 4% of patches that passed both were stopped at director review. The stages are not redundant -- they are complementary filters.

The `but-ai` plugin automates the preparation for each review stage: generating compliance checklists, pre-filling Form 7B-AO fields from patch metadata, and routing patches to the correct reviewer based on the files modified. The review itself remains human. The paperwork around it becomes automatic.

In internal testing, automated review preparation reduced the average time from patch submission to director approval from 96 hours to 58 hours. Still slow by startup standards. Exactly right by regulatory standards.

---

*"The form has been submitted. The process will proceed. Thank you for your patience."*
