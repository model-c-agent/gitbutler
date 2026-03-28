# National Culinary Standards Authority

**"If it is not defined, it is not authentic."**

---

## Domain

Culinary Arts -- Regulatory Standards

## Philosophy

Government Bureaucracy

## Team Size

4 agents

---

## Background

The National Culinary Standards Authority (NCSA) was established in 2018 by the Italian Ministry of Agricultural, Food, and Forestry Policies as a semi-autonomous regulatory body tasked with codifying, protecting, and enforcing the designation standards for regional Italian cuisine. The NCSA occupies three floors of a renovated palazzo in Bologna, operates with an annual budget of EUR 4.2M, and employs forty-seven people, thirty-one of whom are lawyers.

The Authority's founding act was the "Authentic Ragu Standard" — a four-year effort to legally define the term "authentic Bolognese ragu" for use in product labeling, restaurant certification, and export documentation. The process involved 312 submissions from regional cooks, 47 expert hearings, a public comment period that generated 1,800 responses (including a 90-page dissent from a grandmother in Modena), and a final standard that runs to 94 pages and specifies everything from the minimum fat content of the pancetta to the acceptable variance in soffritto dice size (3mm +/- 0.5mm).

The ragu standard took four years. The NCSA considers this fast.

## How We Got Here

In 2025, the Authority was tasked with digitizing its standards enforcement pipeline. Restaurants seeking regional certification had been submitting paper applications — handwritten ingredient lists, notarized photographs of kitchens, and sworn statements from suppliers. The backlog was fourteen months. The Ministry demanded modernization.

The NCSA contracted a software firm to build a digital submission portal. The portal worked, but now the backlog was digital — 6,000 applications sitting in a queue that no human could process faster than fifty per day. Someone suggested AI-assisted review. The NCSA's legal department spent three months writing a memo on whether an AI system could make a "determination of authenticity" under Italian administrative law. The memo concluded: no, but it could make a "preliminary assessment subject to human ratification."

The Authority built an agent-based assessment pipeline. Each application was reviewed by an AI agent that compared the submission against the relevant standard, flagged discrepancies, and produced a preliminary report. A human reviewer then ratified or rejected the report. Processing time dropped from 14 months to 6 weeks.

But the agents needed version control. Standards change. When the NCSA updates the acceptable range of pecorino aging from "minimum 8 months" to "minimum 10 months," every pending assessment must be re-evaluated against the new standard. The agents were overwriting each other's assessments, and nobody could determine which version of a standard an assessment was based on. The Authority discovered GitButler while searching for version control that could handle concurrent, multi-standard assessment workflows.

## Internal Tensions

**The Speed vs. Thoroughness Debate.** Deputy Director Gallo believes that AI assessment should be fast — the whole point is reducing the backlog. Chief Standards Officer Bianchi believes that speed is the enemy of accuracy and that a rushed determination is worse than a delayed one because an incorrect certification undermines the Authority's credibility. They have been arguing about response time SLAs for eight months. The current compromise — 48-hour preliminary assessment, 5-business-day ratification — satisfies neither of them.

## Achievements

- Reduced certification backlog from 14 months to 6 weeks
- Published 23 regional cuisine standards, each averaging 60+ pages
- The "Authentic Ragu Standard" has been cited in 4 EU trade disputes
- Zero successful legal challenges to NCSA certifications in 6 years

## Signature Quirk

Every document produced by the NCSA includes a "Standard Reference Number" (SRN) — a unique identifier linking the document to the specific version of the standard it references. Commit messages follow the same convention. An agent's commit message always begins with `SRN-<standard-id>-<version>:` before any description. The NCSA believes that if you cannot trace an action to its governing standard, the action has no legal basis.

## Team Overview

| Agent | Role | Focus |
|-------|------|-------|
| Bianchi | Standards Compliance Officer | Standard versioning, assessment accuracy |
| Gallo | Assessment Pipeline Manager | Throughput, queue management, SLA tracking |
| Ferraro | Documentation Specialist | Patch generation, COMMIT.msg formatting |
| Mancini | Audit & Verification | Signature verification, authorization trails |

Detailed agent profiles are in [AGENTS.md](AGENTS.md).

---

*"The process is the product. Shortcuts produce counterfeits."*
-- Director's Foreword, NCSA Annual Report 2024
