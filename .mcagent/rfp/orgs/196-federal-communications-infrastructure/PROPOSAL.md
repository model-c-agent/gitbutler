# FCIRB — Technical Proposal

**RFP:** `but-ai` Plugin for GitButler
**Classification:** Federal Infrastructure Compliance Automation

---

## Executive Summary

FCIRB proposes a compliance assessment agent system that cross-references infrastructure modification applications against the 847-page Federal Communications Infrastructure Standards Manual. Agents produce assessments, not decisions. Every finding cites the specific regulatory section, amendment, and effective date. All outputs are FOIA-ready and NARA-compliant.

---

## Requirement 1: PATH-Based Plugin Architecture

The `but-ai` binary is deployed per the Board's Approved Software List (ASL) process. Installation path: `/opt/fcirb/bin/but-ai`. The binary is signed with the Board's code-signing certificate and verified on every execution (mandatory per FCIRB IT Security Policy 3.2.1).

Subcommands: `but ai assess` (run compliance checks against application), `but ai cite` (attach regulatory citations), `but ai stale` (identify assessments affected by FCISM amendments), `but ai record` (commit to audit trail with retention metadata).

The `stale` command addresses the Retroactive Amendment Crisis. When a new FCISM amendment is published, `but ai stale` identifies all assessments that referenced the amended section and flags them for re-evaluation. This is computationally simple (text search for section numbers) but operationally critical.

## Requirement 2: Provider-Agnostic AI

The Board uses Azure OpenAI (FedRAMP High authorized). No other cloud provider is approved. Local inference is not authorized on Board workstations (insufficient hardware, insufficient security controls for local model deployment).

The provider interface includes mandatory audit logging: every API call records request hash, response hash, timestamp, and the FCISM sections referenced in the prompt. This audit log is committed to the repository weekly as a compliance artifact.

Provider switching requires a formal procurement action (estimated timeline: 8-14 months). The architecture supports multiple providers, but regulatory reality limits the Board to one at a time.

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

The compliance assessment pipeline:

1. Application data is ingested from the Board's document management system
2. Webb's compliance agents check each application field against FCISM requirements
3. For each finding, the agent generates INDEX.patch adding a finding record:

```diff
+ {
+   "finding_id": "FCIRB-2026-00847-F003",
+   "application": "FCIRB-2024-00847",
+   "section": "FCISM 8.3.2",
+   "amendment": "2025-Q3",
+   "effective_date": "2025-01-01",
+   "status": "non-compliant",
+   "description": "Proposed installation height exceeds 60m threshold
+     without FAA Form 7460-1 filing",
+   "recommendation": "Request FAA obstruction evaluation"
+ }
```

4. Carmen verifies all citations
5. COMMIT.msg:

```
Assess: FCIRB-2024-00847 — cell tower modification, height increase

Findings: 3 (2 compliant, 1 non-compliant)
Non-Compliant: FCISM 8.3.2 (height threshold) — FAA evaluation required
Analyst-Review-Required: yes
Citation-Verified: Agent/Reyes
Retention-Class: GRS-5.2 (7-year)
```

## Requirement 4: Polyrepo PR Coordination

Three repos: `fcirb-applications` (application records), `fcirb-standards` (FCISM encoded as machine-readable rules), `fcirb-assessments` (agent-generated assessments). When an FCISM amendment modifies a standard, the change propagates:

```
[FCIRB:amendment] fcirb-standards#2025-Q3 → fcirb-assessments
Amendment 2025-Q3 modifies FCISM 8.3.2 (height threshold).
74 existing assessments reference this section.
12 require re-evaluation. See fcirb-assessments#stale-report-2025Q3.
```

Forge adapter: GitHub Enterprise Cloud (government ToS). The Board evaluated GitLab; the procurement action is in progress (estimated completion: Q4 2026).

## Requirement 5: Agent Memory in Git Branches

Memory branch: `refs/fcirb/records/<fiscal-year>`. Memory types per NARA retention schedule:

- **`assessment`**: Compliance assessment records. Retention: 7 years (GRS 5.2).
- **`amendment-impact`**: Records of which assessments were affected by FCISM amendments. Retention: 10 years (GRS 1.1).
- **`configuration`**: Agent configuration and model version records. Retention: 3 years (GRS 3.1).

Memory entries include NARA disposition authority numbers. At end of retention period, records are transferred to the National Archives in a standardized format (JSON with XML wrapper per NARA transfer specification).

No semantic search. Key-based retrieval: `<fiscal-year>/<application-id>/<finding-id>`. The Board's legal counsel explicitly prohibited semantic search in compliance records because "approximate retrieval of regulatory findings is not retrieval; it is speculation."

## Requirement 6: Signed Commits via OpenWallet

Board staff use PIV cards for authentication and signing. Liu is building a PIV-to-OpenWallet bridge (same approach as NESRC's Kowalski — the two agencies are coordinating). Agent-generated assessments carry the generating agent's DID; human-reviewed assessments carry the analyst's PIV signature.

Dual-signature requirement for final determinations: agent DID + analyst PIV. This satisfies Procedure 7.4.4 ("No automated tool shall render a final determination") because the analyst's signature attests that a human reviewed and approved the assessment.

**Unique insight:** FCIRB's retroactive amendment handling reveals a capability that most version control-based agent systems ignore: the ability to evaluate *past outputs against changed rules*. When regulations change retroactively, every previous assessment that referenced the changed regulation becomes potentially invalid. The `stale` detection mechanism — scanning commit history for references to amended sections — transforms the Git repository from a record of past decisions into an active compliance monitoring system. This pattern is valuable beyond government: any domain where rules change retroactively (tax code, building codes, industry standards) needs the ability to identify which past agent outputs are affected by a rule change.

---

## Token Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Park | 1,200 | 400 | 1,600 |
| Webb | 4,500 | 3,500 | 8,000 |
| Liu | 2,500 | 1,500 | 4,000 |
| Osei | 1,000 | 400 | 1,400 |
| Reyes | 2,000 | 1,200 | 3,200 |
| **Task Total** | **11,200** | **7,000** | **18,200** |

Compliance overhead (citation verification, NARA tagging): 2,800 tokens. Grand total per application assessment: **21,000 tokens**.

---

*"The Board notes that this proposal was submitted within the required timeframe and conforms to applicable submission standards."*
— Automated acknowledgment, FCIRB Modernization Office
