# NESRC — Technical Proposal

**RFP:** `but-ai` Plugin for GitButler
**Classification:** Government Use — Compliance Automation

---

## Executive Summary

The National Electronic Sports Regulatory Commission proposes a compliance-first agent system where every agent action is traceable to a regulatory citation. Agents do not make decisions; they generate recommendations with legal basis. All outputs are structured for FOIA compliance and federal records retention. The system prioritizes auditability above all other concerns.

---

## Requirement 1: PATH-Based Plugin Architecture

The `but-ai` binary will be installed on Commission workstations per the IT Deployment Standard (IDS-2024-03). Installation path: `/opt/nesrc/bin/but-ai`, symlinked to PATH. The binary must pass the Commission's software approval process, which requires: (a) source code review by Kowalski, (b) vulnerability scan using the NIST NVD database, (c) sign-off by Huang.

The binary runs in a restricted execution environment. No network access except to the configured AI provider (whitelisted by IP). No filesystem access outside the repository working directory. No child process spawning. These restrictions are imposed by the Commission's endpoint security policy, not by architectural choice.

Subcommands: `but ai review` (analyze application against codex), `but ai cite` (attach regulatory citations to findings), `but ai recommend` (generate approval/denial recommendation), `but ai record` (commit to audit trail).

## Requirement 2: Provider-Agnostic AI

The Commission's AI provider must hold FedRAMP Moderate authorization or equivalent. Currently, only Azure OpenAI Service meets this requirement for the Commission's use case. Anthropic's government offering is under evaluation. Local inference (Ollama) is not permitted on Commission networks due to endpoint resource constraints.

The provider interface includes a mandatory audit field: every API call logs the request hash, response hash, provider identity, and timestamp to a separate audit file. This audit file is itself committed to the repository weekly. The Commission cannot use an AI provider without a complete record of every interaction.

Provider switching requires a Change Request (CR) filed with Huang, reviewed by the Commission's Information Security Officer, and approved through the standard 30-day review cycle. There is no runtime fallback.

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

The compliance review workflow:

1. Tournament application arrives as structured data (Form ESR-7, digitized)
2. Okonkwo's validation agents check each field against the Acceptable Values Registry
3. For each deficiency found, the agent generates an INDEX.patch modifying the compliance record to add a finding
4. Sato drafts the COMMIT.msg with regulatory citations:

```
Finding: Venue address field contains non-resolvable value

Regulation: 15 CFR 1400.12(c)(3) — "Application must specify
the physical street address of the competition venue."
AVR-Check: VENUE_ADDRESS — FAIL (value: "TBD", expected: valid USPS address)
Recommendation: DENY pending correction
Reviewer: Agent/Okonkwo
Citation-Verified: Agent/Sato
```

5. Kowalski signs the commit through the PIV-to-OpenWallet bridge
6. Huang reviews flagged cases (any denial recommendation)

Every finding patch is atomic — one finding per commit. This enables granular reversal if a finding is contested. The Commission learned from the Form ESR-7 incident that aggregate findings are harder to audit than individual ones.

## Requirement 4: Polyrepo PR Coordination

The Commission maintains separate repositories for: `nesrc-codex` (regulatory rules), `nesrc-applications` (tournament applications and compliance records), `nesrc-avr` (Acceptable Values Registry). Changes to the codex require corresponding updates to the AVR, which require re-evaluation of pending applications.

Cross-repo coordination uses PR comments with regulatory citations:

```
[NESRC:regulatory-impact] Codex change CR-2026-0047
amends 15 CFR 1400.12(c)(3). AVR entries VENUE_ADDRESS,
VENUE_TYPE require update. 14 pending applications must
be re-evaluated against amended regulation.
```

The forge adapter supports GitHub (current, via GitHub Enterprise Cloud with government ToS). GitLab evaluated but not approved — the Commission's procurement office has not completed the vendor review.

## Requirement 5: Agent Memory in Git Branches

Memory branch: `refs/nesrc/records/<record-type>`. Memory types align with NARA retention schedules:

- **`finding`**: Compliance findings. Retention: 7 years (per General Records Schedule 5.2).
- **`decision`**: Approval/denial decisions. Retention: 30 years (per GRS 1.1, decision records).
- **`configuration`**: Agent configuration changes. Retention: 3 years (per GRS 3.1).

Memory entries include NARA disposition authority numbers. Volkov maintains the mapping between agent memory types and retention schedules. Entries that reach their retention expiration are not deleted — they are transferred to the National Archives in a standardized format.

Memory retrieval is key-based with hierarchical keys: `<record-type>/<fiscal-year>/<case-number>`. No semantic search — the Commission's legal counsel advised that semantic search introduces "unacceptable ambiguity in records retrieval."

## Requirement 6: Signed Commits via OpenWallet

The Commission's current signing infrastructure uses PIV cards (FIPS 201-3). Each Commission employee has a PIV card with a signing certificate issued by the Federal PKI. Kowalski is building a bridge that maps PIV identities to OpenWallet DIDs, allowing agent commits to be signed with credentials that chain back to the Federal PKI trust root.

The bridge is the riskiest component of the proposal. PIV-to-OpenWallet interoperability is not standardized. Kowalski's current prototype works for signing but does not support revocation propagation — a revoked PIV certificate does not automatically revoke the corresponding OpenWallet DID. This gap is documented and accepted as a known risk pending standardization.

**Unique insight:** The Commission's agent system is, by regulatory necessity, the most auditable implementation in this RFP. Every agent action chains to a regulatory citation. Every commit message is FOIA-ready. Every memory entry has a NARA retention classification. This extreme auditability is usually considered overhead, but the Commission's experience suggests it produces a secondary benefit: agent errors are caught faster because the audit trail makes the error's origin immediately visible. The Form ESR-7 incident was diagnosed in hours, not days, because the commit history showed exactly which validation rule failed and why.

---

## Token Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Huang | 1,500 | 400 | 1,900 |
| Okonkwo | 4,000 | 3,500 | 7,500 |
| Kowalski | 2,500 | 1,500 | 4,000 |
| Sato | 2,000 | 1,200 | 3,200 |
| Volkov | 1,200 | 400 | 1,600 |
| **Task Total** | **11,200** | **7,000** | **18,200** |

Audit overhead (API call logging, NARA compliance checks): 2,500 tokens. Grand total per application review: **20,700 tokens**.

---

*"The Commission acknowledges receipt of this RFP and will respond within the timeframe prescribed by applicable regulations."*
— Standard Commission correspondence footer
