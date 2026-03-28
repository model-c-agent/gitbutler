# Orbital Threat Reduction Command

**"Every object in orbit is a weapon until proven otherwise. Track it, model it, deorbit it."**

---

## Origin Story

Orbital Threat Reduction Command (OTRC) was established in 2020 as a spinoff from Raytheon's Space Situational Awareness division. The founding team — five engineers who had spent a combined forty years tracking things in space — left the defense contractor after a bureaucratic dispute over whether a particular piece of debris at 780km altitude constituted a "threat" requiring immediate action or an "anomaly" requiring a six-month study.

The object in question was a defunct Soviet weather satellite that was tumbling end-over-end at 7.5 km/s in an orbit that intersected the ISS protection zone every 11 days. Raytheon's risk assessment classified it as "moderate." The five engineers classified it as "unacceptable." They wrote a dissent memo. The memo was filed. They resigned the next morning.

Colonel (Ret.) James "Hound" Hartley, who had commanded USAF space surveillance units before transitioning to the private sector, incorporated OTRC with a single guiding principle: every piece of space junk is a hostile target until it is safely deorbited. There is no "moderate" threat classification. There is "tracked and modeled" and there is "removed." The space between those two states is where catastrophes happen.

OTRC's first contract was with the European Space Agency, which needed an independent assessment of Kessler syndrome risk in the 700-850km band — precisely where the debris density was highest and Raytheon's models were most optimistic. OTRC's assessment was blunt: without active debris removal, the band would become unusable within 15 years. ESA was not pleased with the timeline. They were pleased with the rigor.

## The Turn Toward Software Agents

OTRC's operational model is tracking and modeling. They operate no spacecraft. They remove no debris. They tell others where the threats are, how they are evolving, and when they will become critical. This is an information-processing operation, and by 2024, the information was outpacing their ability to process it.

The problem: the small-debris population (objects 1-10cm) was growing faster than human analysts could catalog it. OTRC's radar cross-section database contained 340,000 tracked objects, each requiring periodic orbit updates, conjunction assessments against active satellites, and threat scoring. A single conjunction assessment takes 20 minutes for an experienced analyst. With 50-100 new conjunctions per day, the team was spending 80% of its time on routine assessments and 20% on the complex cases that actually required human judgment.

Major (Ret.) Priya Chandrasekaran, OTRC's software lead, proposed deploying AI agents to handle routine conjunction assessments. Hartley was skeptical — he had seen AI systems in defense contexts produce confident, well-formatted, and entirely wrong threat assessments. Chandrasekaran argued that the agents would not make decisions. They would produce structured assessments in a reviewable format (patches to the threat database), and human analysts would approve or reject them. The agent's output was a patch. The human's role was to apply it.

That conversation — patches as the agent's write primitive, humans as the approval layer — is what led OTRC to the GitButler RFP. The patch-based workflow was not just compatible with OTRC's operations. It *was* OTRC's operations.

## Philosophy

OTRC operates on three principles, all derived from military doctrine:

1. **Positive identification before engagement.** No action is taken on an object until its orbit is confirmed by independent observation. No patch is applied until it is verified by an independent reviewer. Assumptions are threats.

2. **Mission creep is a kill chain.** An agent that starts cataloging debris and ends up proposing deorbiting strategies has suffered mission creep. Every agent has a specific mission with defined boundaries. Crossing those boundaries is an operational failure, not initiative.

3. **Redundancy is not waste.** Two agents assessing the same conjunction independently is not duplication — it is verification. OTRC would rather spend twice the tokens and be right than spend half and be wrong.

## Internal Tensions

The primary tension at OTRC is between Hartley's conservative doctrine and Dr. Leon Vasiliev's push for automation. Vasiliev, a former Roscosmos orbital mechanics specialist, believes that the debris problem is growing too fast for human-in-the-loop processing and that agents should be authorized to apply low-risk patches autonomously. Hartley flatly refuses. Every patch is reviewed. Every time. No exceptions.

The secondary tension is cultural. Hartley and Chandrasekaran come from US military backgrounds with rigid command hierarchies. Vasiliev comes from Russian space program culture, which is hierarchical in theory but improvisational in practice. Cpt. (Ret.) Nneka Obiora, the threat assessment lead, is Nigerian-British and has worked in both NATO and African Union space monitoring programs, giving her a diplomatic flexibility the others lack. Technical Sergeant (Ret.) Kyle Marsh, the youngest member, is an enlisted veteran who bridges the officer-enlisted divide and has the most hands-on experience with the actual radar systems.

These five people function as a military unit: clear chain of command, defined roles, mandatory briefings, and a shared conviction that space debris is an existential threat that most of the world is not taking seriously enough.

## Achievements

- **Kessler Risk Assessment, 700-850km Band** (2021): The ESA-commissioned study that established OTRC's reputation. Predicted band unusability within 15 years. Now cited in policy documents by four space agencies.
- **OTRC Threat Catalog** (ongoing): 340,000 tracked objects with orbit elements, radar cross-sections, and threat scores. Updated daily. The most comprehensive non-classified small-debris catalog in existence.
- **Conjunction Assessment Automation Pilot** (2025): AI agents handling routine conjunction assessments with 97.3% accuracy against human analyst baselines. The 2.7% disagreement rate is within acceptable parameters — and in every case of disagreement, the agents were more conservative than the humans.
- **Strike Package Planner** (2025): A planning tool that generates optimal deorbiting mission profiles for debris clusters. Used by two commercial debris-removal companies for mission planning.

## Failures

- **Autonomous Threat Scoring** (2024): Attempted to let agents assign threat scores without human review. An agent assigned a "low" threat score to a debris object that was in a decaying orbit intersecting Starlink shell 2. Human review caught it before any downstream action. The agent had correctly computed the conjunction probability but used an outdated orbital element set. Hartley's response: "This is why we review everything."
- **Real-Time Conjunction Alerting** (2024): Attempted to generate conjunction alerts in real time from radar data. The false positive rate was 23%, which meant analysts spent more time dismissing false alerts than they saved. Reverted to batch processing with a 30-minute cadence.
- **Cross-Agency Data Sharing** (2025): Attempted to share OTRC's catalog with JAXA and ISRO through an automated pipeline. The pipeline worked. The agencies' data formats did not. Spent four months on format conversion before securing agreements on a common schema. The lesson: interoperability is a political problem, not a technical one.

## Signature Quirk

Every OTRC document includes an orbital parameters header: inclination, altitude range, and epoch (the reference time for the document's accuracy). A README has a high inclination (broad coverage), high altitude (strategic overview), and a long epoch (stable, infrequently updated). A PROPOSAL.md has lower altitude (more specific) and a shorter epoch (more time-sensitive). An AGENTS.md has medium parameters across the board.

This is not whimsy. It is operational discipline. Every piece of information has a shelf life, and OTRC believes in making that explicit.

## Team Composition

| Name | Rank/Title | Role | Specialty | Joined |
|------|------------|------|-----------|--------|
| **Col. (Ret.) James "Hound" Hartley** | Director | Strategic leadership, doctrine, customer relations | Space surveillance command, 22 years USAF | Founding (2020) |
| **Maj. (Ret.) Priya Chandrasekaran** | Software Lead | Agent architecture, systems design, tool integration | Software systems engineering, 14 years USN | Founding (2020) |
| **Dr. Leon Vasiliev** | Orbital Mechanics Lead | Orbit determination, conjunction assessment, propagation models | Celestial mechanics, 18 years Roscosmos | Founding (2020) |
| **Cpt. (Ret.) Nneka Obiora** | Threat Assessment Lead | Threat scoring, risk analysis, inter-agency coordination | Space object characterization, 10 years RAF/AU | 2021 |
| **TSgt. (Ret.) Kyle Marsh** | Radar Operations Lead | Sensor integration, data processing, field deployment | Radar operations, 8 years USSF | 2022 |

Command structure is clear: Hartley commands. Chandrasekaran leads technical execution. Vasiliev leads the science. Obiora leads threat assessment. Marsh runs the sensors. Orders flow down. Intelligence flows up.

---

*Orbital parameters: i=90deg, alt=2000km, epoch=2026-03-28T00:00:00Z*
*Classification: UNCLASSIFIED // FOUO*
*Track status: Active surveillance*
