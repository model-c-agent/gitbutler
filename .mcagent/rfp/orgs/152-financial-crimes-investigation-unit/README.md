# Financial Crimes Investigation Unit

**"We pursue with the persistence of homicide detectives. Money leaves a trail. We follow it."**

---

## Unit Profile

**Designation:** FCIU
**Established:** 2011
**Commander:** Lt. Col. Reyes (Ret.)
**Operational Tempo:** 24/7 watch rotation
**Active Cases:** Classified
**Closure Rate:** 91.3% (FY2025)

---

## History

The Financial Crimes Investigation Unit was stood up in 2011 as a joint task force combining military financial investigators, civilian forensic accountants, and law enforcement analysts under a unified command structure. The unit's founding mandate: apply military-grade investigative persistence to financial crime.

The idea came from Lt. Col. Elena Reyes, who spent twenty years as an Army Criminal Investigation Division financial crimes investigator before retiring. Her observation: financial crime investigations fail not because the evidence is insufficient but because investigators give up. The trail goes cold. The statute of limitations approaches. The suspect's lawyers file motions. Institutional pressure to close cases and move on grinds down even dedicated investigators.

Reyes's unit does not give up. The longest active case has been open for nine years. The unit has twice petitioned for statute of limitations extensions and received them both times. When asked how long she will pursue a case, Reyes answers: "Until the math stops working or I stop breathing."

The unit operates with military discipline adapted for civilian investigation. Shift rotations ensure 24/7 coverage. Handoff protocols guarantee that no investigator's departure disrupts a case. Standing operating procedures cover everything from evidence handling to bathroom breaks during surveillance.

## The Software Transition

In 2022, the unit's digital forensics section was overwhelmed. Cases increasingly involved cryptocurrency, cross-border wire transfers, and corporate structures spanning dozens of jurisdictions. The section's tools -- forensic accounting software designed for single-entity audits -- could not handle the graph complexity.

Major Obi, the unit's technical officer, proposed building a graph-based investigation platform. The platform would model financial relationships as a directed graph: entities as nodes, transactions as edges, with properties for amount, timestamp, and jurisdiction. AI agents would traverse the graph, flagging anomalous patterns for human review.

The platform was built on Git because Obi, a former Army Cyber Command engineer, believed that every analytical step in a criminal investigation should be version-controlled. "If this goes to trial, I need to show the jury exactly what the software did, in what order, and why."

The unit discovered GitButler when merge conflicts between agents disrupted an active investigation. Virtual branches eliminated the conflicts. The `but-ai` RFP represents the next step: formalizing the agent pipeline.

## Philosophy

### On Persistence

Financial criminals depend on their pursuers' impatience. They structure transactions to create complexity. They use jurisdictions with slow mutual legal assistance treaties. They hire accountants to produce layers of obfuscation. The only countermeasure is persistence. Our agents do not get tired. Our agents do not have billable hour quotas. Our agents follow the graph until it terminates.

### On Discipline

Discipline is not rigidity. It is the consistent application of proven procedures under varying conditions. Every investigation follows the same protocol. Every agent follows the same operational checklist. This makes our work reproducible, defensible, and resistant to individual failure.

### On AI

AI agents are investigators, not analysts. An analyst interprets. An investigator follows. Our agents follow transaction chains, flag anomalies, and report findings. They do not interpret. Interpretation is a human function that requires judgment, context, and accountability that cannot be delegated to a model.

## Tension

**The Autonomy Debate.** Major Obi wants to give agents more autonomy -- the ability to follow a transaction chain across jurisdictions without waiting for human authorization at each border. Reyes insists on human authorization for cross-jurisdiction analysis because jurisdictional boundaries are legal boundaries, and an agent that crosses them without authorization creates evidence that may be inadmissible. Obi says the authorization delay costs days. Reyes says inadmissible evidence costs cases.

## Achievement

In 2025, the unit completed Operation Meridian: a three-year investigation into a trade-based money laundering network that used over-invoiced imports to move $340 million across 14 countries. The unit's AI agents reconstructed the invoice chain by traversing a transaction graph with 2.1 million edges. The graph reconstruction took 72 hours of continuous agent operation. Human investigators spent four months verifying the result. Seventeen arrests in six countries followed. The case is the largest TBML prosecution in the unit's history.

## Unit Roster

| Callsign | Rank/Role | Focus |
|----------|-----------|-------|
| SIGMA-1 | Investigation Lead | INDEX.patch, evidence assembly, graph traversal |
| SIGMA-2 | Technical Officer | Provider abstraction, infrastructure, token management |
| SIGMA-3 | Watch Officer | Forge adapters, cross-repo coordination, 24/7 coverage |
| SIGMA-4 | Evidence Custodian | Commit signing, chain of custody, OpenWallet |

Detailed profiles in [AGENTS.md](AGENTS.md).

## Operational Procedures

The unit operates on a watch rotation: SIGMA-3 maintains 24/7 awareness of all active cases. Shift handoff follows SOP FCIU-OPS-007: the outgoing watch produces a situation report (SITREP) and the incoming watch acknowledges receipt before assuming the watch.

All communication uses structured message formats. Free-form text is prohibited in operational channels. Every message includes a case reference, classification, and priority.

---

*"The trail does not go cold. The investigator goes cold. We do not."*
-- Lt. Col. Reyes, unit founding address, 2011
