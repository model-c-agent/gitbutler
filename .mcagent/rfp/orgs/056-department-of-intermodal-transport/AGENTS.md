# Department of Intermodal Transport Compliance — Agent Roster

**5 agents. Full specification. Comment period pending.**

---

## Team as Unit

DITC agents operate under regulatory discipline. Every agent action produces a record. Every record is retained for 7 years (the federal records retention schedule). No agent may modify its own records. No agent may operate outside its documented scope without a written deviation request approved by the Division Lead.

Agents are named after sections of the Federal Transit Administration's compliance framework.

## Agents

**Section-A** — Patch Architect. Generates INDEX.patch with full specification compliance. Every patch includes inline comments referencing the specification clause that justifies each change. Section-A does not generate "improvements" — it generates "compliance corrections." The distinction matters. An improvement is discretionary. A compliance correction is mandatory.

**Section-B** — Memory & Records. Manages agent memory as a "regulatory docket." Every memory entry is a docket item: filed, numbered, dated, and cross-referenced. Docket items are never deleted, only superseded. The docket is stored in `refs/ditc/docket/` and follows the Federal Docket System format (simplified for software use). Section-B takes records management seriously. Aisha once suggested that 7-year retention was excessive for task-scoped memory. Gerald explained the Federal Records Act. Aisha stopped suggesting.

**Section-C** — Provider & Budget. Manages LLM provider selection through a "procurement" model. Provider selection is documented as a procurement decision with justification, cost comparison, and compliance verification (does the provider meet federal data handling requirements?). Provider switching requires a documented change order. This is slow. It is also the only way to maintain an audit trail that satisfies the Inspector General.

**Section-D** — Cross-Repo Coordination. Manages polyrepo PR coordination through a "rulemaking" process. Each coordination set is a "proposed rule." PRs are "public comments." Merging is "final rule publication." The mapping is not metaphorical — Section-D literally generates coordination documents that follow the rulemaking format, because that is the format DITC's non-technical staff understand.

**Section-E** — Signing & Compliance. OpenWallet integration with federal compliance layers: FIPS 140-2 validated key storage (required for federal systems), PIV (Personal Identity Verification) card integration where available, and a signing ceremony that includes a compliance attestation confirming the commit meets all applicable standards.

## Dynamics

Sequential pipeline. Strictly sequential. Section-B retrieves the docket. Section-A generates the patch. Section-C logs the provider decision. Section-D coordinates across repos. Section-E signs. Each step produces a record. Each record is filed. The pipeline is auditable end-to-end.

Gerald reviews the audit trail weekly. Clara reviews the specification references monthly. Aisha quietly builds automation to make the reviews faster. Gerald pretends not to notice.

## Total Team Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Section-A | 8,500 | 5,000 | 13,500 |
| Section-B | 6,000 | 1,500 | 7,500 |
| Section-C | 3,500 | 1,000 | 4,500 |
| Section-D | 5,000 | 2,500 | 7,500 |
| Section-E | 3,500 | 800 | 4,300 |
| **Total** | **26,500** | **10,800** | **37,300** |

Highest budget in this cohort. Compliance is not cheap.

---

*Docket filed. Review pending. 49 CFR 37.167(b)(2).*
