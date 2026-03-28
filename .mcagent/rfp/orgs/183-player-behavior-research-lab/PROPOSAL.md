# Player Behavior Research Lab — Technical Proposal

**RFP:** `but-ai` Plugin for GitButler
**Classification:** Research Prototype → Production Plugin

---

## Executive Summary

PBRL proposes an advisory agent system grounded in cognitive science. Agents do not merely generate patches — they annotate those patches with cognitive load estimates, confidence intervals, and alternative approaches. The system treats every developer interaction as a data point in a behavioral model, enabling agents that improve their recommendations over time.

---

## Requirement 1: PATH-Based Plugin Architecture

The `but-ai` binary is a Rust CLI compiled to a single static binary, installed to `~/.gitbutler/bin/`. The architecture separates the advisory layer (cognitive models, pattern detection) from the execution layer (patch generation, commit signing). This separation is deliberate: advisory outputs are non-destructive (metadata, annotations), while execution outputs are destructive (file modifications, commits). The two layers have different failure modes and different rollback strategies.

The binary exposes subcommands: `but ai advise` (advisory only, no mutations), `but ai patch` (generates INDEX.patch), `but ai apply` (applies patch + signs). The separation allows developers to use the advisory layer without granting execution authority — a key requirement from our ethics review board.

## Requirement 2: Provider-Agnostic AI

Provider abstraction follows a trait-based architecture. Each provider implements `CognitiveProvider`: `analyze(context) -> CognitiveState`, `generate(state, task) -> Patch`, `embed(text) -> Vector`. The trait is deliberately minimal — three methods, no optional extensions. PBRL's experience with game analytics APIs taught us that wide interfaces accumulate dead methods.

Provider selection is per-task, not global. Advisory tasks (low-stakes, high-frequency) route to cheaper providers. Patch generation (high-stakes, lower-frequency) routes to more capable providers. Sonja maintains a provider capability matrix that maps task types to minimum provider requirements.

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Amir's cognitive model drives patch generation. The workflow:

1. Hayashi analyzes recent Git history and produces a cognitive state estimate (developer focus level, codebase complexity gradient, time-of-day fatigue factor)
2. Patel feeds the state estimate into the cognitive model, which adjusts generation parameters (verbosity, refactoring aggressiveness, test coverage expectations)
3. Kristiansen invokes the generation provider to produce INDEX.patch + COMMIT.msg
4. Hayashi annotates the patch with confidence scores and alternative approaches
5. If confidence is below 0.7, the patch is flagged for human review instead of auto-applied

The COMMIT.msg includes a structured footer with cognitive metadata:

```
Cognitive-Load-Estimate: moderate
Confidence: 0.82
Alternatives-Considered: 2
Fatigue-Risk: low
```

This metadata is machine-readable and feeds back into the cognitive model for future tasks.

## Requirement 4: Polyrepo PR Coordination

Cross-repo coordination uses PR comments as a structured advisory channel. Each comment carries a cognitive annotation:

```
[PBRL:advisory] confidence=0.85 load=moderate
Recommendation: merge backend PR before frontend PR.
Basis: dependency analysis shows frontend changes reference backend types
introduced in PR #47. Merging out of order will produce type errors that
correlate with high-cognitive-load debugging sessions in historical data.
```

The advisory approach means PBRL agents never *block* PRs — they *recommend* merge ordering. The human operator makes the final decision. This is consistent with the lab's philosophy: agents inform, they do not command.

Forge abstraction follows the same trait pattern as provider abstraction. Each forge implements `ForgeAdvisor`: `read_pr`, `post_comment`, `list_dependencies`. Three methods. No more.

## Requirement 5: Agent Memory in Git Branches

Memory is stored in `refs/pbrl/memory/<agent-name>` branches. Memory entries are typed:

- **`observation`**: Raw behavioral data (commit patterns, timing, complexity metrics). TTL: 7 days.
- **`model-state`**: Serialized cognitive model parameters. TTL: 30 days.
- **`advisory-outcome`**: Whether a previous advisory was accepted or rejected by the developer. TTL: 90 days. This is the most valuable memory type — it enables the model to learn which recommendations are useful.

Felix enforces a strict data lifecycle: all memory entries must have a TTL. No permanent storage without ethics board approval. Memory entries containing behavioral patterns are anonymized at write time — the agent stores the pattern, not the identity of the developer who exhibited it.

Memory retrieval uses embedding-based similarity search within the observation space. Unlike key-value lookup, this allows agents to find relevant memories even when the exact key is unknown. The tradeoff (false positives) is managed by Hayashi's confidence thresholds.

## Requirement 6: Signed Commits via OpenWallet

Each agent holds an OpenWallet DID with a signing key. Signing is performed by Kristiansen's execution layer — the advisory layer never signs anything. Key rotation follows the university's IT security policy: 90-day rotation, immediate revocation on compromise.

The lab adds one additional signing property: every signed commit includes a `Model-Version` header identifying which version of the cognitive model produced the recommendation. This enables retrospective analysis: if a model version is later found to have a systematic bias, all commits influenced by that model can be identified and reviewed.

**Unique insight:** PBRL treats agent memory not as a cache but as a longitudinal dataset. Over time, the advisory-outcome entries create a feedback loop: the cognitive model learns which types of recommendations developers accept, and adjusts its confidence thresholds accordingly. This means the system gets *more accurate for a specific developer* the longer it runs — a property no stateless agent system can achieve. The memory branch becomes, in effect, a personalized cognitive model of the development team.

---

## Token Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Voss | 2,000 | 600 | 2,600 |
| Patel | 4,500 | 3,000 | 7,500 |
| Kristiansen | 3,500 | 2,500 | 6,000 |
| Hayashi | 3,000 | 1,500 | 4,500 |
| Okoro | 1,500 | 500 | 2,000 |
| **Task Total** | **14,500** | **8,100** | **22,600** |

Advisory overhead (cognitive model inference, annotation generation): 3,000 tokens. Grand total per task: **25,600 tokens**.

---

*"The commit history is a cognitive trace. Read it like a scientist, not an accountant."*
— Dr. Lena Voss, ICSE 2026 keynote
