# Dewey Decimators -- Technical Proposal

**RFP:** GitButler `but-ai` Plugin
**Submitted:** 2026-03-28
**From:** Maya Trujillo, Team Captain

---

## Summary

We are competitive catalogers. We optimize for speed and accuracy under time pressure. Our `but-ai` proposal reflects this: a lean plugin that starts fast, produces clean patches, and gets out of the way. No unnecessary abstractions. No speculative features. Every component earns its token budget or gets cut.

---

## 1. PATH-Based Plugin Architecture

`but-ai` on PATH. Discovered by `but` via PATH search. Invoked per task. Exits on completion.

- Binary: statically linked Rust
- Protocol: JSON lines over stdin/stdout
- Config: `~/.config/but/ai.toml`
- Startup target: under 50ms cold start

We optimize startup time because every millisecond matters in a timed heat. The plugin loads config, validates it, and is ready to receive input in a single initialization pass. No lazy loading, no deferred initialization -- everything is paid for upfront so the hot path is clean.

---

## 2. Provider-Agnostic AI

Minimal `Provider` trait: `complete`, `tool_call`, `stream`. No optional parameters. No provider-specific extensions in the common interface.

| Provider | Latency Profile | Notes |
|----------|----------------|-------|
| Anthropic | 1-3s | Primary target, best tool-calling |
| OpenAI | 1-3s | Compatible, minor schema translation |
| Ollama | 0.5-2s (local) | Fastest for simple tasks |
| LMStudio | 0.5-2s (local) | OpenAI-compatible |

Provider selection is static. Priya considered dynamic provider switching based on latency, tested it, and found the switching overhead exceeded the latency savings. Static wins.

---

## 3. But Agent (INDEX.patch + COMMIT.msg)

### Workflow

1. Read task + workspace state (Maya calls GetProjectStatus)
2. Decompose into subtasks if needed
3. Produce INDEX.patch (unified diff)
4. Produce COMMIT.msg (one-line summary + optional body)
5. Hector validates and signs

### Patch Style

Minimal diffs. We change what the task requires and nothing else. No opportunistic refactoring. No "while we're here" fixes. Those go in separate heats.

### Commit Message Format

```
fix: correct provider timeout handling

Heat: 3
Time: 12:34 remaining
Signed-off-by: Hector (QC)
```

The `Heat:` trailer tracks which work sprint produced the commit. The `Time:` trailer records remaining budget, providing a record of how the team allocated effort.

---

## 4. Polyrepo PR Coordination

Hector manages cross-repo coordination. Structured PR comments in a terse format:

```json
{
  "team": "dewey-decimators",
  "heat": 3,
  "action": "dependency_ready",
  "branch": "feat/provider-timeout",
  "commit": "abc1234",
  "remaining_budget": 14200
}
```

Supported forges: GitHub, GitLab, Bitbucket, Forgejo. Adapter layer is thin -- translates structured JSON to forge comment API. No rendering, no markdown formatting, just data.

---

## 5. Agent Memory in Git Branches

### Heat-Based Memory

Memory is stored in `refs/but-ai/memory/<heat>/<key>`. The heat number is the primary organizing dimension.

| Field | Description |
|-------|-------------|
| `key` | Entry identifier |
| `value` | Memory content |
| `heat` | Which heat created it |
| `relevance` | Score 0-100 |
| `ttl_heats` | Expires after N heats (default: 5) |

### Aging

Memory ages by heat count, not clock time. An entry from 5 heats ago is old regardless of whether those heats took 5 minutes or 5 hours. This maps to how the team actually thinks: recent competition rounds are relevant, old ones are not.

### Retrieval

Top-3 entries by relevance. Aggressive pruning -- the team does not want to read memories during a heat; they want the right memory injected into context automatically.

---

## 6. Signed Commits via OpenWallet

Hector signs all commits. Other agents produce unsigned patches that Hector signs during QC.

- Keys provisioned via OpenWallet at team session start
- Rotation: every 24 hours (daily competition cycle)
- Revocation: immediate, stored in `refs/but-ai/revoked`
- Verification: Verifiable Credential per agent, anchored to repo

Signing is the last step before output. Hector does not sign patches he has not verified. No exceptions, even under time pressure. "You can be fast or you can be wrong. Signing wrong is worse than signing slow."

---

## 7. Token Budget

| Agent | Role | Input | Output | Total |
|-------|------|-------|--------|-------|
| Maya | Patches | 7,800 | 4,200 | 12,000 |
| Deshawn | Memory | 5,000 | 1,200 | 6,200 |
| Priya | Provider/budget | 3,500 | 900 | 4,400 |
| Hector | QC/signing | 4,500 | 1,800 | 6,300 |
| **Total** | | **20,800** | **8,100** | **28,900** |

This is the leanest budget of any team responding to this RFP. We know because we benchmarked. Every token is justified. There is no padding. If a task requires more, we fail fast and report `BUDGET_EXCEEDED` rather than producing degraded output.

---

## 8. Unique Insight: Time-Bounded Task Execution

Most agent systems operate with implicit time assumptions -- they run until they finish or until tokens are exhausted. We propose **explicit time bounding**: every task has a wall-clock deadline in addition to a token budget. When time runs out, the agent produces its best current output, marks it as `HEAT_EXPIRED`, and stops.

This comes from competition. In a cataloging bee, when the timer buzzes, you submit what you have. A partial record with correct fields scores better than no record. A partial patch that handles 3 of 5 cases is more useful than a crashed task that handled none.

Time bounding changes agent behavior. An agent with 2 minutes left and 40% of its task complete will prioritize the highest-value remaining subtask rather than continuing sequentially. This requires task decomposition with priority ordering -- which is exactly what Maya does at the start of every heat.

---

*"Clock's running. Ship it."*
