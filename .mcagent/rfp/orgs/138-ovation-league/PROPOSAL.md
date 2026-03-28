# PROPOSAL.md — Ovation League

**"Fast commits. Bold choices. The audience scores."**

---

## Summary

The Ovation League proposes to build the `but-ai` plugin as competitive improv. Speed is skill. Hesitation wastes tokens. The agent commits to a choice, executes, gets scored (reviewed), and adjusts. The memory system tracks what the "audience" (the codebase, the reviewer, the tests) responds to, creating a feedback loop that makes every subsequent scene better.

---

## Technical Proposal

### Requirement 1: PATH-Based Plugin Architecture

`but-ai` on PATH. TOML manifest. Standard discovery. We do not overthink the entrance — we just walk on stage. Config in `.but/ai.toml`. Provider via flag or config. Done. Next scene.

### Requirement 2: Provider-Agnostic AI

Different stages, same show. The `Stage` trait: `perform(prompt) -> Completion`, `read_room(completion) -> ToolCalls`, `check_score(usage) -> TokenReport`. Four adapters. The league doesn't care about the venue. We care about the performance.

The Ovation League adds latency awareness: each provider adapter tracks response latency and reports it alongside token usage. For improv agents, latency is a performance metric — a slow provider is like a slow scene partner. It kills the energy.

### Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

The agent workflow is an improv scene:

1. **Game call** — Deshawn reads the task, identifies the pattern, calls the format
2. **Scorebook** — Tyrell retrieves memory: what worked before? what didn't?
3. **Initiation** — Kenji commits to a choice and produces INDEX.patch + COMMIT.msg fast
4. **Support** — Ava "yes, ands" — coordinates, reviews, integrates
5. **Score** — Deshawn reviews against the game call: land or reset

The emphasis is on speed-to-first-patch. Kenji aims to produce a complete INDEX.patch within the token equivalent of 3 seconds of scene time. Revisions are allowed (max 2 resets) but the first attempt should be shippable 70% of the time. This is not reckless. It is trained instinct — pattern recognition from thousands of prior scenes (memory entries).

### Requirement 4: Polyrepo PR Coordination

Ava treats cross-repo coordination as a group scene — multiple performers, shared reality, everyone must maintain the same world. PR comments are scene offers:

```
[OVATION-OFFER] from: backend | to: auth | offer: token-refresh-ready
accept-by: next-scene | heighten: add retry-with-backoff when ready
```

Forge-agnostic: Ava implements a `Venue` trait for GitHub/GitLab/Gitea: `make_offer(offer)`, `accept_offer(offer_id)`, `heighten(offer_id, addition)`. The improv metaphor is exact: an offer is a proposal that the other repo's agent can accept and build on, or deny (but denial costs points).

### Requirement 5: Agent Memory in Git Branches

Memory is the scorebook, stored in `refs/ovation/scorebook/`:

```json
{
  "scene_id": "OVL-2026-0847",
  "game": "feat/auth-refactor",
  "pattern": "middleware-centralization",
  "move": "Centralized JWT validation in single middleware",
  "score": 8.5,
  "audience_note": "Clean approach, tests pass, reviewer approved first round",
  "scored_by": "tyrell",
  "ttl_games": 20
}
```

**The scorebook memory model:** Every memory entry has a `score` — a retrospective assessment of how well the approach worked, rated 1-10. High-scoring memories are retrieved preferentially for similar tasks. Low-scoring memories are retrieved as "what not to do" context. The score is assigned after the task completes (not during), based on outcome: did the patch ship? how many review rounds? any regressions?

**Unique memory scheme:** Pattern-based retrieval. The `pattern` field captures the abstract structure of the scene, not the specifics. "middleware-centralization" is a pattern. "JWT validation in auth middleware" is a specific. When Kenji faces a new task, Tyrell retrieves by pattern first ("has the team done a centralization scene before?") and by specifics second. This enables transfer learning: a high-scoring centralization in auth informs a centralization decision in logging.

### Requirement 6: Signed Commits via OpenWallet

Deshawn signs. The commissioner's signature on the scorecard. DID-bound key via OpenWallet. Signing happens after the final score — no signing mid-scene. Key rotation: at the start of each season (quarterly). Emergency rotation: Deshawn calls a timeout, all games (tasks) pause until keys are rotated.

---

## Token Budget

| Agent | Input | Output | Total | Role |
|-------|-------|--------|-------|------|
| Deshawn Mitchell | 6,000 | 2,000 | 8,000 | Strategy, review, signing |
| Kenji Sato | 8,500 | 6,500 | 15,000 | Patch generation |
| Ava Rodriguez | 6,500 | 3,000 | 9,500 | Coordination, support |
| Tyrell Washington | 5,500 | 1,500 | 7,000 | Memory, scoring |
| **Team Total** | **26,500** | **13,000** | **39,500** | |

Scene overhead: ~2,500 tokens (game calls, scoring, resets).
**Total per task: ~42,000 tokens.**

---

## Unique Insight

**Retrospective scoring creates a feedback loop that improves pattern recognition.** Most memory systems store information and retrieve it by relevance. The Ovation League scores every memory after the fact — was this approach successful? — and uses the scores to rank future retrievals. Over time, the scorebook becomes a performance model: the agent learns not just what approaches exist, but which ones work. A memory with a score of 9.2 from a similar task is retrieved with high confidence. A memory with a score of 3.0 is retrieved as a warning. The feedback loop is what separates practice from performance: every scene teaches the next one.

---

*"The Golden Callback goes to the team that learns the fastest."*
