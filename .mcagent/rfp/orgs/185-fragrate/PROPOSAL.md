# FragRate — Technical Proposal

**RFP:** `but-ai` Plugin for GitButler
**Stage:** Seed-stage startup, production system

---

## Executive Summary

FragRate proposes a lean, budget-conscious agent pipeline optimized for correctness-per-token. Four agents, no overhead, every token justified by measurable impact on rating accuracy. The system is designed for auditability: every rating adjustment is a signed commit with a full provenance trail.

---

## Requirement 1: PATH-Based Plugin Architecture

Single binary, `but-ai`, installed to `~/.gitbutler/bin/`. Written in Rust (Mo insists). The binary is small — under 10MB — and statically linked. No runtime dependencies. Deploys via `cargo binstall` or direct download from our GitHub releases.

Subcommands mirror the rating pipeline: `but ai ingest` (import match data), `but ai rate` (compute rating adjustments), `but ai review` (display pending adjustments for approval), `but ai apply` (commit approved adjustments). Each subcommand is a discrete pipeline stage. Stages can run independently or be chained: `but ai ingest && but ai rate && but ai review`.

The plugin reads configuration from `~/.gitbutler/ai.toml` and `.gitbutler/ai.toml` (repo-level overrides). Repo-level config specifies game title, variance baselines, and anomaly thresholds. This per-repo config is critical — FragRate's multi-title support means each game repo has different rating parameters.

## Requirement 2: Provider-Agnostic AI

Dana designed the provider layer around cost. The interface: `complete(messages, budget) -> (response, tokens_used)`. The `budget` parameter is not advisory — it is a hard cap. The provider adapter truncates the request if the estimated token cost exceeds budget. Better to get a truncated response than an overbudget one.

Provider routing: OpenAI for anomaly detection (best tool-calling accuracy), Anthropic for rating model reasoning (best at structured mathematical analysis), Ollama for low-stakes tasks (documentation, memory maintenance). Provider selection is configured per pipeline stage, not per agent.

Fallback: none. If the configured provider fails, the stage fails. Dana's reasoning: silent fallback to a cheaper provider could produce subtly wrong ratings, which are worse than no ratings. We would rather explain downtime than explain incorrect ratings.

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

The rating adjustment pipeline:

1. Kai defines the rating model parameters for the current period
2. Dana allocates token budgets per stage
3. Mo's agents ingest match data and produce rating deltas as INDEX.patch files modifying the rating database (stored as JSON in Git)
4. Each patch includes a COMMIT.msg with structured metadata:

```
Rate: update player ratings for Valorant ranked period 2026-W13

Players-Affected: 12,847
Mean-Delta: +3.2
Anomalies-Flagged: 7
Model-Version: glicko2-fragrate-v4.1
Confidence: 0.94
```

5. Kai reviews flagged anomalies. Approved patches are signed and committed.

The JSON-in-Git approach is unconventional but deliberate. Rating histories must be auditable, diffable, and revertable. SQL databases provide none of these. A rating dispute is resolved by `but log --follow ratings/player/<id>.json` — the complete history of every adjustment, who computed it, and why.

## Requirement 4: Polyrepo PR Coordination

Three repos: `fragrate-ingest` (data pipelines), `fragrate-engine` (rating computation), `fragrate-api` (public API). Cross-repo coordination is essential because a rating model change in the engine repo requires corresponding schema changes in the API repo and ingestion format changes in the ingest repo.

PR comments follow a dependency protocol:

```
[FR:depend] fragrate-engine#142 blocks fragrate-api#89
Rating model v4.1 introduces `confidence` field.
API schema must add `confidence` before engine PR merges.
```

Mo built the forge adapter to support GitHub (current) and GitLab (planned for a self-hosted enterprise client). The adapter is thin — it posts structured comments and reads them back. No webhooks, no event streams. Polling at 60-second intervals. Simple, reliable, debuggable.

## Requirement 5: Agent Memory in Git Branches

Memory branch: `refs/fragrate/memory/shared` (single shared branch — we are four people, not a distributed system). Memory types:

- **`variance-baseline`**: Per-title statistical baselines. TTL: 30 days, refreshed on each rating period. This is the memory that prevents another Smurf Detection Meltdown.
- **`model-params`**: Current rating model parameters. TTL: until next model version.
- **`dispute-outcome`**: Results of rating disputes (upheld, overturned, modified). TTL: 180 days. Feeds back into the anomaly detection model.

Memory retrieval is key-based, not embedding-based. Dana vetoed semantic search — "I need to know exactly what the agent is reading, not approximately." Every memory entry has a deterministic key: `<type>:<game-title>:<period>`. No ambiguity.

## Requirement 6: Signed Commits via OpenWallet

Each agent has an OpenWallet DID. Key rotation every 30 days, managed by Dana's infrastructure automation. The signing flow is simple: Mo's pipeline generates the patch, Kai approves, Mo signs and commits.

For rating adjustments, the signed commit is the legal artifact. Two esports organizations have requested that FragRate provide cryptographic proof that a rating was computed by a specific model version and not manually adjusted. The OpenWallet signature chain provides this — the commit is signed by the computing agent, the approval is signed by Kai, and the provenance is embedded in the commit message.

**Unique insight:** FragRate stores rating databases as JSON files in Git, making the entire rating history a first-class Git artifact. This means `but-ai` agents do not just produce *code* patches — they produce *data* patches. The INDEX.patch format is format-agnostic: it works as well on JSON rating files as on Rust source code. This reveals a broader capability of the `but-ai` architecture: agents that manage structured data, not just source code. The audit trail that Git provides for code can serve equally well for any data that requires versioning, attribution, and non-repudiation.

---

## Token Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Kai | 2,500 | 1,500 | 4,000 |
| Dana | 3,000 | 2,000 | 5,000 |
| Mo | 4,000 | 4,500 | 8,500 |
| Jessie | 2,000 | 1,200 | 3,200 |
| **Task Total** | **11,500** | **9,200** | **20,700** |

Pipeline overhead (data ingestion parsing, anomaly scoring): 4,000 tokens. Grand total per rating period task: **24,700 tokens**.

---

*"Show me the commit that changed my rating, or the rating is a lie."*
— Kai Oduya, FragRate investor pitch, 2025
