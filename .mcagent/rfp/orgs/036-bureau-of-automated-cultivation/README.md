# Bureau of Automated Cultivation

**"Form AG-7012 must be filed before any automated cultivation system may be deployed, modified, maintained, or discussed in a public forum."**

---

## Jurisdictional Authority

The Bureau of Automated Cultivation (BAC) is a regulatory body within the Department of Agriculture's Division of Mechanized Farming. Established by the Agricultural Automation Safety Act of 2020, the Bureau is charged with certifying autonomous agricultural equipment, maintaining a registry of approved cultivation algorithms, and investigating incidents involving automated farming systems.

The Bureau employs 187 staff across five branches: Certification, Registry, Investigation, Research, and — since 2025 — the Digital Systems Modernization Unit (DSMU). The DSMU exists because Congress added a line item to the Bureau's appropriation requiring it to "evaluate and, where appropriate, adopt emerging digital technologies for regulatory operations." The line item came with $340,000 in annual funding and no further guidance. The DSMU's director, Dr. Amara Osei, interprets this mandate as broadly as possible. Her supervisor, Deputy Director Hoffman, interprets it as narrowly as he can.

## The Problem We Are Paid to Solve

The Bureau's certification process for a new autonomous cultivation system takes, on average, 14 months. The growing season for most crops is 4-6 months. This means that by the time a system is certified, the season it was designed for has ended, and the next season has begun with new firmware that requires re-certification. The Bureau is aware of this temporal mismatch. The Bureau has formed a committee to study it.

The certification backlog is the Bureau's defining shame and its institutional justification: if the process were fast, you would not need 187 staff to administer it. The DSMU was created, in part, to explore whether AI-assisted review could reduce certification time. It has not yet succeeded, but it has produced three internal reports demonstrating that AI-assisted review is theoretically possible, which is enough to justify the next budget cycle.

## Why This RFP

The `but-ai` plugin's agent workflow — structured patches, auditable memory, signed commits — maps directly to the Bureau's certification requirements. If AI agents that modify agricultural software produce auditable, signed artifacts with traceable decision chains, the certification process could review those artifacts instead of the raw code. This would not eliminate the 14-month timeline, but it could reduce the human review component from 60% of total time to an estimated 25%.

Dr. Osei wrote the business case. Deputy Director Hoffman approved it "without endorsement." The distinction is bureaucratically meaningful and practically irrelevant.

## Team (DSMU)

| Name | Grade | Role |
|------|-------|------|
| **Dr. Amara Osei** | GS-15 | DSMU Director, systems architect |
| **Thomas Hoffman** | GS-14 | Deputy Director (oversight, not contribution) |
| **Lena Kowalski** | GS-13 | Lead engineer, certification automation |
| **Marcus Chen** | GS-12 | Memory systems, audit trail design |
| **Fatima Al-Rashid** | GS-11 | Provider integration, testing |

## Internal Tension

Dr. Osei wants the Bureau to contribute meaningful code to the `but-ai` ecosystem. Deputy Director Hoffman wants the Bureau to observe, evaluate, and perhaps adopt — but not build. The tension manifests in every meeting: Osei proposes a feature, Hoffman asks whether building features is within the Bureau's statutory authority, Osei cites the appropriation language, Hoffman cites the Bureau's enabling statute, and Lena builds the feature while they argue.

## Notable Achievement

In 2025, the DSMU completed a pilot project that used AI-assisted review to process three certification applications in 4 months instead of 14. The AI reviewed firmware change logs, flagged safety-relevant modifications, and produced structured review summaries that human certifiers could evaluate in hours instead of weeks. The pilot was considered a success by the DSMU and "an interesting experiment" by the certification branch, which continues to process applications at its original pace.

---

*BAC-DSMU-2026-012. Classification: Public.*
*Filed per Bureau Administrative Procedure 4.1.2.*
*Approved for release: Dr. Amara Osei, DSMU Director.*
*Noted without endorsement: Thomas Hoffman, Deputy Director.*
