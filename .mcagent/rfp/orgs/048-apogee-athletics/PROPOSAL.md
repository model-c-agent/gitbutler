# Apogee Athletics — Technical Proposal for `but-ai`

**RFP Response | Tier 2 Abbreviated**

---

## Executive Summary

Apogee Athletics proposes a `but-ai` implementation built around performance measurement and competitive optimization. Our domain expertise in gamifying safety-critical behavior — making satellite operators compete on collision avoidance — translates to agents that are measured, ranked, and improved through continuous performance feedback.

## Requirement 1: PATH-Based Plugin Architecture

Rust binary at `$PATH` as `but-tool-ai`. Stateless invocation: JSON in, JSON out. The binary embeds a lightweight metrics collector that logs every invocation to `$XDG_STATE_HOME/but-ai/metrics.jsonl` — timestamp, agent, task type, tokens consumed, time elapsed, outcome (success/partial/failure). This log powers the APR (Agent Performance Rating) system.

Configuration via `$XDG_CONFIG_HOME/but-ai/config.toml` with per-repo overrides. Metrics collection is always on; there is no opt-out. You cannot improve what you do not measure.

## Requirement 2: Provider-Agnostic LLM Interface

Four-provider backend (OpenAI, Anthropic, Ollama, LMStudio). Wing agent maintains a live "provider leaderboard" ranking each backend by latency, cost-per-token, and tool-call accuracy. Provider selection is dynamic: Wing routes each request to the current leader in the category most relevant to the request type (latency-sensitive requests go to the fastest provider; cost-sensitive requests go to the cheapest).

Provider benchmarks are updated rolling — each call updates the provider's stats using an exponential moving average. New providers can be added by implementing the `Provider` trait and registering in config; they start with neutral ratings and earn their rank through performance.

**Domain Insight:** In sports analytics, the best predictor of future performance is recent form, not career average. Wing uses the same principle: a provider's last 20 calls matter more than its last 2,000. This catches performance regressions (API degradation, model updates) faster than static benchmarks.

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Pivot generates patches through a "play execution" cycle:
1. **Read the pitch** — GetProjectStatus, GetBranchChanges. Understand the game state.
2. **Call the play** — Select patch strategy based on task type and Keeper's memory.
3. **Execute** — Generate INDEX.patch targeting maximum acceptance probability.
4. **Score** — Self-evaluate the patch against historical acceptance patterns before submission.

COMMIT.msg includes the APR score and a brief performance note:

```
feat(scoring): add real-time conjunction alert API

APR: 0.84 | Tokens: 2,100/1,400 | Strategy: standard-feature
Agent: Pivot
Memory-Refs: keeper/replay-2026-0188
```

Pivot's patches are optimized for review speed: conventional structure, minimal diff size, clear naming. A patch that takes a reviewer 30 seconds to understand scores higher than a clever patch that takes 5 minutes.

## Requirement 4: Polyrepo PR Coordination

Sweeper manages cross-repo coordination using a "defensive zone" model. Each repo is a defensive zone. Sweeper maintains a zone map in `refs/apogee/zones/` that tracks:
- Active PRs per zone (repo).
- Cross-zone dependencies (PRs that must merge together).
- Threat assessment (probability of merge conflict based on file overlap analysis).

When threat assessment exceeds a threshold, Sweeper preemptively opens a coordination PR that documents the dependency and blocks premature merging. Forge adapters (GitHub, GitLab, Gitea) implement a shared `ForgeAdapter` trait. PR comments are structured with metrics embedded.

## Requirement 5: Agent Memory in Git Branches

Keeper manages memory as a "match replay" archive. Each memory entry is a recorded play:

| Field | Description |
|-------|-------------|
| `replay_id` | Unique match identifier |
| `recorded_at` | Timestamp |
| `play_type` | patch, review, coordination, signing |
| `outcome` | accepted, rejected, partial |
| `score` | Replay value (0-1) |
| `content` | The memory payload |
| `ttl_days` | Based on score: high-value replays live longer |

Memory stored in `refs/apogee/replay/<agent-id>/`. Relevance scoring combines semantic similarity with replay value — a highly-scored memory from a related task ranks above a low-scored memory from an identical task. This prevents agents from repeating unsuccessful strategies.

GC runs automatically when the replay archive exceeds a configurable size threshold. Low-value replays are evicted first (relegation).

## Requirement 6: Signed Commits via OpenWallet

Captain manages signing. Only Captain signs; other agents cannot. This concentrates signing authority and creates a single audit point. Captain verifies:
- The patch was produced by an authorized agent.
- The agent's current APR is above the minimum threshold (configurable, default 0.5).
- The patch has been reviewed (by a human or by a designated review agent if configured).

Key rotation every 30 days. Signing metadata includes the APR of the agent that produced the patch, giving reviewers a quick signal of agent reliability.

## Token Budget

| Agent | Role | Input/task | Output/task | Total |
|-------|------|-----------|-------------|-------|
| Pivot | Patch generation | 8,000 | 5,000 | 13,000 |
| Keeper | Memory & replay | 4,500 | 800 | 5,300 |
| Wing | Provider routing | 3,000 | 700 | 3,700 |
| Sweeper | PR coordination | 4,000 | 1,500 | 5,500 |
| Captain | Signing & leadership | 3,000 | 800 | 3,800 |
| **Per-task total** | | **22,500** | **8,800** | **31,300** |

## Unique Domain Insight

Two years of gamifying satellite operator behavior taught us that measurement changes behavior even without enforcement. Operators who could see their collision avoidance score improved by 31% — not because we penalized poor performance, but because visibility created accountability.

Our proposal embeds this principle at the agent level. Every agent action is measured and the metrics are visible to the human operator. We do not hide agent performance behind aggregate success rates. If an agent is producing patches that get rejected 40% of the time, the human should see that number and decide whether to reconfigure, retrain, or replace the agent. Transparency is the mechanism. Improvement is the outcome.

---

*Final whistle. APR updated. Season continues.*
