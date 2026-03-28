# Department of Intermodal Transport Compliance — Technical Proposal for `but-ai`

**RFP Response | Tier 2 Abbreviated | Docket DITC-2026-0043**

---

## Executive Summary

The Department of Intermodal Transport Compliance proposes a `but-ai` implementation built on specification-driven development: every agent action is traceable to a requirement, every output is documented, and every record is retained per federal guidelines. Our domain expertise in compliance verification across 847 jurisdictions translates to agents that produce auditable, specification-compliant output at the cost of speed.

## Requirement 1: PATH-Based Plugin Architecture

Rust binary at `$PATH` as `but-tool-ai`. The binary maintains a complete invocation log at `$XDG_STATE_HOME/but-ai/invocation.log` — every call, every argument, every exit code, every timestamp. The log format follows the NIST SP 800-92 log management standard (simplified). The binary validates its own integrity on startup using a FIPS 140-2 compliant hash check.

Configuration via `$XDG_CONFIG_HOME/but-ai/config.toml`. The configuration file includes a version field and a change history comment block. Config changes without an updated version number are rejected.

## Requirement 2: Provider-Agnostic LLM Interface

Four-provider backend. Section-C manages provider selection as a documented procurement decision. Each provider is evaluated against:
- Data handling compliance (cloud providers must meet FedRAMP equivalents for sensitive workloads).
- Cost per token (documented).
- Quality benchmark (documented).
- Availability SLA (documented).

Provider switching is permitted only via change order. The change order specifies the old provider, the new provider, the justification, and the expected cost impact. Change orders are logged in the docket.

**Domain Insight:** In federal compliance, every decision must be defensible to an auditor who asks "why did you choose this?" three years after the fact. Section-C applies this principle: every provider selection is accompanied by a justification that can be retrieved from the docket indefinitely. This is expensive in storage and annotation overhead. It is also the only way to operate in regulated environments without risk of audit failure.

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Section-A generates patches as "compliance corrections." Each patch:
1. **References the specification** — Every change references the requirement it implements or the defect it corrects.
2. **Includes impact assessment** — COMMIT.msg contains a "regulatory impact" section estimating the scope of the change.
3. **Follows the Standard** — Code formatting, naming, and structure conform to DITC's internal coding standard (a 47-page document that Clara maintains and updates quarterly).

COMMIT.msg format:
```
fix(height-validation): correct threshold from 84.0 to 84.276 inches

Regulatory Impact: Low. Affects validation module only.
Specification: 49 CFR 37.167(b)(2), DITC-STD-2024-007 Section 4.3
Agent: Section-A
Docket: DITC-2026-0044
Provider: Section-C procurement order PO-2026-112
```

Patches are verbose. The annotations cost output tokens. DITC considers this the cost of compliance.

## Requirement 4: Polyrepo PR Coordination

Section-D manages cross-repo coordination as a "rulemaking process":
- **Notice of Proposed Rulemaking (NPRM)** — Section-D opens coordination PRs in all affected repos, announcing the proposed change set.
- **Comment Period** — Human reviewers and automated checks provide feedback.
- **Final Rule** — After review, Section-D approves the coordination set for merge.

Coordination documents are stored in `refs/ditc/rulemaking/<rule-id>/`. Forge adapters (GitHub, GitLab, Gitea) implement the DITC comment format, which includes structured metadata fields.

Section-D will not merge a coordination set if any PR has unresolved comments. All comments must be resolved or explicitly acknowledged.

## Requirement 5: Agent Memory in Git Branches

Section-B manages memory as a regulatory docket. Each entry:

| Field | Description |
|-------|-------------|
| `docket_number` | Sequential (DITC-YYYY-NNNN) |
| `filed_date` | ISO 8601 |
| `category` | task, project, standard, precedent |
| `content` | Memory payload |
| `cross_references` | Related docket entries |
| `retention_years` | 7 (federal minimum) |
| `status` | active, superseded |

Memory stored in `refs/ditc/docket/<category>/`. Memory is never deleted within the retention period. Superseded entries are marked as such but remain accessible. This means the memory footprint grows monotonically. DITC accepts this because retention compliance is non-negotiable.

Retrieval uses full-text search with docket number cross-referencing. When Section-A requests context, Section-B returns both the matching entries and their cross-references, ensuring no relevant precedent is missed.

## Requirement 6: Signed Commits via OpenWallet

Section-E handles signing with federal compliance requirements:
- Key storage in FIPS 140-2 validated module (software or hardware).
- Signing ceremony includes a compliance attestation field: "This commit conforms to [standard references]."
- Signature metadata includes the docket number for the task.
- Key rotation every 90 days with 30-day overlap.
- All signing operations logged to the invocation log and the docket.

Section-E refuses to sign commits missing required fields (specification reference, docket number, regulatory impact assessment).

## Token Budget

| Agent | Role | Input/task | Output/task | Total |
|-------|------|-----------|-------------|-------|
| Section-A | Patch + specifications | 8,500 | 5,000 | 13,500 |
| Section-B | Docket management | 6,000 | 1,500 | 7,500 |
| Section-C | Provider procurement | 3,500 | 1,000 | 4,500 |
| Section-D | Rulemaking coordination | 5,000 | 2,500 | 7,500 |
| Section-E | Signing + compliance | 3,500 | 800 | 4,300 |
| **Per-task total** | | **26,500** | **10,800** | **37,300** |

Highest budget in this cohort. Approximately 20% of output tokens are consumed by specification annotations and compliance metadata. This is the cost of operating in a regulated domain. We do not apologize for it.

## Unique Domain Insight

Eighteen years of federal transit compliance taught us that the most expensive errors are the ones that look correct until an auditor examines them three years later. A bus stop that is 2.3 inches too short looks fine. It passes casual inspection. It fails formal verification. The cost of fixing 340 non-compliant installations is orders of magnitude greater than the cost of getting the specification right in the first place.

Our proposal applies this insight to agent memory: every decision an agent makes is documented with full context and retained for years. When a future task involves code that a previous agent modified, the docket provides the complete history of why changes were made, what specifications they referenced, and whether they were later superseded. This level of documentation is unusual in agent systems. We believe it is essential for any system that operates on code with long-term maintenance requirements.

---

*Docket DITC-2026-0043. Comment period: 90 days. Section 4.7.3(b) applies. Sincerely, The Department.*
