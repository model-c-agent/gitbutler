# Code Blue Athletics — Technical Proposal

**RFP Response: `but-ai` Plugin for GitButler**

---

## Executive Summary

We propose a `but-ai` implementation optimized for **speed and competitive evaluation**. Agents are benchmarked against each other on standardized tasks, and the best-performing configuration is promoted to production. Our system includes a built-in benchmarking framework that runs agent configurations through a standardized "season" of tasks and publishes performance leaderboards.

---

## Requirement 1: PATH-based Plugin Architecture

PATH-based binary with benchmarking built in.

**Design:**
- Binary: `but-ai`, optimized for startup latency (<100ms cold start)
- Commands: `but ai patch`, `but ai bench` (run benchmark suite), `but ai leaderboard`, `but ai memory`
- Config: `~/.config/but-ai/cba.toml`
- `but ai bench run` executes the agent against a standardized task suite and reports scores
- `but ai leaderboard` displays historical performance across configurations and providers
- `but ai bench compare <config-a> <config-b>` runs head-to-head comparison

---

## Requirement 2: Provider-Agnostic AI

Providers compete on a **latency-quality leaderboard**.

**Architecture:**
- Provider trait: standard invoke/stream with latency reporting
- Every provider call records: latency_ms, tokens_in, tokens_out, quality_score (post-hoc)
- Provider leaderboard stored in `refs/cba/leaderboard/providers`
- Circuit breaker: provider removed from rotation after 3 consecutive failures, reinstated after cooldown
- Speculative execution (optional): send to 2 providers, use first quality response

**Supported:** OpenAI, Anthropic, Ollama, LMStudio

---

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Agents produce patches scored on a **multi-factor rubric**.

**Scoring rubric:**
- **Correctness** (40%): Does the patch apply cleanly? Do tests pass?
- **Completeness** (25%): Does the patch address the full task scope?
- **Efficiency** (20%): Tokens consumed relative to patch complexity
- **Style** (15%): Does the patch match codebase conventions?

**Agent workflow:**
1. Read task and context
2. Generate INDEX.patch + COMMIT.msg
3. Self-score against rubric (agent evaluates its own output)
4. If self-score > threshold: commit
5. If self-score < threshold: retry (max 2 retries)

**Benchmark integration:** Every patch is automatically scored and the result appended to the agent's historical performance record.

---

## Requirement 4: Polyrepo PR Coordination

Cross-repo coordination with **time-bounded operations**.

**Protocol:**
- PR comments: `<!-- cba:coord:{action}:{payload} -->`
- Actions: `propose`, `ack`, `ready`, `timeout`, `merge`
- Every coordination phase has a timeout (configurable, default 30 minutes)
- If a dependent repo does not respond within the timeout, the system proceeds with available repos and flags the unresponsive one

**Forge adapters:** GitHub, GitLab, Gitea, Forgejo. Standard trait.

**Clock management:** All timeouts are tracked. Cross-repo coordination produces a timing report showing how long each repo took to respond — useful for identifying bottlenecks.

---

## Requirement 5: Agent Memory in Git Branches

Memory as a **replay buffer** with pattern extraction.

**Storage:** `refs/but-ai/memory/<season>/<key>`

Seasons are time-bounded collections (e.g., one month). At season end, the best-performing memory entries are promoted to the "hall of fame" (`refs/but-ai/memory/hof/<key>`) and the rest expire.

**Replay system:** Every agent run is recorded as a replay:
```toml
[replay]
id = "run-2026-03-28-042"
task = "Add error handling"
score = 87.5
tokens = 31200
provider = "anthropic/claude-sonnet"
memories_used = ["error-pattern", "handler-convention"]
```

Replays are analyzed for patterns: which memories correlate with high scores? Which providers produce better results for which task types?

---

## Requirement 6: Signed Commits via OpenWallet

Standard signing with **integrity guarantees for competitive data**.

**Design:**
- All agent commits signed via OpenWallet
- Benchmark scores are included in the commit's VC (preventing post-hoc score manipulation)
- Key rotation: 30 days
- Anti-tampering: any modification to a scored commit triggers an alert

---

## Token Budget

### Per-Task Budget (standard 200-line, 3-file feature)

| Agent | Role | Input | Output | Total |
|-------|------|-------|--------|-------|
| Voss | Evaluation | 2,800 | 800 | 3,600 |
| Sadeqi | Patch | 9,500 | 4,500 | 14,000 |
| Watanabe | Provider | 5,800 | 2,000 | 7,800 |
| Strand | Memory | 5,500 | 700 | 6,200 |
| Restrepo | Signing | 3,200 | 600 | 3,800 |
| **Total** | | **26,800** | **8,600** | **35,400** |

### Competitive Scaling

| Round | Description | Budget |
|-------|-------------|--------|
| Warmup (trivial) | Single file, <30 lines | 14,160 (0.4x) |
| Regular season | Standard feature | 35,400 (1.0x) |
| Playoffs | Multi-repo | 70,800 (2.0x) |
| Championship | Architecture change | 88,500 (2.5x) |

---

## Unique Insight: Competitive Benchmarking as Continuous Improvement

Most agent systems are configured once and evaluated informally. Our system treats agent configuration as an ongoing competition. Every configuration change produces a measurable score delta. Every provider switch is a roster move that must justify itself statistically.

We publish leaderboards — not for vanity, but for selection pressure. When you can see that configuration A scores 12% higher than configuration B on error-handling tasks, you switch. When you can see that provider X produces 20% faster responses with equivalent quality, you route accordingly.

Competition is not the opposite of collaboration. Competition is the mechanism by which the best approach is identified. We run the tournament. The data picks the winner.

---

*"Check the scoreboard."*
