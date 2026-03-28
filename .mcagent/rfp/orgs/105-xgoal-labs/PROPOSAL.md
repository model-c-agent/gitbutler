# xGoal Labs -- Technical Proposal

**RFP Response: `but-ai` Plugin for GitButler**

---

## Executive Summary

We build ML models that football clubs bet their seasons on. Our `but-ai` plugin applies the same rigor to agent development: every patch is versioned, every model change is tracked, every decision is explainable. If you cannot explain it to a head coach, it does not ship.

---

## Requirement 1: PATH-Based Plugin Architecture

Static binary on `$PATH`. The binary includes `but-ai model-diff` -- a subcommand that compares two model versions and outputs a human-readable summary of what changed and why the metrics shifted. This is our most-requested internal tool, now exposed as a plugin command.

**Startup target:** Under 150ms. Model-diff under 3 seconds for models up to 50MB.

---

## Requirement 2: Provider-Agnostic AI

`Completer` trait, four providers. Provider selection optimized for ML development workflows:
- Code generation tasks: Use the most capable available model (Claude Sonnet / GPT-4o)
- Documentation and commit messages: Use the cheapest model (Haiku / GPT-4o-mini)
- Feature engineering exploration: Use local models (Ollama) to avoid leaking proprietary features

**Feature confidentiality:** xGoal Labs' 47-feature xG model is IP. Provider calls that include feature definitions are restricted to local models or providers with enterprise data agreements.

---

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Agents produce patches. Patches that modify the model include additional artifacts:

**Model-change patches:**
- INDEX.patch (code changes)
- COMMIT.msg with `Model-Impact:` trailer summarizing metric changes
- EVAL.json (before/after evaluation metrics on the holdout set)

```
Model-Impact: xG log-loss improved from 0.2841 to 0.2793 (-1.7%).
    Top feature importance shift: defensive_pressure_index moved from
    rank #8 to rank #5. Interpretability: HIGH (linear coefficient).
```

Anika reviews the Model-Impact trailer before approving. Metric improvement without interpretability is not sufficient.

---

## Requirement 4: Polyrepo PR Coordination (Forge-Agnostic)

Four repos: vision-pipeline, xg-model, api, dashboard. Forge adapter coordinates in dependency order.

**Release train:** Model releases are the locomotive. When the xG model repo merges a new version, it triggers coordinated PRs in the API repo (schema update) and dashboard repo (visualization update). The vision pipeline is independent but pinned to a specific API version.

**Schema compatibility check:** Before any cross-repo merge, the coordination system validates that the model's output schema matches the API's expected input schema. Schema mismatches halt the train.

---

## Requirement 5: Agent Memory in Git Branches

Memory in `refs/xgoal/models/<version>/`. Model-centric memory: each entry is associated with a model version and captures what was tried, what worked, and what failed.

**Memory schema:**
```json
{
  "key": "defensive-pressure-feature-v3",
  "value": "Added Voronoi-based defensive pressure index. Improved log-loss by 1.7%. Interpretable as linear coefficient.",
  "model_version": "v4.2.0",
  "metric_delta": {"log_loss": -0.0048, "brier": -0.003},
  "experiment_status": "merged",
  "created": "2026-03-28T10:00:00Z",
  "ttl": null
}
```

**Failed experiment memory:** Experiments that were tried and failed are stored with `experiment_status: "abandoned"` and the reason for failure. This prevents agents from re-running failed experiments. Jun estimates this saves 15% of model development time.

---

## Requirement 6: Signed Commits via OpenWallet

Development commits signed with developer keys. Release commits signed with a release key managed by Fatou. The release key is the company's attestation that a model version has passed validation.

**Client verification:** Licensed clubs can verify the release signature to confirm they are running an officially validated model version, not a modified fork.

---

## Token Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Diego | 9,200 | 4,500 | 13,700 |
| Anika | 3,800 | 900 | 4,700 |
| Priya | 6,500 | 2,800 | 9,300 |
| Jun | 5,800 | 800 | 6,600 |
| Fatou | 3,400 | 900 | 4,300 |
| Sam | 3,000 | 600 | 3,600 |
| **Total** | **31,700** | **10,500** | **42,200** |

---

## Unique Insight: Failed Experiment Memory Prevents Redundant Exploration

ML development is iterative. Teams try features, hyperparameters, and architectures. Many fail. Without institutional memory, teams re-try failed approaches months later, wasting time rediscovering why they did not work.

Our memory system stores failed experiments alongside successful ones. When an agent proposes a feature engineering approach, the memory system checks for similar prior experiments and surfaces them: "A similar approach was tried in v3.8.0 and abandoned because [reason]."

In six months of use, failed-experiment memory prevented 23 redundant experiments. At an average of 4 GPU-hours per experiment, that is 92 GPU-hours saved. More importantly, it prevented 23 instances of an engineer spending a day building something that would not work -- a saving measured in morale, not just compute.

Jun's system does not just remember what worked. It remembers what did not, and why. That second kind of memory is worth more.

---

*"Every failed experiment is a memory. Every memory saves the next attempt."*
