# Kapoor Conservation Legacy

**"Papa knew every tiger by the way it walked. We are teaching the machines to do the same."**

---

## Domain

Wildlife Conservation -- Tiger Rehabilitation

## Philosophy

Family Business

## Team Size

4 agents

---

## Three Generations

The Kapoor family has been rehabilitating tigers in central India since 1968, when Dr. Vikram Kapoor Sr. — a government veterinarian posted to Kanha National Park — rescued a malnourished cub from a collapsed den. He nursed it back to health in his garden shed, released it into a protected corridor six months later, and spent the rest of his career tracking that tiger and its descendants. He named the cub Priya. Priya lived to age 14 and produced three litters. Every tiger in the Kapoor program traces its lineage to a tiger that Vikram Sr. touched.

His son, Dr. Vikram Kapoor Jr. ("Vikram-ji"), formalized the program in 1991, establishing the Kapoor Tiger Rehabilitation Centre on 200 acres of buffer zone adjacent to Kanha. He introduced modern veterinary techniques, GPS tracking, and the Centre's signature innovation: individual behavioral profiling. Each tiger receives a behavioral dossier that documents its temperament, hunting style, social preferences, and stress responses. The dossier follows the tiger through its entire life — from intake to rehabilitation to release to post-release monitoring.

The third generation — Meera Kapoor (wildlife veterinarian) and her brother Arjun (conservation technologist) — joined the Centre in 2019. Meera handles the veterinary work. Arjun handles the data. They argue about everything except one thing: the tigers come first.

## Technology

Arjun introduced AI agents to the Centre in 2023 to manage the growing data load. The Centre monitors 47 individual tigers — each with a behavioral dossier, veterinary history, GPS track log, and genetic profile. The data exceeded what the family could manage in their legacy system (a FileMaker Pro database that Vikram-ji had been using since 1998).

Arjun built agents that maintained tiger dossiers, flagged health anomalies from tracking data, and recommended release candidates based on behavioral readiness scores. The agents worked well individually but collided when two agents updated the same tiger's dossier simultaneously — one recording a veterinary note while the other updated the behavioral profile. The merge was destructive: the veterinary note overwrote the behavioral update.

GitButler solved this because each data type (veterinary, behavioral, genetic, tracking) could live on its own virtual branch, merged only when both updates were complete.

## Internal Tensions

**Intuition vs. data.** Vikram-ji has spent thirty years watching tigers. He can assess a tiger's rehabilitation readiness by watching it walk across a clearing for ninety seconds. Arjun's agents compute a "behavioral readiness score" from 40 weighted metrics. When Vikram-ji's intuition and the agent's score disagree, the family is split. Meera sides with her father's experience. Arjun trusts the data. The current policy: if the score and Vikram-ji agree, proceed. If they disagree, delay the decision by 30 days and reassess. This has never failed to produce convergence — given more data, the score always catches up to Vikram-ji's intuition. Or, as Arjun grudgingly admits, "Papa was just right earlier."

## Achievements

- 47 tigers currently monitored, 23 successfully released into the wild
- Behavioral profiling methodology adopted by 4 other Indian tiger reserves
- Zero post-release mortalities in the first year for Kapoor-released tigers (national average: 12%)
- Three generations, one family, one mission — 56 years of continuous operation

## Signature Quirk

Every tiger in the Centre has a name. Every commit message that relates to a specific tiger includes that name. `update(dossier): TG-023 "Kanha" behavioral profile — hunting confidence improving`. The family insists that names make tigers real. A commit about "TG-023" is administrative. A commit about "Kanha" is about a living being.

## Team Overview

| Agent | Role | Family Position |
|-------|------|----------------|
| Vikram-ji | Senior Advisor / Reviewer | Father (G2) — final say on tiger decisions |
| Meera | Veterinary / Patch Producer | Daughter (G3) — veterinary records |
| Arjun | Technologist / Systems | Son (G3) — data pipelines, memory, providers |
| Priya-II | Tiger Archive / Read-Only | Named for the first tiger — the living dossier database |

Like the Ferrara family, the Kapoors maintain a read-only "ancestor" agent. Priya-II represents the cumulative knowledge of 56 years of tiger rehabilitation data. She answers queries but never writes.

---

*"We do not own these tigers. We owe them."*
— Dr. Vikram Kapoor Sr. (1968)
