# RideStack — Technical Proposal for `but-ai`

**RFP Response | Tier 2 Abbreviated**

---

## Executive Summary

RideStack proposes a `but-ai` implementation optimized for development velocity: parallel agent execution, preemptive PR creation, asynchronous signing, and aggressive iteration cycles. Our domain expertise in integrating fragmented transit data systems translates to agents that connect disparate codebases quickly through adapter-first thinking.

## Requirement 1: PATH-Based Plugin Architecture

Rust binary at `$PATH` as `but-tool-ai`. Optimized for cold-start latency (target: <100ms to first useful output). The binary supports a `--stream` flag that outputs partial results as they are generated, enabling the calling tool to display progress in real time.

Configuration via `$XDG_CONFIG_HOME/but-ai/config.toml`. RideStack adds a `[speed]` section with configurable iteration limits, parallel agent count, and draft PR behavior. Defaults are aggressive; conservative users can dial them back.

## Requirement 2: Provider-Agnostic LLM Interface

Four-provider backend. Fare agent optimizes for cost-per-accepted-patch, a compound metric that factors in both token cost and acceptance probability (estimated from the agent's recent history). When two providers produce similar quality, Fare always picks the cheaper one.

Provider selection is dynamic and per-call (not per-task). A single task may use multiple providers: a frontier model for the initial patch generation and a smaller model for revision based on review feedback. Fare routes each call independently.

**Domain Insight:** In MaaS, the cheapest mode is not always the fastest, and the fastest is not always the best. A scooter is cheap and fast for 2km but terrible for 20km. Fare applies the same multimodal routing logic to providers: different calls in the same task go to different providers based on the call's requirements.

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Route generates patches through a rapid iteration cycle:
1. **Quick read** — Minimal context loading: project status, target files, recent commits. Skip full codebase surveys.
2. **First draft** — Generate INDEX.patch quickly. Favor coverage over perfection.
3. **Self-review** — Quick sanity check: does it apply? Does it address the task? Major issues only.
4. **Ship** — Submit for review. If rejected, iterate using review feedback. Max 2 iterations.

COMMIT.msg is compact:
```
feat(adapter): add GBFS v3 feed normalizer

Agent: Route | Iter: 1/2 | Cost: $0.003 | Modes: 14
```

## Requirement 4: Polyrepo PR Coordination

Junction manages cross-repo coordination with a "preemptive draft" strategy:
- At task start, Junction opens draft PRs in all affected repos immediately.
- As patches land, drafts are updated and marked ready for review.
- Cross-repo dependencies are tracked in PR descriptions with status badges.
- Junction does not block merges for coordination — it warns. Human operators decide whether to wait.

This is faster than synchronous coordination but riskier. RideStack accepts the risk because iteration is cheap and rollback is supported.

Forge adapters (GitHub, GitLab, Gitea) focus on speed: minimal API calls, batch operations where supported, aggressive caching of repo metadata.

## Requirement 5: Agent Memory in Git Branches

Transfer manages memory using a "transfer point" model. Memories are stored as connections between contexts:

| Field | Description |
|-------|-------------|
| `id` | Unique identifier |
| `origin_context` | Where the memory was learned (project, task, branch) |
| `content` | Memory payload |
| `applicability` | List of context tags where this memory is useful |
| `hit_count` | Times this memory has been retrieved and used |
| `ttl_days` | 7 base, +3 per hit (max 30) |

Memory stored in `refs/ridestack/transfer/`. Retrieval is cross-context: Transfer actively searches for memories from OTHER projects that might apply to the current one. This is RideStack's distinguishing memory feature — most proposals retrieve only within the current project.

GC runs automatically after each task, evicting expired entries. The memory footprint stays small.

## Requirement 6: Signed Commits via OpenWallet

Tap handles signing with asynchronous batch pre-signing:
1. At task start, Tap pre-signs N commit slots (default: 5) with the agent's DID key.
2. As patches are produced, they fill pre-signed slots.
3. If more patches are produced than pre-signed slots, Tap signs additional slots on demand.

This removes signing from the critical path. The tradeoff is that pre-signed slots include a nonce that expires after 1 hour, so unused slots are wasted. Fare considers this acceptable given the latency savings.

Key rotation every 30 days. Emergency revocation invalidates all pre-signed slots immediately.

## Token Budget

| Agent | Role | Input/task | Output/task | Total |
|-------|------|-----------|-------------|-------|
| Route | Patch generation | 7,000 | 4,500 | 11,500 |
| Transfer | Memory & transfer | 4,500 | 800 | 5,300 |
| Fare | Provider & budget | 2,500 | 600 | 3,100 |
| Junction | PR coordination | 4,000 | 2,000 | 6,000 |
| Tap | Signing (async) | 2,000 | 400 | 2,400 |
| **Per-task total** | | **20,000** | **8,300** | **28,300** |

The leanest budget in this cohort. We ship more with less. That is the startup way.

## Unique Domain Insight

Three years of integrating fragmented transit data taught us that the adapter pattern is the only architecture that scales across incompatible systems. You cannot force GTFS and GBFS and TOMP to agree on a schema. You write adapters that normalize each into your internal representation and move on.

Our proposal applies this insight to the provider abstraction and forge adapter layers. Instead of building a unified provider interface that tries to expose every feature of every backend, we build thin adapters that normalize each provider's capabilities into the minimum viable interface that `but-ai` needs. New providers are added by writing an adapter, not by extending a complex abstraction. This is less elegant than a unified interface. It is also faster to implement, easier to maintain, and less likely to break when a provider changes their API.

---

*Modes: 14. PRs: draft. Ship it now. Fix it tomorrow.*
