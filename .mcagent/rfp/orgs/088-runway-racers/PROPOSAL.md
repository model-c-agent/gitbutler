# Runway Racers -- Technical Proposal

**RFP Response: `but-ai` Plugin for GitButler**

---

## Executive Summary

We build fast. We measure everything. Our `but-ai` plugin treats every AI interaction as a timed event with recorded metrics. Performance is not a feature -- it is the architecture.

---

## Requirement 1: PATH-Based Plugin Architecture

Single binary, installed via `cargo binstall but-ai`, discoverable on `$PATH`. The binary self-reports capabilities through `but-ai --manifest` in under 50ms. We benchmark plugin startup time the way we benchmark sprint times: anything over 200ms is a regression.

**Key decisions:**
- Static binary, no runtime deps
- Manifest response cached by `but` CLI for 60 seconds
- Plugin health check (`but-ai --ping`) returns latency metrics alongside status
- All subcommands report execution time in stderr (opt-out via `--quiet`)

---

## Requirement 2: Provider-Agnostic AI

Providers are ranked on a leaderboard. The `Completer` trait normalizes access; the provider selector picks the optimal provider for each task based on historical performance data.

**Providers:** OpenAI, Anthropic, Ollama, LMStudio

**Selection algorithm:**
1. Estimate task complexity (lines of context, expected output size)
2. Query provider leaderboard for best cost/latency ratio at that complexity
3. Route to top-ranked provider
4. Record actual performance and update leaderboard

**Fallback:** If the selected provider fails, automatically retry with the second-ranked provider. Log the failure for leaderboard adjustment.

---

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Every agent task is a "heat" -- a timed, scoped unit of work. The agent receives context, generates INDEX.patch and COMMIT.msg, and records performance metrics.

**Heat protocol:**
1. Start timer
2. Read context (GetProjectStatus, GetBranchChanges)
3. Generate patch
4. Validate (`git apply --check`)
5. Stop timer, record metrics (tokens used, time elapsed, patch size)
6. Commit with signing

**Metric tracking:** Every patch records `X-Heat-Duration`, `X-Tokens-In`, `X-Tokens-Out` in the COMMIT.msg trailer. These are machine-readable and feed into Dr. Osei's analytics pipeline.

---

## Requirement 4: Polyrepo PR Coordination (Forge-Agnostic)

Cross-repo coordination uses structured PR comments with a sports metaphor: each coordinated PR is a "relay leg." The baton is a JSON payload embedded in a comment.

**Forge adapter:** Trait-based, implementations for GitHub, GitLab, Bitbucket. All interactions are adapter-mediated.

**Relay protocol:**
- Leg 1: Agent opens PR in repo-A with a baton comment
- Leg 2: Agent in repo-B reads the baton, performs its work, opens PR with updated baton
- Handoff validation: Each leg must acknowledge the previous leg's baton before starting
- Final merge: All legs must report "finished" before any leg merges

---

## Requirement 5: Agent Memory in Git Branches

Memory stored in `refs/racers/stats/<agent>/` as JSON blobs. Every entry includes performance metadata.

**Memory entry schema:**
```json
{
  "key": "pattern-grading-optimization",
  "value": "...",
  "heat_duration_ms": 1200,
  "tokens_used": 3400,
  "personal_best": false,
  "created": "2026-03-28T09:00:00Z",
  "ttl": 604800
}
```

**Retrieval:** Top-5 by relevance score. Entries flagged `personal_best` get a 1.2x relevance boost -- the system learns from peak performance, not average performance.

---

## Requirement 6: Signed Commits via OpenWallet

Every commit signed. No exceptions. Unsigned commits are treated like unverified performances -- they do not count.

**Implementation:**
- OpenWallet key provisioning at agent startup
- 90-day rotation cycle
- Revocation triggers re-signing of all commits in the affected window (if possible) or flagging (if not)
- Verification via `but-ai verify` with result output matching athletic verification format: PASS / FAIL / UNDER_REVIEW

---

## Token Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Dr. Osei | 5,000 | 600 | 5,600 |
| Viktor | 8,000 | 4,000 | 12,000 |
| Lena | 6,000 | 2,500 | 8,500 |
| Kwame | 3,500 | 900 | 4,400 |
| Priya | 3,200 | 700 | 3,900 |
| **Team Total** | **25,700** | **8,700** | **34,400** |

---

## Unique Insight: Personal Bests as Memory Anchors

In athletics, personal records define an athlete's trajectory. We apply this to agent memory. When an agent achieves a new personal best on any metric (fastest patch, smallest token budget for a given complexity, highest review-pass rate), the memory entry is flagged `personal_best` and given elevated relevance in future retrievals.

This creates a positive feedback loop: agents learn from their best work, not their average work. Over time, the system's baseline performance drifts upward because the memory it draws from is biased toward excellence.

Dr. Osei's data from the first six months of internal testing shows a 17% improvement in patch acceptance rate when personal-best memory anchoring is active versus uniform memory weighting.

---

*"The clock doesn't lie. The diff doesn't lie. Ship it."*
