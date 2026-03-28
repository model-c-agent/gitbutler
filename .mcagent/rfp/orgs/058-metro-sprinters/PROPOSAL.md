# Metro Sprinters — Technical Proposal for `but-ai`

**RFP Response | Tier 2 Abbreviated**

---

## Executive Summary

Metro Sprinters proposes a `but-ai` implementation centered on performance measurement and competitive feedback loops. Our domain expertise in turning schedule adherence into a competitive sport translates to agents that are continuously measured, ranked, and improved through visible performance dashboards.

## Requirement 1: PATH-Based Plugin Architecture

Rust binary at `$PATH` as `but-tool-ai`. The binary includes a `--dashboard` flag that outputs current agent performance metrics in JSON format, enabling external dashboard tools to display league tables. Every invocation logs performance data to `$XDG_STATE_HOME/but-ai/league.jsonl`.

Configuration via `$XDG_CONFIG_HOME/but-ai/config.toml` with per-repo overrides. Performance thresholds (minimum adherence score, maximum response time) are configurable.

## Requirement 2: Provider-Agnostic LLM Interface

Four-provider backend. Frequency manages provider selection using a "league table" approach: providers are ranked by a composite score of latency, cost, and output quality. Rankings are updated after every call. The top-ranked provider gets the next call unless the task has constraints that require a specific capability.

Provider "suspension": if a provider's composite score drops below a threshold for 10 consecutive calls, Frequency suspends it from the rotation for 24 hours, then reinstates it on probation (3 calls at reduced priority before full reinstatement).

**Domain Insight:** In transit operations, a bus that runs consistently 2 minutes late is more useful than a bus that is sometimes on time and sometimes 15 minutes late. Consistency is more valuable than occasional excellence. Frequency applies this principle: it prefers providers with low variance in output quality over providers with higher average quality but unpredictable performance.

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Headway generates patches with adherence-aware pacing:
1. **Estimate** — Predict expected completion time and quality level based on task complexity.
2. **Execute** — Generate INDEX.patch within the estimated time window.
3. **Score** — Compare actual output against estimate. Log the adherence gap.
4. **Adjust** — If adherence was poor (actual deviated significantly from estimate), log the cause for future calibration.

COMMIT.msg includes adherence data:
```
feat(headway): add real-time arrival prediction endpoint

Adherence: 0.93 | Est: 45s, Actual: 42s | League: 2/5
Agent: Headway | Provider: anthropic (rank: 1)
Memory: dwell/delay-2026-0119
```

## Requirement 4: Polyrepo PR Coordination

Interchange manages cross-repo coordination using a "connection" model. Each cross-repo dependency is a connection that must be made for the journey (merge set) to succeed. Interchange maintains a connection board in `refs/sprinters/connections/` tracking each connection's status.

When a connection is at risk (a dependent PR has failing checks or unresolved comments), Interchange escalates with a warning that includes the estimated delay. Forge adapters (GitHub, GitLab, Gitea) implement a minimal trait. PR comments include connection status and estimated coordination time.

## Requirement 5: Agent Memory in Git Branches

Dwell manages memory using a "delay record" model. Each memory entry records:

| Field | Description |
|-------|-------------|
| `id` | Unique identifier |
| `expected` | What was planned (task description, estimated complexity) |
| `actual` | What happened (actual complexity, outcome, time) |
| `delay` | Gap between expected and actual |
| `cause` | Identified cause of deviation |
| `ttl_days` | 21 default, +7 per reuse (max 60) |

Memory stored in `refs/sprinters/history/`. The "delay" field is the key innovation: retrieving memories where similar tasks had large delays warns the current agent to allocate more resources. Memories where expected matched actual confirm the estimation model is calibrated.

GC removes expired entries. High-delay memories are retained 2x longer because they contain the most valuable information.

## Requirement 6: Signed Commits via OpenWallet

Whistle handles signing with a quality gate:
- Pre-sign quality check against a configurable checklist.
- Sign only if the quality check passes.
- Log quality score alongside signature metadata.
- Quality scores feed into the agent's league position.

Key rotation every 30 days. Whistle is the referee: fair, consistent, and not swayed by urgency.

## Token Budget

| Agent | Role | Input/task | Output/task | Total |
|-------|------|-----------|-------------|-------|
| Headway | Patch generation | 7,500 | 4,000 | 11,500 |
| Dwell | Memory & delay records | 4,500 | 800 | 5,300 |
| Frequency | Provider league table | 3,000 | 700 | 3,700 |
| Interchange | PR coordination | 4,500 | 1,800 | 6,300 |
| Whistle | Signing & quality gate | 3,000 | 700 | 3,700 |
| **Per-task total** | | **22,500** | **8,000** | **30,500** |

## Unique Domain Insight

Three years of measuring transit schedule adherence taught us that the most powerful metric is not absolute performance — it is the gap between expected and actual. A bus that is consistently 2 minutes late has a small gap (it is reliably late, which riders can plan around). A bus with high average on-time performance but large variance has a big gap (sometimes on time, sometimes 15 minutes late, which riders cannot plan around).

Our proposal applies this to agent performance: we track the delay record (gap between estimated and actual) for every task. Over time, this builds a calibration model that makes future estimates more accurate. Accurate estimates mean better resource allocation, which means better budget utilization, which means more value per token. The delay record is the flywheel that makes everything else work.

---

*Leaderboard updated. Season: Spring 2026. Top of the table: Headway.*
