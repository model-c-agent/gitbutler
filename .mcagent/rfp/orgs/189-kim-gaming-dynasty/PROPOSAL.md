# Kim Gaming Dynasty — Technical Proposal

**RFP:** `but-ai` Plugin for GitButler
**Approach:** Family-Budget-Constrained Agent System

---

## Executive Summary

The Kim Gaming Dynasty proposes a cost-minimized agent pipeline where every token expenditure is justified by business value. The system uses a tiered provider strategy (local Ollama for routine work, cloud APIs for high-stakes tasks) and a family governance model where Grandmother holds budget veto. The architecture is lean, practical, and optimized for a small team that cannot afford waste.

---

## Requirement 1: PATH-Based Plugin Architecture

Yuna builds the `but-ai` binary in Rust. Installed to `~/.gitbutler/bin/`. The binary is small (target: under 8MB) because Grandmother asked "why does it need to be big?" and Yuna could not justify a larger binary.

Subcommands: `but ai analyze` (read game data, produce coaching insights), `but ai patch` (generate INDEX.patch for coaching reports), `but ai apply` (commit signed patch), `but ai cost` (display token expenditure for the current session — this command was added at Grandmother's request).

The `cost` subcommand is unique to the Dynasty's implementation. It displays tokens consumed, tokens remaining in the period budget, and the estimated monetary cost at the current provider's rate. Sun-hee checks it weekly. Yuna checks it before every provider call.

## Requirement 2: Provider-Agnostic AI

Three-tier provider strategy, designed by Hana's cost analysis:

| Tier | Provider | Use Case | Cost |
|------|----------|----------|------|
| Local | Ollama (Mistral 7B) | Routine analysis, memory operations | $0/month |
| Standard | Anthropic Haiku | Coaching report generation | ~$800/month |
| Premium | Anthropic Sonnet | Complex game state analysis, anomaly detection | ~$400/month |

Total monthly AI budget: $1,200, approved by Sun-hee. The provider interface is a Rust trait: `analyze(context) -> Analysis` and `generate(analysis, task) -> Patch`. Two methods. Yuna argued for three. Grandmother asked what the third one costs. Yuna removed it.

Provider health is checked on first call per session. If the configured provider fails, the system falls back to the next tier *down* (never up — falling back to a more expensive provider would violate the budget). If Ollama is the last tier and it fails, the operation fails.

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

The coaching report pipeline:

1. Min-jun defines coaching standards for the current meta
2. Yuna's analysis agents ingest game data and produce structured insights
3. Insights are formatted as coaching reports — diffs against the player's previous performance baseline
4. Reports are committed as INDEX.patch + COMMIT.msg:

```
Coach: update player 도현 crosshair placement analysis

Map: Ascent
Site: B
Improvement: +4.2% headshot rate vs. previous period
Recommendation: maintain current sensitivity, add jiggle-peek drill
Model: mistral-7b via ollama
Cost: 1,240 tokens ($0.00)

Reviewed-By: Min-jun
```

The `Cost: $0.00` line appears frequently. It makes Grandmother happy.

5. Min-jun reviews recommendations flagged below 0.85 confidence
6. Dohyun's signing pipeline commits approved reports

## Requirement 4: Polyrepo PR Coordination

Two repos: `kim-analytics` (coaching engine) and `kim-platform` (subscriber-facing frontend). Hana manages cross-repo coordination because she manages the business relationship between the analytics product and the subscriber experience.

PR comments are bilingual (Korean and English) because the family works in Korean but their platform documentation is in English:

```
[Kim:sync] kim-analytics#34 → kim-platform#21
분석 엔진에 자신감 점수 추가됨. 프론트엔드 표시 필요.
Confidence score added to analytics engine. Frontend display needed.
```

Forge adapter: GitHub only. Yuna will add GitLab support if a customer requests it and if the development cost is approved by Sun-hee.

## Requirement 5: Agent Memory in Git Branches

Memory branch: `refs/kim/memory/shared`. Dohyun designed the memory system around "game sense" — a metaphor from competitive gaming for accumulated intuitive knowledge.

Memory tiers:

- **`reflex`**: Recent observations. Like muscle memory in gaming — fast to access, fast to fade. TTL: 48 hours.
- **`awareness`**: Patterns and trends. Like map awareness — knowing where the opponents tend to be. TTL: 14 days.
- **`wisdom`**: Enduring knowledge. Like game sense — knowing *why* opponents behave the way they do. TTL: 180 days.

Promotion between tiers is semi-automatic: entries that are retrieved more than 5 times within their TTL are promoted one tier. Entries that are never retrieved expire. This mimics how game knowledge works — the things you use frequently become deeper knowledge; the things you never recall fade.

Memory retrieval uses key-based lookup with a recency bias: if multiple entries match a key prefix, the most recent is returned. Dohyun considered embedding-based search but Sun-hee asked how much it costs. Dohyun switched to key-based.

## Requirement 6: Signed Commits via OpenWallet

Each family member has an OpenWallet DID. Key management is handled by Dohyun, who keeps the key ceremony simple: generate on a hardware token, back up to a USB drive stored in Grandmother's safe deposit box (Sun-hee insisted). Key rotation every 90 days, aligned with quarterly business reviews.

The signing chain: Yuna generates the patch, Min-jun approves the content, Dohyun signs and commits. Sun-hee does not sign commits. Sun-hee signs invoices.

**Unique insight:** The Kim Dynasty's cost-tracking subcommand (`but ai cost`) reveals a missing capability in most agent frameworks: real-time budget visibility. Most systems track token expenditure after the fact. The Kims track it *during* the task, because Grandmother requires it. This pre-emptive cost awareness changes agent behavior — when the agent can see its remaining budget, it makes different decisions about context reading depth and generation verbosity. Budget-awareness is not just an accounting feature; it is a cognitive constraint that improves output efficiency. The family discovered this by accident: agents running against tight budgets produced more concise, more focused outputs than agents with generous allocations.

---

## Token Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Sun-hee | 500 | 200 | 700 |
| Min-jun | 2,000 | 1,000 | 3,000 |
| Hana | 1,500 | 1,000 | 2,500 |
| Yuna | 4,500 | 4,000 | 8,500 |
| Dohyun | 2,500 | 1,500 | 4,000 |
| **Task Total** | **11,000** | **7,700** | **18,700** |

Monthly budget cap: $1,200 (Grandmother-approved). Grand total per coaching task: **18,700 tokens** (mostly on Ollama — cost: negligible).

---

*"If the computer can do it for free, the computer should do it for free."*
— Park Sun-hee, approving the Ollama deployment
