# PROPOSAL.md — The Dramaturgy & Cognition Lab

*"The codebase has a predictive model. Every patch is a prediction error."*

---

## Summary

The Dramaturgy & Cognition Lab proposes to build the `but-ai` plugin through the lens of predictive processing. A codebase is a structure that creates expectations in its reader. A patch is a violation of those expectations — ideally a productive one. The lab's memory architecture uses prediction error signals to surface not just relevant memories, but memories that challenge the agent's current model of the codebase.

---

## Technical Proposal

### Requirement 1: PATH-Based Plugin Architecture

The `but-ai` binary is installed to PATH. The manifest file follows an experimental protocol format: it declares the plugin's capabilities as hypotheses ("this plugin can generate patches," "this plugin can coordinate across repos") that are tested at invocation time via capability probing. If a declared capability fails its probe, it is marked unavailable rather than crashing. The lab treats software configuration the same way it treats experimental design: declare your assumptions, test them, report the results.

### Requirement 2: Provider-Agnostic AI

The lab's research uses multiple measurement instruments (EEG, fMRI, GSR, eye tracking), each with different characteristics but a shared analysis pipeline. The provider abstraction follows the same pattern: each LLM provider is an instrument with different characteristics, normalized through a shared `Instrument` trait: `measure(prompt) -> Completion`, `analyze(completion) -> ToolCalls`, `calibrate(usage) -> TokenReport`.

The lab adds a calibration step absent from most proposals: before using a provider for a task, the system runs a brief calibration prompt to verify tool-calling fidelity. Calibration results are stored in memory and used to adjust expectations for the provider's behavior. This is standard practice in neuroscience — you calibrate the instrument before every session.

### Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

The agent workflow follows the lab's experimental cycle:

1. **Hypothesis** — Scholz maps the task as a dramaturgical structure with expected outcomes
2. **Data collection** — Bergstrom retrieves relevant memory, Tanaka reads the codebase
3. **Methodology review** — Oliveira verifies the approach is sound
4. **Experiment** — Tanaka produces INDEX.patch + COMMIT.msg
5. **Analysis** — Oliveira reviews the patch against the hypothesis
6. **Publication** — Scholz approves and signs

The COMMIT.msg follows academic abstract format: Background (why the change is needed), Methods (what was changed), Results (what the change produces), Conclusion (one-line summary). This is verbose by engineering standards. The lab considers it the minimum viable documentation.

### Requirement 4: Polyrepo PR Coordination

Cross-repo coordination is modeled as multi-site experiments. Each repo is a study site. PR comments are study coordination messages — structured, timestamped, and reproducible. The comment schema includes a "protocol version" field to ensure all repos are coordinating using the same schema version.

Forge-agnostic: the lab implements a `StudySite` trait: `distribute_protocol(message)`, `collect_results(site) -> Vec<Result>`, `synchronize(sites)`. GitHub, GitLab, and Gitea implement `StudySite`.

### Requirement 5: Agent Memory in Git Branches

Memory is stored in `refs/dcl/data/` as Git blobs. Each entry follows experimental data standards:

```json
{
  "entry_id": "DCL-2026-0847",
  "timestamp": "2026-03-28T14:30:00Z",
  "source": "tanaka",
  "observation": "Auth middleware validates JWT on every request, not just initial",
  "confidence": 0.95,
  "prediction_error": 0.72,
  "replicated_by": ["DCL-2026-0823"],
  "contradicts": [],
  "ttl_days": 45
}
```

**The prediction error memory model:** Every memory entry has a `prediction_error` score — a measure of how surprising this observation was relative to the agent's existing model of the codebase. High prediction error memories are surfaced preferentially during retrieval, because they represent the most informative observations. This is the lab's core insight applied to agent memory: the most useful thing to remember is the thing that surprised you.

A memory with high confidence and high prediction error is a confirmed surprise — a fact about the codebase that defies the expected pattern. These are the memories that prevent agents from making assumptions based on outdated or incomplete models.

### Requirement 6: Signed Commits via OpenWallet

Dr. Scholz signs all commits as lab PI. The signing key is bound to her DID via OpenWallet. The lab's signing protocol includes a methodology review step (Oliveira verifies the patch's validity before Scholz signs) that mirrors the lab's publication process — no paper is submitted without internal peer review, and no commit is signed without internal code review.

Key rotation: annually, aligned with the academic year. Emergency rotation: immediate upon suspicion of compromise, with all commits since the last known-good state flagged for re-review.

---

## Token Budget

| Agent | Input | Output | Total | Role |
|-------|-------|--------|-------|------|
| Dr. Scholz | 6,500 | 2,500 | 9,000 | Direction, signing |
| Dr. Oliveira | 6,000 | 2,500 | 8,500 | Methodology review |
| Tanaka | 9,000 | 6,500 | 15,500 | Patch generation |
| Bergstrom | 5,500 | 1,500 | 7,000 | Memory architecture |
| **Team Total** | **27,000** | **13,000** | **40,000** | |

Calibration and review overhead: ~3,500 tokens.
**Total per task: ~43,500 tokens.**

---

## Unique Insight

**Memory retrieval should prioritize prediction errors, not confirmations.** Most memory systems rank by relevance: "find me memories similar to the current task." The DCL system ranks by prediction error: "find me memories that surprised me when I first encountered them." A high-prediction-error memory is information that violated the agent's model of the codebase — an unexpected pattern, a counterintuitive design choice, a function that does something different from what its name suggests. These are precisely the memories that prevent an agent from generating a confident-but-wrong patch based on assumptions. Confirmation is comfortable. Surprise is informative.

---

*"Where does the play happen? Between the stage and the brain. Where does the code happen? Between the patch and the reader's expectations."*
