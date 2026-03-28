# Combines FC -- Technical Proposal

**RFP:** `but-ai` Plugin for GitButler
**Race Pace:** Target <5 min/task
**Date:** 2026-03-28

---

## Race Brief

We build autonomous harvesters that compete on speed and precision. Our proposal applies the same principles to AI agent workflows: measure everything, optimize the bottleneck, and never sacrifice quality for speed (because grain loss is a disqualification).

---

## Requirement 1: PATH-Based Plugin Architecture

Binary on PATH. Warm-start daemon (`but-ai daemon`) keeps provider connections hot and system prompt pre-loaded. Cold start: ~350ms. Warm start: <60ms.

Configuration: TOML with a `[performance]` section that exposes latency tuning knobs:
- `daemon_enabled`: keep connections warm
- `preload_system_prompt`: cache the system prompt in the daemon
- `parallel_context_load`: load file contents concurrently (default: true)
- `speculative_start`: begin generation before context is fully loaded (default: false)

The performance section exists because we measure everything and want to tune everything.

---

## Requirement 2: Provider-Agnostic AI

Four providers. Trait with three methods: `complete`, `complete_with_tools`, `token_count`. We omit `init` as a separate method — initialization happens in the constructor.

Provider benchmarking: on first configuration and weekly thereafter, the plugin runs a standard benchmark (fixed prompt, measure latency and output quality). Results stored in memory. Header uses benchmark data to set realistic time targets per provider.

Fallback: configurable. When enabled, the plugin falls back to the next provider if the primary exceeds a latency threshold (default: 8s per call). We support fallback because race conditions do not wait for provider recovery.

---

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Rotor generates patches at race pace. The workflow is optimized for wall-clock time:

1. Context loading and memory retrieval happen concurrently
2. Rotor begins generation as soon as minimum context is available
3. INDEX.patch and COMMIT.msg are produced in a single generation pass (not sequentially)
4. Validation (`git apply --check`) runs immediately
5. Sieve's review begins as soon as the patch is valid

Commit messages include performance telemetry:

```
Adjust crop density threshold for section 4

Time-to-patch: 2.4min
Tokens: 8200/17000
Review: 1 round
```

---

## Requirement 4: Polyrepo PR Coordination

The team uses two repos: `harvest-algo` (core algorithms) and `field-config` (per-field parameters). Coordination via PR comments:

```
[cfc:sync] algo=harvest-algo@adaptive-speed config=field-config@salina-2026 status=ready
```

Dependencies are simple: algorithm changes land first, then config updates. No complex DAG — the team has two repos and one direction of dependency.

Forge: GitHub. Minimal trait.

---

## Requirement 5: Agent Memory in Git Branches

Memory in `refs/but-ai/mem/<season>/<key>`. Season-bounded because competitive parameters change every year (new rules, new field conditions, new opponents' strategies).

### Telemetry Memory

The team's unique memory type is telemetry: structured performance data from every task.

| Field | Type | Example |
|-------|------|---------|
| `time_to_patch` | duration | 2.4 min |
| `tokens_used` | integer | 8,200 |
| `review_rounds` | integer | 1 |
| `provider_latency` | duration | 1.2s avg |
| `patch_size` | lines | 47 |

Telemetry is aggregated weekly into summary statistics. The summaries feed into Header's estimation model: given a task of type X, what is the expected time, token cost, and review round count?

This creates a feedback loop: the team gets faster because the estimation model gets better, and the estimation model gets better because the team generates more data.

TTL: raw telemetry 48 hours, weekly summaries per season, lifetime summaries permanent.

---

## Requirement 6: Signed Commits via OpenWallet

Team DID signing. All commits signed by Hopper with the team's shared credential. Key rotation: 7 days (aligned with weekly practice sessions).

Signing is pre-cached: Hopper requests a signing credential at daemon startup and reuses it for all commits in the session. This eliminates per-commit signing latency (~200ms saved per commit on our benchmark).

---

## Token Budget

| Agent | Input | Output | Total | Role |
|-------|-------|--------|-------|------|
| Header | 4,500 | 2,000 | 6,500 | Decomposition |
| Rotor | 10,000 | 7,000 | 17,000 | Patch generation |
| Sieve | 5,500 | 2,000 | 7,500 | Review |
| Hopper | 3,500 | 1,000 | 4,500 | Memory/budget/signing |
| **Team** | **23,500** | **12,000** | **35,500** | |

---

## Unique Insight: Telemetry-Driven Estimation

Every AI agent system estimates costs before executing tasks. Most estimates are static: "a 200-line patch costs approximately 40,000 tokens." These estimates are wrong because they ignore variance across task types, codebases, and providers.

Our competitive harvesting experience taught us that estimation improves with data. The first time we raced a field, our time estimates were off by 30%. After 20 races, they were off by 5%. The difference was telemetry: we measured actual performance, compared it to estimates, and adjusted the model.

We apply this to `but-ai`. Every task produces telemetry: actual tokens consumed, actual time elapsed, actual review rounds. The telemetry feeds into an estimation model that improves with each task. After 50 tasks, the model predicts token consumption within 10% for that codebase and provider combination.

This is not machine learning. It is actuarial science applied to token budgets: collect data, compute statistics, update estimates. Simple, effective, and proven across 3,000 years of insurance mathematics and three years of autonomous harvesting.

---

*Race pace. Clean grain. Time logged. See you at the next NAAHL.*
