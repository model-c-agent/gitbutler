# FreqGhost — Technical Proposal

**RFP:** `but-ai` Plugin for GitButler
**Approach:** Evidence-Integrity Spectrum Analysis

---

## Executive Summary

FreqGhost proposes a local-only, anonymity-preserving agent system for automated spectrum occupancy analysis. Agents refine analysis methodology, re-process historical data, and cross-reference findings across surveys. Every finding is signed pseudonymously. Every methodology change triggers re-evaluation of affected publications. The system prioritizes evidence integrity above all other concerns.

---

## Requirement 1: PATH-Based Plugin Architecture

The `but-ai` binary installs to `~/.local/bin/but-ai`. Compiled from Rust, statically linked, no network capability (network calls stripped at compile time, same approach as /gg/noRe). The binary ships with a reproducibility manifest — a hash of the compiler version, feature flags, and build inputs — so that third parties can verify they built the same binary from the same source.

Subcommands: `but ai analyze` (run occupancy analysis on survey data), `but ai reprocess` (re-analyze historical data with updated methodology), `but ai crossref` (identify patterns across surveys), `but ai scrub` (anonymize output before publication).

The `scrub` command is essential to the collective's operational security. It removes measurement timestamps (which could reveal member schedules), precise GPS coordinates (replaced with neighborhood-level locations), and any SDR-specific identifiers (serial numbers, firmware versions). The output after scrubbing is still scientifically valid but does not compromise member identity.

## Requirement 2: Provider-Agnostic AI

Local only. Ollama with Codestral (for signal processing code generation) and Mistral (for report analysis and cross-referencing). No cloud providers. The collective's data never leaves a member's machine.

The provider interface: `analyze(survey_data, methodology_version) -> Findings`. Single method. The `methodology_version` parameter is mandatory — every finding is tied to the methodology that produced it. When the methodology is updated, the agent knows which findings to re-evaluate.

No fallback. If Ollama is unavailable, the member runs the analysis manually. The agents are productivity tools, not dependencies.

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

The analysis pipeline:

1. deadband's detection agent processes raw measurement data and identifies occupancy events
2. spectral's analysis agent computes occupancy statistics per frequency band
3. sideband's cross-reference agent compares findings against historical data and licensee records
4. The agent generates INDEX.patch updating the survey report:

```diff
+ {
+   "band_mhz": "1710-1755",
+   "licensee_inferred": "Carrier-A (FCC ULS cross-reference)",
+   "occupancy_pct": 7.3,
+   "measurement_days": 60,
+   "detection_method": "energy-threshold-v4.2",
+   "confidence": 0.91,
+   "classification": "severely-underutilized"
+ }
```

5. COMMIT.msg:

```
Analyze: 1710-1755 MHz occupancy, survey-portland-2026-Q1

Occupancy: 7.3% (60-day weighted average)
Licensee: Carrier-A (inferred from FCC ULS database)
Detection: energy-threshold-v4.2
Previous-Analysis: 8.1% (v4.1 method, superseded)
Change-Reason: v4.2 improves detection of bursty low-power signals
Scrubbed: yes (timestamps removed, coordinates generalized)
```

6. carrier_sense reviews for information leakage, then signs

## Requirement 4: Polyrepo PR Coordination

One repo per survey (city + time period). Cross-survey coordination handles pattern propagation:

```
[FG:crossref] survey-portland-2026-Q1#14 ↔ survey-seattle-2025-Q3#8
Carrier-A 1710-1755 MHz occupancy: 7.3% (Portland), 6.8% (Seattle).
Cross-market pattern: consistent under-utilization across Pacific NW.
Note: this is a correlation, not a causal claim.
```

sideband strips all causal language from cross-reference comments. The forge adapter targets the collective's self-hosted Forgejo instance (Tor-hosted). A read-only mirror on GitHub provides public access to published datasets.

## Requirement 5: Agent Memory in Git Branches

Memory branch per survey: `refs/fg/memory/<survey-id>`. Memory types:

- **`methodology-version`**: Serialized analysis parameters for each methodology version. TTL: permanent. Methodology versions are never expired — they are needed to reproduce any finding.
- **`finding`**: Published findings with methodology version and confidence score. TTL: until superseded by re-analysis.
- **`crossref-link`**: Links between findings across surveys. TTL: 2 years. Cross-references are valuable but decay as new data accumulates.
- **`detection-benchmark`**: Fixed benchmark results for regression testing detection algorithms. TTL: permanent.

Memory retrieval is key-based: `<survey-id>:<band>:<methodology-version>`. squelch enforces a strict rule: raw measurement data is never stored in memory branches (it is too large and too sensitive). Memory contains analysis outputs only.

## Requirement 6: Signed Commits via OpenWallet

Each member has a pseudonymous OpenWallet DID. DIDs are not linked to any real-world identity. The DID proves continuity — the same analyst who published the 2024 Portland survey also published the 2026 update — without revealing who that analyst is.

carrier_sense manages keys on air-gapped hardware. Key rotation every 90 days. Revocation requires two-member co-signature (same threshold scheme as /gg/noRe, which FreqGhost admires but has no formal relationship with).

Publication signing adds a collective attestation: at least three members' DIDs co-sign every published dataset. This proves that the finding was reviewed by a quorum of the collective, not produced by a single member acting independently.

**Unique insight:** FreqGhost's re-processing pipeline — automatically re-analyzing historical data when the methodology improves — solves a problem endemic to data-driven agent systems: methodological drift. When an agent's analysis method changes, all previous outputs generated by the old method become potentially inaccurate. Most systems ignore this — they publish the old results and the new results side by side, without reconciliation. FreqGhost's approach is to treat methodology as a first-class version: every finding is tagged with the methodology version that produced it, and when the methodology changes, the system identifies all affected findings and queues them for re-analysis. The result is a publication corpus that is always internally consistent — every finding reflects the current best methodology, not the methodology that happened to be current when the survey was conducted. This pattern applies to any agent system where the analysis method evolves over time: machine learning models, static analysis rules, compliance standards.

---

## Token Budget

| Handle | Input | Output | Total |
|--------|-------|--------|-------|
| spectral | 3,500 | 2,000 | 5,500 |
| deadband | 3,000 | 2,500 | 5,500 |
| sideband | 2,500 | 1,500 | 4,000 |
| carrier_sense | 1,500 | 500 | 2,000 |
| squelch | 1,500 | 800 | 2,300 |
| **Task Total** | **12,000** | **7,300** | **19,300** |

Re-processing overhead (when methodology changes): 8,000 tokens per historical survey. This is the largest variable cost — a methodology change triggers re-analysis across the entire publication corpus. The collective considers this non-negotiable: publishing stale findings undermines credibility.

---

*"The spectrum is not empty. It is hoarded. Here are the numbers."*
