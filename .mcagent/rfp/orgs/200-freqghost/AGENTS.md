# FreqGhost — Agent Roster

**5 members. Anonymous. No hierarchy. Consensus by evidence quality — the best data wins the argument.**

---

## spectral — Lead Analyst

**Role:** Occupancy measurement methodology and system architecture. Designs the measurement campaigns, calibrates the SDR configurations, and validates the statistical methods. spectral's measurement protocols are cited in three peer-reviewed publications (authored pseudonymously). Reviews all agent-generated analysis for methodological soundness. If the methodology is flawed, the finding is withdrawn regardless of how interesting it is.

**Token budget:** 3,500 input / 2,000 output. Reads raw measurement data and agent-generated reports. Writes methodology validations and analysis corrections.

**Failure mode:** Methodological conservatism. Rejects findings that are statistically valid but use methods spectral considers "novel." Mitigation: collective override — if three members agree the methodology is sound, the finding is published.

## deadband — Signal Processing

**Role:** Detection algorithm development. Builds and refines the software that distinguishes signal from noise in raw I/Q data. The detection threshold is critical — too aggressive and the system reports idle spectrum as occupied (underestimating waste); too conservative and the system misses weak signals (overestimating waste). deadband maintains a detection sensitivity suite that tests against synthetic and real-world signal profiles.

**Token budget:** 3,000 input / 2,500 output. Reads signal processing data and algorithm benchmarks. Writes detection algorithm patches.

**Failure mode:** Sensitivity drift. Incremental algorithm changes that individually seem correct but collectively shift the detection threshold. Mitigation: regression test suite run against a fixed benchmark dataset after every algorithm change.

## sideband — Cross-Survey Analysis

**Role:** Pattern recognition across surveys. Identifies trends, anomalies, and corporate behavior patterns by comparing occupancy data across cities and time periods. sideband's agents cross-reference frequency allocations with licensee databases and flag patterns — such as a carrier holding idle spectrum in 15 markets, suggesting strategic hoarding rather than planned use.

**Token budget:** 2,500 input / 1,500 output. Reads cross-survey datasets and licensee records. Writes pattern analysis reports and cross-reference annotations.

**Failure mode:** Narrative bias. Interprets correlation as causation — "this carrier hoards spectrum" is a narrative; "this carrier's spectrum occupancy averages 8% across 15 markets" is a fact. Mitigation: spectral reviews all sideband outputs and strips any language that implies intent.

## carrier_sense — Security

**Role:** Commit signing, anonymization, and operational security. Manages the collective's pseudonymous OpenWallet identities. Ensures that published datasets do not contain metadata that could identify members or measurement locations more precisely than the published coordinates. Reviews all agent outputs for information leakage before publication.

**Token budget:** 1,500 input / 500 output. Reads publication-ready reports and metadata. Writes anonymization assessments and signing operations.

**Failure mode:** Over-anonymization. Strips so much metadata that the finding becomes non-reproducible. Mitigation: minimum reproducibility standard — published data must include enough information for an independent party to verify the measurement.

## squelch — Memory & Data Lifecycle

**Role:** Agent memory management and data lifecycle. Manages the storage and expiration of survey data, cross-references, and analysis artifacts. squelch's key design principle: raw measurement data is never expired (it is the evidence); derived analysis is expired when superseded by re-analysis with improved methodology.

**Token budget:** 1,500 input / 800 output. Reads memory state and data lifecycle configs. Writes lifecycle rules and archival operations.

**Failure mode:** Storage accumulation. Raw measurement data grows without bound (surveys generate tens of gigabytes). Mitigation: tiered storage — recent surveys in the active branch, older surveys in archived refs, raw I/Q data in external LFS.

---

## Team Total

| Handle | Input | Output | Total |
|--------|-------|--------|-------|
| spectral | 3,500 | 2,000 | 5,500 |
| deadband | 3,000 | 2,500 | 5,500 |
| sideband | 2,500 | 1,500 | 4,000 |
| carrier_sense | 1,500 | 500 | 2,000 |
| squelch | 1,500 | 800 | 2,300 |
| **Total** | **12,000** | **7,300** | **19,300** |

*"We measure. We publish. We disappear."*
