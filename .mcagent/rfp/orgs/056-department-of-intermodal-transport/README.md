# Department of Intermodal Transport Compliance

**"Section 4.7.3(b): Bus stops shall be exactly seven feet in height, measured from the base of the mounting post to the upper edge of the signage panel."**

---

## Origin

The Department of Intermodal Transport Compliance (DITC) was established in 2008 by federal mandate to harmonize public transit standards across state and municipal jurisdictions. Its founding director, Dr. Margaret Huang-Patterson, was a transportation policy scholar who had spent fifteen years documenting the 847 distinct bus stop specifications in use across the fifty states and territories. Her doctoral thesis, "The Seven-Foot Problem" — named for the absurd variation in regulated bus stop heights (ranging from 5'4" to 8'11" across jurisdictions) — became the foundational document for the Department.

DITC employs 220 people across three offices (Washington DC, Denver, and Sacramento). The software division — the group submitting this proposal — consists of 14 engineers embedded within the Standards Implementation Branch. Their job is to build the software that transit agencies use to verify compliance with federal intermodal standards. Every software change goes through a 90-day comment period, a regulatory impact assessment, and a cross-division review before deployment.

The 90-day comment period is not a bug. It is law.

## Philosophy

Standards exist because inconsistency kills. Not metaphorically — literally. When a bus stop in one jurisdiction signals "stop here" with a red stripe and the adjacent jurisdiction signals "stop here" with a blue stripe, confusion becomes risk. DITC believes that inconsistency is the root of most system failures, and that the cure for inconsistency is specification. Detailed, precise, sometimes absurdly specific specification.

They apply this to software the same way: every API must be specified before implementation. Every data format must be documented with examples. Every error condition must have a defined response. If a behavior is not specified, it does not exist.

## Internal Tension

The software team's youngest engineers — hired between 2022 and 2025 — are in quiet revolt against the 90-day comment period. They have proposed an "expedited review" track for non-safety-critical software changes (14 days instead of 90). Dr. Huang-Patterson, now Deputy Secretary, rejected the proposal on the grounds that any change to the review process itself requires a 90-day comment period, and the comment period for changing the comment period has not yet been opened. The younger engineers cannot tell if this is bureaucratic wit or bureaucratic earnestness. They suspect both.

## Achievement

In 2024, DITC's compliance verification software detected that 340 transit agencies across 12 states were using a non-compliant bus stop mounting specification that placed the signage panel 2.3 inches below the regulated minimum height. The error traced to a typo in a widely-used CAD template distributed by a private vendor. DITC issued a compliance advisory, the vendor corrected the template, and 340 agencies updated their installations. The process took 14 months. Nobody was injured, because the non-compliance was 2.3 inches, not 2.3 feet. But rules are rules.

## Team (Software Division)

| Name | Role | Background |
|------|------|------------|
| Dr. Margaret Huang-Patterson | Deputy Secretary (Advisor) | Policy scholar, "The Seven-Foot Problem" |
| Gerald Okoye | Division Lead / Software Architect | 16 years federal IT, ex-FHWA |
| Clara Jimenez | Senior Engineer / Standards | Specification writing, ex-NIST |
| Dmitri Volkov | Backend Engineer | Compliance engines, ex-USDA data systems |
| Aisha Patel | Junior Engineer | Full-stack, 2 years out of CMU, impatient |

Agent profiles in [AGENTS.md](AGENTS.md). Technical proposal in [PROPOSAL.md](PROPOSAL.md).

## Signature Quirk

Every commit message includes a CFR (Code of Federal Regulations) citation for the standard most relevant to the change. Example: `fix: correct height validation threshold [49 CFR 37.167(b)(2)]`. Commits that do not reference a standard are flagged in review as "ungrounded."

---

*"Comment period open. Ninety days. Sincerely, The Department."*
