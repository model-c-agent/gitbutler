# Dominguez Orbital Services — Technical Proposal for `but-ai`

**RFP Response | Tier 2 Abbreviated**

---

## Executive Summary

Dominguez Orbital Services proposes a `but-ai` implementation built on the principles of a 37-year family operation: reliability over innovation, clarity over cleverness, and backward compatibility as a first-class requirement. Our four-agent architecture is deliberately simple — fewer moving parts, fewer failure modes, easier to debug at 2 AM when the Calima knocks out the power.

## Requirement 1: PATH-Based Plugin Architecture

Rust binary at `$PATH` as `but-tool-ai`. Stateless invocation. The binary is compiled with minimal dependencies — the Dominguez family learned from Esteban that every dependency is a liability. Static linking, no runtime requirements beyond a working filesystem and network stack.

Configuration: `$XDG_CONFIG_HOME/but-ai/config.toml` with environment variable overrides. Repository config in `.but-ai.toml`. The configuration format is documented with comments explaining every field — Carmen's rule: "If the next person cannot understand the config file, the config file is wrong."

## Requirement 2: Provider-Agnostic LLM Interface

Four backends (OpenAI, Anthropic, Ollama, LMStudio). Palma manages provider selection with a stability bias: the default provider is used until it fails, then the fallback chain activates. No dynamic routing, no performance-based switching. The family prefers predictability: same provider, same behavior, same cost model. Switching happens only on failure or by explicit human configuration change.

Token counting uses provider-native methods. Budget tracking is simple: per-task allocation, hard cap, no borrowing between tasks. When budget is exhausted, the agent produces a partial result with a clear explanation of what was completed and what was not.

**Domain Insight:** Esteban tracked satellites with a Commodore 64 that had 64KB of RAM. Carmen automated the station on a Pentium II. The family's instinct is to do more with less. Palma's budget management reflects this: the default token allocation is deliberately tight, forcing agents to be efficient. Teams that want larger budgets must explicitly configure them. The safe default is frugal.

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Tenerife generates patches through a methodical process:
1. **Read everything** — Project status, branch state, all files that will be modified, recent commits. Tenerife does not guess what is in a file.
2. **Match the style** — Analyze existing code conventions (naming, formatting, patterns) and replicate them exactly.
3. **Generate the minimal diff** — Smallest correct patch. No opportunistic refactoring. No style improvements beyond the task scope.
4. **Validate** — `git apply --check` and basic lint. If validation fails, regenerate once. If it fails again, produce a partial patch with explanation.

COMMIT.msg is plain and descriptive:
```
fix: handle leap second in TLE epoch parsing

Corrects off-by-one in epoch day calculation for TLEs
spanning a leap second boundary. Affects objects tracked
during 2024-12-31 23:59:60 UTC.

Agent: Tenerife
Budget: 1,800/1,200 tokens
Weather: La Laguna, 19C, partly cloudy, seeing 3/5
```

## Requirement 4: Polyrepo PR Coordination

Palma handles cross-repo coordination with a "shipping manifest" model (borrowed from Carmen's years at Telefonica). Each multi-repo change set has a manifest:
- List of repos involved.
- List of PRs per repo.
- Merge order (sequential, not parallel — predictable over fast).
- Status of each PR.

Manifests are stored in `refs/dominguez/manifests/`. Palma checks manifest consistency before any PR in the set is merged. If a dependency is not ready, the merge is blocked with a clear message explaining what is waiting for what.

Forge adapters (GitHub, GitLab, Gitea) implement a minimal interface. PR comments are human-readable first, machine-readable second.

## Requirement 5: Agent Memory in Git Branches

Gomera manages memory using a "generational archive" model:

| Generation | Scope | TTL | Retrieval Priority |
|------------|-------|-----|--------------------|
| Current (Diego) | This task | 1 session | Highest |
| Recent (Carmen) | This project, last 30 days | 30 days | High |
| Foundational (Esteban) | Codebase-wide patterns | 180 days | Medium |

Memory stored in `refs/dominguez/archive/<generation>/<entry-id>` as JSON blobs. Each entry has a creation date, last-accessed date, and access count. Entries that are accessed frequently get promoted to a higher generation. Entries that are never accessed decay naturally.

Relevance scoring is simple: keyword match with recency weighting. No embeddings, no vector search. Sofía argued for semantic search; Diego vetoed it as too complex for the reliability guarantee they want. "If you cannot explain the retrieval algorithm to Carmen, it is too complicated."

GC runs on explicit invocation (`but-ai memory gc`), not automatically. The family prefers manual maintenance — they have seen too many automated cleanup jobs delete the wrong thing.

## Requirement 6: Signed Commits via OpenWallet

Hierro handles signing with a conservative key lifecycle:
- 90-day key rotation (longest in this RFP).
- 14-day overlap for in-flight signature verification.
- Manual key provisioning — no automated key generation.
- Emergency revocation requires two agents (Hierro + Palma) to agree, preventing single-point revocation errors.

Signing is the final step in the pipeline. Hierro signs only after the complete sequential review chain (Gomera memory check, Tenerife patch validation, Palma budget/coordination check) has completed. No shortcuts.

## Token Budget

| Agent | Role | Input/task | Output/task | Total |
|-------|------|-----------|-------------|-------|
| Tenerife | Patch generation | 8,500 | 4,500 | 13,000 |
| Gomera | Memory & history | 5,000 | 800 | 5,800 |
| Palma | Budget, provider, coordination | 6,000 | 2,500 | 8,500 |
| Hierro | Signing & trust | 2,500 | 500 | 3,000 |
| **Per-task total** | | **22,000** | **8,300** | **30,300** |

Lean budget. Four agents instead of five. Combined roles where possible. The Dominguez way: do more with less.

## Unique Domain Insight

Thirty-seven years of family operation taught us that the systems that survive are not the most advanced — they are the most maintainable. Esteban's Commodore programs were crude, but Carmen could read them. Carmen's shell scripts were unsophisticated, but Diego could modify them. Each generation inherited the previous generation's code and extended it without rewriting it.

Our proposal applies this principle to agent memory: the generational model ensures that foundational knowledge persists across task lifetimes while recent context stays fresh. More importantly, the memory format is intentionally simple (JSON, keyword-indexed, no embeddings) so that a human can read, audit, and correct any memory entry with a text editor. Memory systems that require specialized tools to inspect are memory systems that will rot in the dark.

---

*Station uptime: 99.7%. Bodega open Saturdays. Come for the tracking data, stay for the wine.*
