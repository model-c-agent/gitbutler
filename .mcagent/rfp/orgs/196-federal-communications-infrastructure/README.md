# Federal Communications Infrastructure Review Board

**"Application FCIRB-2024-00847 is under review. Estimated processing time: 14 months."**

---

## Statutory Authority

The Federal Communications Infrastructure Review Board (FCIRB) was established by the Telecommunications Infrastructure Modernization Act of 2022 as an independent review body within the Department of Commerce. Its mandate: review and approve all proposed modifications to federally licensed communications infrastructure, including cell tower installations, spectrum reallocation requests, satellite ground station permits, and emergency communications system upgrades.

FCIRB is not the FCC. The FCC issues licenses. FCIRB reviews infrastructure changes *after* licensing, ensuring that proposed modifications comply with environmental regulations, historical preservation requirements, flight path clearances, and the 847-page Federal Communications Infrastructure Standards Manual (FCISM), which was last fully updated in 2019 and is amended quarterly through Federal Register notices that no single person has read in their entirety.

The Board has 62 employees: 8 engineers, 23 review analysts, 14 administrative staff, 9 legal counsel, and 8 management. The Board processes approximately 3,200 applications per year with an average review time of 14 months. This review time is a source of persistent Congressional criticism, industry frustration, and internal despair. The Board's technology stack includes a custom Oracle database from 2011, a document management system that requires Internet Explorer, and 23 filing cabinets.

## The Modernization Initiative

In 2024, Congress mandated that FCIRB reduce average review time to 6 months. Congress did not provide additional funding. The Board's Chief Technology Officer, Dr. Helen Park, submitted a modernization proposal that included AI-assisted review agents. The proposal was approved after a 10-month interagency review (which Dr. Park notes was longer than the target review time the agents were supposed to achieve).

The agents would ingest infrastructure modification applications, cross-reference them against the FCISM, environmental databases, FAA flight path data, and historical preservation registries, and produce compliance assessments. Human review analysts would review the assessments, not the raw applications. The goal: reduce analyst review time from 40 hours per application to 8 hours.

The agents needed an auditable trail. The Government Accountability Office had been clear: automated government decisions must be reproducible, attributable, and reversible. Dr. Park chose Git for the audit trail and GitButler for agent workflow management after a procurement process that took five months and generated 340 pages of documentation to acquire a free, open-source tool.

## Philosophy

The Board does not have a philosophy. The Board has procedures. Procedure 7.4.2: "All infrastructure modification reviews shall be conducted in accordance with the standards set forth in the FCISM, as amended." Procedure 7.4.3: "Automated review tools shall produce outputs that conform to the documentation standards in Section 12 of the FCISM." Procedure 7.4.4: "No automated tool shall render a final determination. Final determinations are the exclusive authority of certified review analysts."

Agents do not decide. Agents assess. The distinction is legally significant.

## The Retroactive Amendment Crisis

In August 2025, FCIRB published FCISM Amendment 2025-Q3, which modified the environmental review thresholds for small cell installations. The amendment was retroactive to January 2025. This meant that 74 applications already reviewed and approved under the old threshold needed re-evaluation.

The review agents re-processed the 74 applications against the amended standard. Twelve were flagged as non-compliant under the new threshold. Three of those twelve had already been built. The Board issued stop-work orders. The applicants sued, arguing that retroactive application of amended standards to completed installations violated due process. The case is pending.

The incident demonstrated both the value and the risk of automated re-evaluation. The agents correctly identified the non-compliant applications in hours — a task that would have taken analysts weeks. But the speed of identification created a political problem: the Board was forced to act on findings it would have preferred to discover slowly.

## Achievement

**Pilot results: 62% reduction in analyst review time.** During the six-month pilot (January-June 2025), applications processed with agent assistance had an average analyst review time of 15.2 hours, compared to 40 hours for traditional review. The pilot was conducted on low-complexity applications only (Category 1 and 2). High-complexity applications (Category 3-5) have not yet been tested.

## Staff

| Member | Title | Role |
|--------|-------|------|
| Dr. Helen Park | Chief Technology Officer | Architecture, modernization lead |
| Marcus Webb | Senior Review Engineer | FCISM encoding, compliance logic |
| Janet Liu | Systems Analyst | Plugin deployment, infrastructure |
| Derek Osei | Records Manager | Memory, retention, NARA compliance |
| Carmen Reyes | Legal Technologist | Regulatory citation, COMMIT.msg compliance |

Details in [AGENTS.md](AGENTS.md).

---

*"The Board appreciates your patience during the review process."*
— Automated email footer, sent 3,200 times per year
