# The Lagrange Debris Observatory

**"We do not predict collisions. We observe, catalog, and let the data speak."**

---

## Origin Story

The Lagrange Debris Observatory (LDO) was established in 2017 through a memorandum of understanding between MIT Lincoln Laboratory, the University of Western Australia's International Centre for Radio Astronomy Research (ICRAR), and the European Southern Observatory (ESO). The founding proposition, authored by Dr. Elara Konstantopoulos of MIT, was straightforward: the space debris tracking community was producing models without sufficient observational validation. Everyone was predicting. No one was measuring.

Konstantopoulos had spent a decade at Lincoln Lab operating the Haystack radar for NASA's Orbital Debris Program Office. She knew the radar data. She also knew its gaps. Haystack could detect objects down to 1cm at 1,000km altitude, but it was a staring radar — it observed a fixed patch of sky and counted whatever crossed its beam. That gave you statistical population estimates, not individual object tracks. For small debris (1-10cm), which accounts for over 900,000 objects and poses the greatest collision risk to active spacecraft, the community was relying on statistical models extrapolated from a few hundred detectable events per year.

Konstantopoulos wanted a dedicated network. She recruited Dr. Raj Acharya from ICRAR (who operated radar arrays for radio astronomy and understood phased-array beam forming at a level no one in the debris community could match) and Dr. Ingrid Halvorsen from ESO (who had developed optical debris tracking techniques using decommissioned telescopes in Chile). Together they built LDO: three radar/optical sites on three continents providing continuous coverage of the most congested orbital regimes.

The sites:

- **Haystack Auxiliary (Westford, Massachusetts, USA):** A dedicated S-band radar operating 24/7 for debris staring observations. Operated by Lincoln Lab staff under LDO's data-sharing agreement.
- **Murchison Array (Murchison, Western Australia):** A repurposed radio astronomy array adapted for debris radar tracking. Operated by ICRAR.
- **Paranal Optical (Cerro Paranal, Chile):** Two decommissioned 1.8m telescopes repurposed for optical debris tracking. Operated by ESO.

By 2025, LDO's catalog contained 1.2 million tracked objects — three times the size of the US Space Command's public catalog — because LDO tracked smaller objects with higher cadence and published everything openly.

## The Turn Toward Agents

LDO's data pipeline was designed for astronomers, not for operational space traffic management. The pipeline ingested raw radar returns and optical images, processed them into orbit determinations, and stored them in a PostgreSQL database. Catalog updates were published daily as a bulk data release. This was fine for research but useless for anyone who needed to know "will object X hit satellite Y in three days?"

The problem was scale. Each of the 1.2 million tracked objects needs periodic orbit updates (every 3-14 days depending on orbit altitude and uncertainty), conjunction screening against all active satellites (~12,000 as of 2025), and data quality assessment (is this a real object or a processing artifact?). LDO's seven-person team was spending 90% of its time on data pipeline maintenance and 10% on the research that justified the consortium's existence.

Dr. Fatima Al-Rashidi, LDO's data systems lead, proposed deploying AI agents to handle routine catalog maintenance. The agents would not make scientific judgments. They would process raw data into orbit determinations, screen for conjunctions against a known satellite catalog, and flag anomalies for human review. Every output would be a structured patch to the debris catalog — never a direct database write.

The patch-based approach came from Al-Rashidi's experience with astronomical data pipelines, where every catalog update is versioned and auditable. GitButler's INDEX.patch + COMMIT.msg workflow mapped perfectly: each agent produces a catalog update as a patch, a reviewer verifies it, and the orchestrator applies it. No agent writes to the database directly.

## Philosophy

LDO is an academic consortium. Its principles reflect the scientific method:

1. **Observation trumps prediction.** Models are hypotheses. Radar returns are data. When they disagree, the data wins. LDO agents never speculate — they report what they observe and flag what they cannot explain.

2. **Every measurement has an uncertainty.** An orbit determination without an error bar is not an orbit determination — it is a guess. LDO agents always report confidence intervals, observation counts, and data quality metrics. No number is presented without context.

3. **Reproducibility is non-negotiable.** If another team cannot reproduce LDO's result from the same raw data, the result is invalid. Every agent operation must be reproducible: given the same inputs and memory state, the same patch must be produced. This means deterministic prompts, versioned tool configurations, and logged random seeds.

4. **Open data, open methods.** LDO publishes everything — catalog data, processing code, agent configurations, and even agent memory. If LDO's agents have a bias, the community should be able to identify it.

## Internal Tensions

The primary tension is between American pragmatism and European rigor. Konstantopoulos (MIT) wants to ship useful products to the space traffic management community — conjunction alerts, risk assessments, maneuver recommendations. Halvorsen (ESO) wants to publish scientifically rigorous papers and considers "products" a corruption of the research mission. Acharya (ICRAR) mediates, noting that useful products attract funding that supports rigorous research.

The secondary tension is about automation. Al-Rashidi and Dr. Tomoko Ishikawa (LDO's conjunction specialist) want agents to handle more of the pipeline. Dr. Marcus Oyelaran (LDO's optical processing lead) and Dr. Lena Berger (quality assurance) resist, arguing that automated processing without sufficient human oversight will eventually produce a catalog error that damages LDO's scientific credibility. Dr. Sven Lindqvist, the newest postdoc, is caught between the camps and has diplomatically avoided declaring a position by focusing on "infrastructure that makes oversight easier."

This is a healthy tension. The automation advocates push for efficiency. The oversight advocates push for rigor. The result is a system that automates routine work while maintaining human review at every critical junction.

## Achievements

- **LDO Small-Debris Catalog** (ongoing): 1.2 million tracked objects, updated daily. The most comprehensive public small-debris catalog in existence. Used by 14 space agencies, 30 satellite operators, and 200+ academic groups.
- **Multi-Continental Radar Fusion** (2021): The data fusion algorithm that combines radar returns from three continents into a single orbit determination. Published in Advances in Space Research. 340 citations.
- **Optical-Radar Cross-Calibration** (2023): Demonstrated that optical and radar debris tracking can be cross-calibrated to sub-centimeter accuracy for objects > 5cm. Published in Journal of Spacecraft and Rockets.
- **Agent-Assisted Catalog Maintenance Pilot** (2025): AI agents handling 60% of routine orbit updates with 99.1% agreement against human analyst baselines. The 0.9% disagreement triggered 47 investigations, 44 of which found the agent correct and the human analyst had used stale data.
- **Conjunction Screening Service** (2025): Automated conjunction alerts for LDO catalog objects against active satellite positions. 50 subscribers. Free for academic use.

## Failures

- **Real-Time Processing Attempt** (2022): Tried to process radar returns in real time instead of batch. The data volume overwhelmed the pipeline. Reverted to batch processing with 30-minute cadence.
- **Machine Learning Orbit Propagator** (2023): Trained a neural network to propagate orbits faster than numerical integration. It was 5x faster and 10x less accurate. Published as a negative result (Halvorsen insisted). Filed in the "interesting but not ready" cabinet.
- **Cross-Agency Data Integration** (2024): Attempted to merge LDO's catalog with USSC's public catalog. The catalogs used different reference frames, different epoch conventions, and different object identification schemes. Six months of work produced a 15% match rate. The remaining 85% required manual cross-referencing. LDO now maintains its own catalog independently.

## Signature Quirk

Every LDO document includes a data quality header inspired by astronomical observation logs: observation type (radar, optical, or analytical), confidence interval, number of independent observations confirming the document's claims, and an epoch (the document's reference time). A README based on historical records has high observation count and a long epoch. A PROPOSAL.md based on original analysis has moderate observation count and a shorter epoch. An AGENTS.md describing new agents has low observation count ("we have designed them but not yet validated them in production").

This is academic pedantry. LDO is proud of it.

## Team Composition

| Name | Affiliation | Role | Specialty | Joined |
|------|-------------|------|-----------|--------|
| **Dr. Elara Konstantopoulos** | MIT Lincoln Lab | Director / PI | Radar debris tracking, population modeling | Founding (2017) |
| **Dr. Raj Acharya** | ICRAR / UWA | Radar Systems Lead | Phased-array radar, beam forming, signal processing | Founding (2017) |
| **Dr. Ingrid Halvorsen** | ESO | Optical Systems Lead | Optical debris tracking, astrometry, photometry | Founding (2017) |
| **Dr. Fatima Al-Rashidi** | MIT Lincoln Lab | Data Systems Lead | Data pipelines, agent architecture, distributed systems | 2019 |
| **Dr. Tomoko Ishikawa** | JAXA (seconded) | Conjunction Analysis Lead | Conjunction assessment, probability of collision, risk analysis | 2020 |
| **Dr. Marcus Oyelaran** | University of Lagos (affiliated) | Optical Processing Lead | Image processing, photometric reduction, artifact detection | 2021 |
| **Dr. Lena Berger** | ESO | Quality Assurance Lead | Data quality, statistical validation, anomaly detection | 2022 |

Seven researchers across four institutions on three continents. They collaborate via weekly video calls, shared Git repositories, and a monthly in-person workshop that rotates between the three sites. Decisions are made by consensus among the three PIs (Konstantopoulos, Acharya, Halvorsen), with Al-Rashidi holding veto power on technical architecture.

---

*Observation: Radar + Analytical*
*Confidence: 95% (based on 8 years of operational data)*
*Independent observations: 4 (MIT, ICRAR, ESO, JAXA)*
*Epoch: 2026-03-28T00:00:00Z*
