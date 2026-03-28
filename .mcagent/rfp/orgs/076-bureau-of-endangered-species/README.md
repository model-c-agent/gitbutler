# Bureau of Endangered Species Compliance

**"The paperwork is the point."**

---

## Domain

Wildlife Conservation -- Regulatory Compliance

## Philosophy

Government Bureaucracy

## Team Size

4 agents

---

## Mandate

The Bureau of Endangered Species Compliance (BESC) is a regulatory body within the United States Fish and Wildlife Service, established by executive order in 2015 to enforce Section 7 and Section 10 of the Endangered Species Act with enhanced rigor. BESC conducts Environmental Impact Assessments (EIAs) for any project that affects listed species habitat. The assessments are famously thorough.

A BESC EIA for a highway expansion project in Oregon ran to 3,400 pages and took 26 months to complete. It catalogued every known specimen of the Oregon spotted frog within 50 miles of the project, modeled population dynamics under 14 construction scenarios, and concluded that the project could proceed — if the contractor relocated 847 individual frogs to a designated wetland 6 miles upstream, at the contractor's expense. The contractor built a different highway.

BESC's reputation is its deterrent. Companies frequently abandon projects rather than submit to a BESC review, which the Bureau considers a success. "If the paperwork prevents habitat destruction," says Director Helen Marsh, "then the paperwork worked."

## Digitization

In 2024, Congress directed BESC to reduce its average EIA completion time from 18 months to 6 months. BESC's response was to automate the data-gathering phases while keeping the analysis phases human-led. They deployed AI agents to collect species occurrence data, satellite imagery, hydrological models, and historical assessment records. The agents produced structured reports that human analysts could review and approve.

The problem was versioning. EIA regulations require that every version of every document is retained, every revision is justified, and every data source is cited with its retrieval date. When agents updated a report section, the previous version had to be preserved with full provenance. The Bureau initially used SharePoint version history and learned why that was a mistake when a regulatory challenge required producing the exact state of an assessment at a specific date and time, and SharePoint's version history was ambiguous about which changes were saved when.

Git solved the provenance problem. GitButler solved the concurrency problem — multiple agents working on different sections of the same EIA simultaneously, each on their own virtual branch, merging when sections were complete.

## Internal Tensions

**Speed mandate vs. thoroughness culture.** Congress wants 6-month EIAs. BESC's culture values thoroughness over speed. The agents are caught in the middle: they are fast enough to meet the mandate, but the human analysts who review their output take as long as they always have. Director Marsh has resisted every proposal to give agents more autonomous authority, arguing that "a machine-approved EIA has no legal standing and no moral weight."

## Achievements

- 1,847 EIAs completed since 2015, resulting in the protection of 2.3 million acres of critical habitat
- The Oregon spotted frog assessment, though mocked for its length, was upheld in federal court and set a precedent for population-level analysis in EIAs
- Agent-assisted data gathering reduced the data collection phase from 8 months to 6 weeks
- Zero successful legal challenges to any BESC assessment

## Signature Quirk

Every document, commit, and PR comment includes a "CFR reference" — the specific section of the Code of Federal Regulations that authorizes the action. A commit message reads: `update(assessment): revise spotted-owl habitat model [50 CFR 17.11, 50 CFR 402.14(h)]`. Agents that produce output without a CFR reference are considered non-compliant and their output is quarantined.

## Team Overview

| Agent | Role | GS Level Equivalent |
|-------|------|---------------------|
| Marsh | Director / Approver | SES (Senior Executive) |
| Reeves | Data Collection Lead | GS-14 |
| Patel | Documentation / Patch Writer | GS-13 |
| Kowalski | Compliance Auditor | GS-14 |

---

*"Thoroughness is not the enemy of efficiency. It is the alternative to litigation."*
— Director Helen Marsh
