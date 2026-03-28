# HitForge -- Technical Proposal

**RFP:** `but-ai` Plugin for GitButler
**Velocity Target:** <3 min time-to-merge
**Submission Date:** 2026-03-28

---

## Executive Summary

We propose a speed-first implementation of `but-ai`. Our design prioritizes latency reduction at every layer: parallel agent execution, aggressive caching, speculative patch generation, and a single-round review policy. We accept higher token costs in exchange for faster wall-clock delivery.

The music production industry taught us that time kills creativity. A producer waiting 20 minutes for a bounce is a producer who loses momentum. A developer waiting 10 minutes for an AI agent to produce a patch is a developer who context-switches. We build for momentum.

---

## Requirement 1: PATH-Based Plugin Architecture

The plugin binary (`but-ai`) is installed via `cargo binstall` or direct download. It registers with the `but` CLI through PATH discovery -- no manual configuration. On first invocation, it creates `~/.config/but-ai/config.toml` with sensible defaults and prints a one-line confirmation.

Startup time budget: <200ms cold, <50ms warm. We achieve warm starts through a persistent daemon process (`but-ai daemon`) that stays resident between invocations and maintains provider connections, memory caches, and pre-loaded system prompts. The daemon is optional -- the plugin works without it, just slower.

---

## Requirement 2: Provider-Agnostic AI

Four providers: OpenAI, Anthropic, Ollama, LMStudio. A `Provider` trait with three methods: `complete`, `stream`, `estimate_tokens`. We intentionally keep the trait minimal -- feature differences between providers are handled by capability flags, not separate methods.

Provider hot-switching is supported. If the primary provider returns an error or exceeds latency threshold (configurable, default 5s), the system falls back to the next configured provider automatically. This is a production necessity -- we have seen OpenAI rate-limit mid-session and cannot afford to halt the pipeline.

Provider benchmarking runs on first configuration: the plugin sends a standard prompt to each configured provider and records latency, token count accuracy, and tool-calling reliability. Results are stored in memory for routing decisions.

---

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

### Speculative Execution

Our key innovation: the agent begins generating the patch before the full context is loaded. As context streams in (file contents, branch state, memory entries), the agent revises its partial patch incrementally. By the time context loading is complete, the patch is 60-80% finished.

This is speculative execution applied to code generation. It risks wasted tokens if early context is misleading, but our benchmarks show a 40% reduction in wall-clock time for typical tasks. The speculative approach is configurable and can be disabled for high-stakes tasks.

### Patch Validation

Patches are validated in three steps:
1. `git apply --check` (syntactic validity)
2. Test suite execution if configured (semantic validity)
3. Producer review (contextual validity)

Steps 1 and 2 run in parallel. If step 1 fails, the patch is regenerated (up to 2 retries). If step 2 fails, the failure is included in the review context for Producer.

---

## Requirement 4: Polyrepo PR Coordination

Cross-repo coordination uses lightweight PR comments with a structured schema:

```
<!-- but-ai:coord {"from":"repo-a","dep":"repo-b#branch","status":"pending"} -->
```

Embedded in HTML comments so they are invisible to human readers but parseable by agents. Forge-agnostic -- works on any platform that supports PR comments (all of them).

Dependency tracking is optimistic: patches are generated assuming dependencies will be met, and coordination comments update status asynchronously. This avoids blocking the pipeline on cross-repo round-trips.

---

## Requirement 5: Agent Memory in Git Branches

Memory is stored in `refs/but-ai/mem/<agent>/<key>` as JSON blobs. We use aggressive TTL defaults:

| Memory Type | Default TTL | Rationale |
|-------------|-------------|-----------|
| Task context | 24 hours | Stale context causes more harm than missing context |
| Code patterns | 7 days | Patterns are stable but codebases evolve |
| Provider metrics | 30 days | Provider performance changes slowly |
| Coordination state | 1 hour | Cross-repo state is highly volatile |

Retrieval uses keyword matching (not vector similarity) because it is faster and our benchmarks show comparable precision for code-related queries. We sacrifice recall for latency.

### Hit Rate Scoring

Borrowed from our music prediction model: every memory retrieval is scored by the downstream agent. Was this memory useful? The scores feed back into the retrieval algorithm, creating a recommendation loop. Over time, the memory system learns which types of memories are valuable for which types of tasks.

---

## Requirement 6: Signed Commits via OpenWallet

Each agent has a DID-based identity provisioned through an OpenWallet-compatible provider. Signing uses Ed25519 keys with 24-hour rotation. The signing flow:

1. Agent requests ephemeral signing key from wallet
2. Patch is hashed (SHA-256 of INDEX.patch content)
3. Commit is signed with the ephemeral key
4. Signature metadata (DID, key ID, expiry) is embedded in the commit trailer

We pre-fetch signing keys at daemon startup to avoid latency during commit. Keys are cached in memory (never on disk) and refreshed 30 minutes before expiry.

---

## Token Budget

| Agent | Input | Output | Total | Role |
|-------|-------|--------|-------|------|
| A&R | 5,000 | 2,500 | 7,500 | Task triage |
| Engineer | 10,000 | 8,000 | 18,000 | Patch generation |
| Producer | 7,000 | 3,000 | 10,000 | Review |
| Mixer | 5,000 | 1,500 | 6,500 | Budget/memory/signing |
| **Team** | **27,000** | **15,000** | **42,000** | |

Speculative execution adds ~15% overhead (included above). The budget assumes one review round. Tasks requiring re-scoping consume an additional A&R cycle (7,500 tokens).

---

## Unique Insight: Momentum Is a Feature

Every AI agent proposal will talk about accuracy, safety, and reproducibility. Those matter. But the metric that determines whether developers actually use the tool is latency.

Our prediction model taught us this. The first version of HitForge had 78% accuracy but took 45 minutes to analyze a track. Nobody used it. We dropped accuracy to 73% and reduced analysis time to 90 seconds. Usage increased 12x. The lesson: a slightly worse answer delivered immediately is more valuable than a perfect answer delivered late, because the user is still in context to act on it.

We apply this to `but-ai`: every architectural decision is evaluated against wall-clock time. Parallel execution, speculative generation, aggressive caching, single-round review -- these are not shortcuts. They are the recognition that developer attention is the scarcest resource in the system, and any millisecond spent waiting is a millisecond of attention lost.

---

*Ship it. Measure it. Fix it. Repeat.*
