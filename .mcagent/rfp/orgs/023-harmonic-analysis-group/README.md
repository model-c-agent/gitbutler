# The Harmonic Analysis Group

**"Reproducibility is the only form of honesty we trust."**

---

## Origin

The Harmonic Analysis Group formed in 2017 inside the acoustics department at ETH Zurich, when three postdocs and two PhD candidates discovered they were all independently publishing papers on spectral decomposition of mixing console workflows. Their advisor, Prof. Margrit Fehr, told them to either collaborate or stop duplicating each other's literature reviews. They chose collaboration, grudgingly.

What began as a shared LaTeX repository became a research lab. The group treats music production the way particle physicists treat collider data: every session is an experiment, every mix is a dataset, every mastering decision requires a hypothesis, a control, and a p-value. They have published 34 peer-reviewed papers on topics ranging from "Perceptual Thresholds for Reverb Tail Truncation" to "A Bayesian Framework for Snare Drum EQ Decisions."

Their lab occupies two rooms in the ETH main building. One room contains $400,000 worth of measurement microphones, anechoic chamber panels, and a Neve 8078 console they acquired from a defunct London studio. The other room contains whiteboards covered in transfer function equations. The Neve has never been cleaned. The whiteboards have never been photographed -- group policy forbids sharing unfinished derivations.

## Why Software

In 2024, the group received a Swiss National Science Foundation grant to study "reproducibility in computational audio processing." The premise: if two engineers apply the same EQ curve to the same audio file in the same DAW, do they get bit-identical output? The answer, after eight months of testing across 14 DAWs, was no. Floating-point rounding, plugin load order, buffer size, and even the operating system's thread scheduler introduced measurable variation.

This horrified them. The group pivoted from studying reproducibility in audio to building tools that enforce it. Their first tool, `repromix`, generated deterministic audio processing pipelines from declarative YAML manifests. The second tool, `diffwav`, produced unified diffs between audio files at the sample level. The third tool was a Git-based versioning system for mix sessions that tracked every parameter change as a commit.

When the `but-ai` RFP appeared, Dr. Kenji Tanaka -- the group's most software-inclined member -- argued that the INDEX.patch workflow was structurally identical to their `diffwav` approach. The group saw an opportunity to apply their reproducibility framework to a domain where it might actually matter at scale.

## Philosophy

The group operates on three axioms:

1. **If it is not reproducible, it is not real.** A result that cannot be independently verified is an anecdote, not a finding.
2. **Measurement precedes intuition.** No design decision is accepted without quantitative justification. "It feels right" is not an argument.
3. **Peer review is not optional.** Every artifact -- code, paper, patch, commit message -- is reviewed by at least one other group member before it leaves the lab.

They apply these axioms with the rigidity you would expect from people who spent their formative years in Swiss academia. Deadlines are suggestions. Rigor is not.

## Team

The group has five members. Prof. Fehr is the nominal advisor but does not participate in day-to-day work. The four active researchers function as equals, though Dr. Tanaka is the de facto technical lead by virtue of being the only one who enjoys writing Rust.

| Member | Focus | Background |
|--------|-------|------------|
| **Dr. Kenji Tanaka** | Systems, Rust, deterministic builds | PhD: spectral analysis of vinyl mastering artifacts |
| **Dr. Lina Voss** | Signal processing, formal verification | PhD: perceptual models of dynamic range compression |
| **Matteo Ferrara** | Statistics, experiment design | PhD candidate: Bayesian optimization of mix parameters |
| **Priya Chandrasekaran** | Psychoacoustics, human factors | PhD candidate: listener fatigue in automated mastering |

## Internal Tension

The group's central disagreement is about formalism. Tanaka and Voss believe the proposal should include formal proofs of correctness for critical components -- specifically, that the token budget system should be provably bounded and that the patch generation workflow should be formally verified to preserve semantic equivalence. Ferrara and Chandrasekaran argue that formal verification is disproportionately expensive for a plugin proposal and that statistical guarantees (with confidence intervals) are sufficient.

This argument has been running for four months. The compromise visible in their PROPOSAL.md is Tanaka's: formal invariants for the signing and budget systems, statistical guarantees for everything else. Voss considers this a defeat. Ferrara considers it a victory. Both are wrong -- it is a truce.

## Notable Achievement

In 2025, the group published a paper in the Journal of the Audio Engineering Society demonstrating that their `repromix` tool could reproduce a professional mastering chain across three different operating systems with less than 0.1 dB variation. The paper was cited 47 times in its first year -- remarkable for a niche topic. More importantly, it caught the attention of two major DAW vendors, who invited the group to consult on deterministic rendering pipelines. The group declined both invitations because the vendors required NDAs, and the group does not sign documents that restrict publication.

## Signature Quirk

Every document produced by the group includes a "confidence interval" in the header metadata. This README has a CI of 95% -- meaning the group is 95% confident that the factual claims herein are accurate. Their PROPOSAL.md has a CI of 87%, because it contains predictions about system behavior that have not yet been empirically validated. They are not joking.

---

*CI: 95%. Sample size: 4 researchers, 34 publications, 1 unreproducible argument about formalism.*
