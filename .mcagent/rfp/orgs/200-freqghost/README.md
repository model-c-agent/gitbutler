# FreqGhost

**"They bought the spectrum. They never used it. We found it."**

---

## Who We Are

FreqGhost is an anonymous research collective that maps unused radio spectrum — frequencies that have been licensed to corporations or government agencies but sit idle, wasting bandwidth that communities could use. We publish our findings as open datasets. We do not advocate for policy. We present evidence. The evidence speaks.

The collective has no public members, no office, no website (our datasets are published through Tor-hosted repositories), and no legal entity. We communicate through encrypted channels. We sign our publications with pseudonymous keys. We have been operating since 2022.

What we can say: the collective includes RF engineers, data scientists, a lawyer who advises on spectrum policy, and several programmers. Some of us have worked for telecom operators. Some of us currently work for telecom operators. We are careful.

## What We Do

FreqGhost conducts spectrum occupancy surveys: systematic measurements of which frequencies are being used, by whom, and how much of the time. We deploy measurement equipment (software-defined radios, directional antennas, autonomous logging stations) in urban and suburban areas and record occupancy data continuously for periods of 30-90 days.

The findings are consistent across every market we have surveyed (17 cities, 4 countries): between 40% and 70% of licensed spectrum below 6 GHz is unused at any given time. Corporations hold licenses worth billions of dollars for frequencies they transmit on sporadically or not at all. Meanwhile, community networks, emergency services, and rural ISPs compete for the scraps.

Our datasets include: frequency, bandwidth, occupancy percentage (time-weighted), signal strength, estimated licensee (inferred from FCC/Ofcom databases), and measurement methodology. Every measurement is reproducible — we publish the SDR configuration, antenna specifications, and recording parameters.

## The Agent Problem

Our analysis pipeline is largely automated. SDRs collect raw I/Q data. Signal processing software identifies occupancy events. Statistical analysis computes occupancy metrics. The pipeline produces structured reports as JSON files, one per frequency band per survey period.

The reports are stored in Git. Each survey is a repository. Each frequency band is a file. The analysis history — from raw measurement to published finding — is a sequence of commits. When our analysis methodology improves (which happens frequently as we refine our signal detection algorithms), we re-process historical data and produce updated reports as new commits.

In 2025, we built agents to automate the analysis refinement loop. An agent reviews published findings, compares them against new methodology improvements, identifies findings that would change under the improved methodology, and proposes re-analysis patches. The patches modify the occupancy reports with updated statistics and an annotation explaining what changed and why.

The agents also cross-reference occupancy data across cities to identify patterns — for example, a corporation that hoards spectrum in every market it holds licenses in, or a frequency band that is consistently underutilized across all surveyed markets.

The `but-ai` RFP aligns with our needs: signed commits (our credibility depends on provenance), agent memory (cross-survey pattern recognition requires historical context), and local inference (we do not send spectrum data to cloud APIs for the same reason we do not publish our members' names).

## Philosophy

Information wants to be free. Spectrum wants to be used. We measure, we publish, we let the public decide what to do with the information. We do not lobby. We do not protest. We do not break laws (spectrum monitoring is legal in every jurisdiction we operate in; spectrum *usage* is not what we do). We are empiricists, not activists.

Our agents follow the same principle: they analyze, they do not advocate. An agent report that says "this frequency is 95% idle" is a fact. An agent report that says "this frequency *should* be reallocated" is an opinion. Our agents produce facts only.

## The Corporate Pressure Campaign

In late 2025, a major U.S. wireless carrier issued a public statement claiming that FreqGhost's datasets were "misleading and technically flawed." They did not specify which datasets or which flaws. We responded by publishing our full measurement methodology, raw data, and analysis code for the disputed survey. The carrier did not respond further.

Separately, two of our members received legal threats (cease and desist letters regarding "unauthorized spectrum analysis"). Our lawyer confirmed that spectrum monitoring is protected activity. The letters were published (redacted to protect member identity) alongside our legal analysis.

The incident reinforced two principles: (1) published methodology must be impeccable — if our methods have a flaw, our opponents will find it, and (2) signed commits matter — when our findings are challenged, we need cryptographic proof that the data has not been altered since publication.

## Achievement

**EU Parliament citation**: FreqGhost's pan-European spectrum occupancy dataset (covering London, Berlin, Amsterdam, and Barcelona) was cited in the European Parliament's 2025 report on spectrum efficiency, contributing to the recommendation that idle spectrum licenses be subject to "use it or lose it" provisions. The recommendation is under committee review.

## Members

| Handle | Specialty |
|--------|-----------|
| spectral | Lead analyst, occupancy measurement, architecture |
| deadband | Signal processing, detection algorithms |
| sideband | Cross-survey analysis, pattern recognition |
| carrier_sense | Security, signing, anonymization |
| squelch | Memory systems, data lifecycle |

Details in [AGENTS.md](AGENTS.md).

---

*"The spectrum does not belong to the license holder. It belongs to physics."*
