# OrbitalJanitor.io — Technical Proposal for `but-ai`

**RFP Response | Tier 2 Abbreviated**

---

## Executive Summary

OrbitalJanitor.io proposes a `but-ai` implementation modeled on orbital debris management principles: autonomous agents operating within strict delta-v budgets, producing minimal-footprint patches, and maintaining memory through an orbital decay model that naturally expires stale context. Our domain expertise in risk-scored prioritization translates directly to agent task triage.

## Requirement 1: PATH-Based Plugin Architecture

The `but-ai` binary will be a statically-linked Rust CLI installed to `$PATH`. Discovery follows the `but-tool-ai` convention. The binary accepts structured JSON on stdin and returns results on stdout. No daemon process — cold start per invocation, like a satellite transponder activating on command.

Configuration via `$XDG_CONFIG_HOME/but-ai/config.toml` with environment variable overrides for CI environments. Plugin version negotiation through a `--capabilities` flag that returns supported features as a JSON manifest.

## Requirement 2: Provider-Agnostic LLM Interface

Abstraction layer with four backends: OpenAI, Anthropic, Ollama, LMStudio. Each backend implements a `Provider` trait with `complete()`, `tool_call()`, and `stream()` methods. Provider selection via config with runtime fallback chain: if the primary provider returns a 5xx or times out, the agent cascades to the next provider in the chain.

Token counting is provider-specific (tiktoken for OpenAI, Anthropic's native counting, approximation for Ollama/LMStudio). Delta-V agent tracks actual vs. estimated token usage per call and adjusts future estimates using an exponential moving average — the same algorithm we use for conjunction probability trending.

**Domain Insight:** In orbital mechanics, you never commit to a maneuver you cannot complete with remaining fuel. Same principle: never start a patch generation cycle if the remaining token budget cannot cover the worst-case output length. We call this the "minimum delta-v check" and it runs before every LLM call.

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Agents produce unified diffs written to `INDEX.patch` and commit metadata written to `COMMIT.msg`. The patch file is the agent's only write channel — no direct file mutations. Apogee agent validates patch applicability with `git apply --check` before submission.

Patch generation follows a three-phase cycle:
1. **Survey** — Read project status, branch changes, recent commits (seismic before drill, to borrow BoreStack's metaphor, though we would call it "radar before rendezvous").
2. **Generate** — Produce the diff. Apogee targets the smallest correct patch, measured by hunks not lines.
3. **Verify** — Apply in sandbox, run lint, confirm no unintended file changes.

COMMIT.msg follows Conventional Commits with an appended metadata block containing agent ID, provider used, token cost, and memory refs consulted.

## Requirement 4: Polyrepo PR Coordination

COLA agent handles cross-repository coordination using a conjunction screening model. Before opening PRs across repos, COLA identifies "conjunction windows" — time periods where changes in different repos might interact. PRs are grouped into coordination sets with a shared tracking ref.

Forge-agnostic adapter layer supports GitHub, GitLab, and Gitea via their respective REST APIs. PR comments use a structured schema with machine-readable metadata blocks (hidden in HTML comments) and human-readable summaries above.

Dependency ordering uses branch name encoding (`s01.s02` convention). COLA monitors PR status across repos and triggers downstream PRs when dependencies merge.

## Requirement 5: Agent Memory in Git Branches

Memory stored in `refs/but-ai/memory/<agent-id>/<namespace>` as Git blobs. Perigee manages the memory lifecycle using an orbital decay model:

| Decay Class | Initial TTL | Boost on Reuse | Analogy |
|-------------|-------------|----------------|---------|
| GEO | 90 days | +30 days | Geostationary — long-lived reference memory |
| MEO | 30 days | +14 days | Medium orbit — project-scoped patterns |
| LEO | 7 days | +3 days | Low orbit — task-specific context |
| Suborbital | 1 session | None | Ballistic — single-use, no persistence |

Relevance scoring uses cosine similarity between the task embedding and memory entry embeddings. Top-5 retrieval with a minimum similarity threshold of 0.4. Memory entries below threshold decay faster (reduced boost on reuse).

Garbage collection runs on `but-ai memory gc`, which removes expired refs and compacts the memory tree. Integrates with Git's native GC for object cleanup.

## Requirement 6: Signed Commits via OpenWallet

Seal agent manages OpenWallet integration. Each agent identity maps to a DID (Decentralized Identifier). Signing keys are provisioned at agent initialization and stored in the system keychain (libsecret on Linux, Keychain on macOS).

Commit signing uses the OpenWallet `sign` API with the agent's DID as the signer. Signature verification is embedded in the PR review workflow — COLA agent checks signatures on all patches before including them in a coordination set.

Key rotation follows a 30-day cycle with 7-day overlap for signature verification of in-flight commits. Emergency revocation publishes a revocation entry to the memory branch, and all agents check revocation status before trusting a signature.

## Token Budget

| Agent | Role | Input/task | Output/task | Total |
|-------|------|-----------|-------------|-------|
| Apogee | Patch generation | 7,500 | 4,000 | 11,500 |
| Perigee | Memory management | 5,000 | 800 | 5,800 |
| Delta-V | Budget & providers | 3,500 | 1,000 | 4,500 |
| COLA | PR coordination | 5,500 | 2,500 | 8,000 |
| Seal | Signing | 2,500 | 500 | 3,000 |
| **Per-task total** | | **24,000** | **8,800** | **32,800** |

Budget assumes a 200-line, 3-file feature task. Complex multi-repo coordination tasks may require up to 2x. Delta-V enforces hard caps and will produce partial results (with `BUDGET_LOW: partial patch, N tokens remaining` flag) rather than exceed allocation.

## Unique Domain Insight

Orbital debris management taught us that the cost of inaction compounds exponentially. One collision creates hundreds of fragments, each capable of causing another collision — the Kessler syndrome. The same dynamic exists in codebases: one unreviewed agent commit can introduce a pattern that other agents replicate, compounding technical debt at machine speed.

Our proposal includes a "Kessler detector" — a static analysis pass that flags agent-generated patterns appearing in more than 3 files without human review. If triggered, the agent pauses and requests explicit human approval before continuing. This is not a feature we have seen in other proposals, and it comes from watching what happens when autonomous systems operate without feedback loops.

---

*Current LEO object count at time of writing: 28,441. Tomorrow it will be higher.*
