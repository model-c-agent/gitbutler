# The Olympian Friars — Technical Proposal

**RFP Response: `but-ai` Plugin for GitButler**

---

## Executive Summary

We propose a `but-ai` implementation structured around **liturgical cycles** — bounded work sessions with mandatory checkpoints. Our approach treats agent operation as a discipline, not a stream. Every agent works in defined intervals, produces checkpointed output, and rests. This prevents runaway token consumption, forces incremental commits, and aligns naturally with GitButler's virtual branch model where each cycle's work is isolated until review.

---

## Requirement 1: PATH-based Plugin Architecture

The `but-ai` binary will be installed to `$XDG_BIN_HOME` (defaulting to `~/.local/bin`) and discovered by the `but` CLI via PATH lookup. The plugin follows the Git subcommand convention: `but ai <verb>`.

**Implementation:**
- Single static binary, no runtime dependencies
- Subcommands: `but ai patch`, `but ai review`, `but ai memory`, `but ai budget`
- Configuration via `$XDG_CONFIG_HOME/but-ai/config.toml` with environment variable overrides
- Plugin version handshake: `but ai --protocol-version` returns compatibility range

**Insight:** We model the plugin as a monastic cell — self-contained, requiring nothing from outside except what is explicitly passed through the door (stdin, env vars, CLI args).

---

## Requirement 2: Provider-Agnostic AI

All LLM calls pass through a **Provider Office** abstraction — named after the monastic offices that structure our day.

**Supported providers:**
- OpenAI (GPT-4o, o3)
- Anthropic (Claude Sonnet, Opus)
- Ollama (local models)
- LMStudio (local models)

**Architecture:**
- Trait-based provider interface: `ProviderOffice { fn invoke(&self, prompt: Prompt, budget: TokenBudget) -> Result<Response> }`
- Provider selection via config or `BUT_AI_PROVIDER` env var
- Capability detection: each provider reports supported features (tool calling, streaming, JSON mode)
- Fallback chain: if primary provider fails, secondary is attempted with budget adjustment

**Unique element:** Provider calibration files. Each provider has a calibration profile recording observed token-per-character ratios, latency percentiles, and tool-calling accuracy. These profiles are stored in `refs/friars/calibration/<provider>` and updated after each session.

---

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Each agent operates within a **cycle** — a bounded work session analogous to a liturgical hour.

**Cycle structure:**
1. **Invocation** — Agent reads task, retrieves explicitly requested memories
2. **Lectio** — Agent scans codebase context within token budget
3. **Labor** — Agent produces INDEX.patch (unified diff) and COMMIT.msg
4. **Checkpoint** — Output is committed to the agent's virtual branch
5. **Rest** — Agent state is serialized; agent halts until next cycle

**Patch format:** Standard unified diff. COMMIT.msg follows Conventional Commits with an additional `Cycle:` trailer indicating which work session produced it.

**Key design decision:** An agent never modifies files directly. All output is INDEX.patch. The `but` CLI applies the patch. This separation ensures that a malformed agent output cannot corrupt the working tree.

---

## Requirement 4: Polyrepo PR Coordination

Cross-repository coordination uses a **Chapter Meeting** protocol — named after the monastic chapter where community decisions are made.

**Protocol:**
- Each repository is a "cell" with its own agent team
- Coordination happens via structured PR comments following a schema: `<!-- but-ai:chapter:{action}:{payload} -->`
- Actions: `propose`, `consent`, `block`, `complete`
- A cross-repo change requires consent from agents in all affected repositories
- Forge-agnostic: adapter trait supports GitHub, GitLab, Gitea, Forgejo

**Dependency tracking:** Cross-repo dependencies are recorded in a manifest file (`.but-ai/chapters.toml`) listing repository URLs and branch dependencies. The manifest is committed to each participating repo.

---

## Requirement 5: Agent Memory in Git Branches

Memory is stored in Git refs under `refs/but-ai/memory/` with a structure inspired by our liturgical cycle model.

**Memory hierarchy:**
- `refs/but-ai/memory/session/<id>` — Ephemeral, expires at cycle end
- `refs/but-ai/memory/task/<id>` — Persists across cycles within one task
- `refs/but-ai/memory/project/<key>` — Long-lived project patterns
- `refs/but-ai/memory/communal/<key>` — Shared across all agents (read-only by default)

**Invocation model:** Memories are not automatically injected. Each cycle begins with a "lectio" phase where the agent scans memory headers (key + summary, ~10 tokens each) and explicitly requests full retrieval of relevant entries. This prevents memory bloat and keeps token budgets predictable.

**Expiration:** TTL-based. Session memories: 1 hour. Task memories: 7 days. Project memories: 90 days. Communal memories: no expiration, manual curation only.

---

## Requirement 6: Signed Commits via OpenWallet

Every agent commit is signed using an OpenWallet-managed key pair.

**Key lifecycle:**
- **Provisioning:** Key generated at agent deployment, bound to agent identity
- **Rotation:** Every 30 days or 500 commits, whichever comes first
- **Revocation:** Immediate on compromise detection; all commits signed by revoked key are flagged
- **Attestation:** Each signed commit includes a Verifiable Credential linking the agent identity to its key

**Signing flow:**
1. Agent produces INDEX.patch + COMMIT.msg
2. Patch is hashed (SHA-256)
3. Agent requests signature from OpenWallet credential
4. Signed commit is created with `but commit --sign`
5. Signature verification is recorded in `refs/but-ai/attestations/<commit-sha>`

**Witness requirement:** Our implementation supports optional multi-agent signing — a commit can require signatures from N agents before it is considered valid. Default: 1. Recommended for critical paths: 2.

---

## Token Budget

### Per-Task Budget (typical 200-line, 3-file feature)

| Agent | Role | Input | Output | Total |
|-------|------|-------|--------|-------|
| Matteo | Patch generation | 8,500 | 3,800 | 12,300 |
| Giacomo | Provider/forge | 6,200 | 2,500 | 8,700 |
| Simone | Memory | 5,800 | 600 | 6,400 |
| Luca | Signing | 3,200 | 800 | 4,000 |
| **Total** | | **23,700** | **7,700** | **31,400** |

### Budget Envelope by Task Complexity

| Complexity | Multiplier | Total Tokens |
|------------|-----------|--------------|
| Trivial (single file, <50 lines) | 0.5x | 15,700 |
| Standard (3 files, ~200 lines) | 1.0x | 31,400 |
| Complex (10+ files, cross-repo) | 2.0x | 62,800 |
| Extraordinary (architecture change) | 3.0x | 94,200 |

Budget overruns trigger graceful degradation: partial patches with explicit `PARTIAL:` markers and continuation instructions for the next cycle.

---

## Unique Insight: Bounded Cycles as a Token Governance Mechanism

Most AI agent systems run in open-ended loops: the agent works until it finishes or hits a hard token limit. Both failure modes are ungraceful — the first risks runaway costs, the second produces truncated output.

Our liturgical cycle model introduces a third option: **time-bounded work sessions with mandatory checkpoints**. An agent does not run until it finishes. It works for one cycle, produces the best output it can within that cycle's budget, checkpoints its state, and stops. If the task requires more work, it resumes in the next cycle with fresh budget and explicit memory invocation.

This has three practical benefits:
1. **Predictable costs.** Each cycle has a fixed budget. Total cost = number of cycles x cycle budget.
2. **Incremental review.** Each cycle produces a commit. Reviewers can inspect work-in-progress after every cycle, not just at the end.
3. **Natural retry boundaries.** If a cycle produces bad output, only that cycle's work is discarded. Previous cycles' checkpoints are preserved.

The monastery taught us this: sustained attention is not about working longer. It is about working in rhythm.

---

*Submitted under the seal of the Abbey of San Luca.*
*Ora et data.*
