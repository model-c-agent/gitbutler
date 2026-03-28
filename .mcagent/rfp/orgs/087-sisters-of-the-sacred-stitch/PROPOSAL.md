# Sisters of the Sacred Stitch -- Technical Proposal

**RFP Response: `but-ai` Plugin for GitButler**

---

## Executive Summary

We propose a `but-ai` plugin built on principles of stewardship, accountability, and thrift. Every token spent must justify itself against the order's charitable mission. Our architecture draws from nine centuries of vestment craft: cut precisely, stitch once, waste nothing.

---

## Requirement 1: PATH-Based Plugin Architecture

The `but-ai` binary will be a standalone executable installed to `$PATH`, discovered by the `but` CLI at runtime through the existing plugin resolution mechanism. No dynamic linking. No runtime dependencies beyond the binary itself.

**Design:**
- Single static binary (Rust, cross-compiled for linux-x86_64, darwin-arm64, darwin-x86_64)
- Plugin manifest embedded in the binary as a `--manifest` subcommand output
- Discovery via `$PATH` prefix matching: `but` scans `$PATH` for `but-ai*` and invokes `but-ai --manifest` to negotiate capabilities
- Graceful degradation: if `but-ai` is absent from `$PATH`, the `but` CLI operates normally without AI features

**Why this way:** The sisters have no IT department. Installation must be `cargo binstall but-ai` and nothing else. Binary simplicity is a form of mercy to the end user.

---

## Requirement 2: Provider-Agnostic AI

The plugin abstracts LLM providers behind a trait boundary. All providers implement a single `Completer` trait with methods for chat completion, tool calling, and token counting.

**Supported providers:**
- OpenAI (GPT-4o, GPT-4o-mini)
- Anthropic (Claude Sonnet, Claude Haiku)
- Ollama (local models)
- LMStudio (local models)

**Configuration:** Provider selection via `but-ai.toml` or environment variables (`BUT_AI_PROVIDER`, `BUT_AI_MODEL`, `BUT_AI_API_KEY`). Provider-specific parameters (temperature, top_p) are passed through without the plugin interpreting them.

**Token counting:** Each provider adapter implements `count_tokens()` using the provider's native tokenizer or a compatible approximation. Sr. Faustina insists on accurate counts; estimates are flagged with a margin of error.

---

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Agents produce exactly two artifacts per task: an INDEX.patch (unified diff) and a COMMIT.msg (conventional commit message). No direct file writes. No side effects.

**Workflow:**
1. Agent receives task + context (codebase state, memory entries, budget allocation)
2. Agent reads relevant files via tool calls (GetProjectStatus, GetBranchChanges, GetCommitDetails)
3. Agent generates INDEX.patch as a unified diff against the current HEAD
4. Agent generates COMMIT.msg following the project's commit convention
5. Patch is validated (applies cleanly, does not exceed scope)
6. If validation passes, patch is committed with COMMIT.msg and signed

**Patch validation rules (Sr. Immaculata's requirements):**
- Patch must apply cleanly to HEAD with `git apply --check`
- Patch must not modify files outside the declared scope
- Patch must not introduce new dependencies without explicit authorization
- COMMIT.msg must reference the task identifier

---

## Requirement 4: Polyrepo PR Coordination (Forge-Agnostic)

Cross-repository coordination uses a PR comment protocol. Each PR comment is a structured message envelope with a schema header.

**Forge abstraction:**
- `ForgeAdapter` trait with implementations for GitHub, GitLab, and Bitbucket
- All forge interactions go through the adapter; no forge-specific code leaks into business logic
- PR comments use a machine-readable schema prefix (`<!-- but-ai:coordination:v1 -->`) that is invisible in rendered markdown

**Coordination protocol:**
- Agent A opens PR in repo-1 with a coordination comment linking to repo-2
- Agent B in repo-2 detects the link, reads the coordination comment, and opens a corresponding PR
- Status synchronization via polling (webhooks are forge-specific and unreliable)
- Final merge requires all coordinated PRs to be in a "ready" state

---

## Requirement 5: Agent Memory in Git Branches

Memory entries are stored as blobs in a dedicated Git ref namespace: `refs/but-ai/memory/<agent-id>/`.

**Memory structure:**
- Each entry is a JSON blob: `{ key, value, tags, created, ttl, liturgical_season }`
- The `liturgical_season` field is our unique addition: memories tagged during Lent (austerity) carry different relevance weights than memories from Ordinary Time
- Index is a tree object mapping keys to blob SHAs
- TTL-based expiration runs during `but-ai gc`, which is invoked automatically before each task

**Relevance scoring:**
- Cosine similarity between task context embedding and memory entry embedding
- Top 5 entries injected into agent context
- Hard cap prevents memory bloat in token budget

**Why liturgical seasons:** The sisters observed that code written under deadline pressure (Lent-like austerity) differs from exploratory code (Easter-like generosity). Tagging memories with the "season" of their creation provides a lightweight mood indicator that improves retrieval relevance by 12% in internal testing.

---

## Requirement 6: Signed Commits via OpenWallet

Every agent commit is signed using a key managed by an OpenWallet-compatible credential wallet.

**Key lifecycle:**
- Key provisioning: Agent receives a signing key at deployment via the OpenWallet issuance protocol
- Key rotation: Automatic rotation every 90 days; old key is retained for verification but cannot sign new commits
- Key revocation: Immediate revocation via the OpenWallet revocation endpoint; all commits signed with the revoked key are flagged
- Verification: `but-ai verify <commit>` checks the signature against the OpenWallet trust registry

**Signature format:** Standard Git commit signatures (GPG-compatible) with the OpenWallet DID embedded in the signature comment field for cross-referencing.

---

## Token Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Sr. Scholastica | 3,200 | 800 | 4,000 |
| Sr. Agnes | 7,500 | 3,500 | 11,000 |
| Sr. Immaculata | 4,000 | 1,200 | 5,200 |
| Sr. Bernadette | 5,500 | 600 | 6,100 |
| Sr. Faustina | 3,000 | 800 | 3,800 |
| Br. Marcus | 2,800 | 600 | 3,400 |
| **Team Total** | **26,000** | **7,500** | **33,500** |

Budget is per-task for a standard 200-line, 3-file feature. Complex tasks scale to 2x. Coordination overhead is included in Sr. Scholastica's allocation.

---

## Unique Insight: Cost-Per-Stitch as Token Governance

The sisters have governed thread usage for centuries using a cost-per-stitch metric: every stitch in a vestment has a calculable cost (thread length, labor time, opportunity cost of alternative garments). This metric drives every design decision. We apply the same principle to AI tokens.

Every token consumed by the `but-ai` plugin has a calculable downstream impact. Sr. Faustina maintains a live dashboard mapping token expenditure to charitable surplus. If a refactoring task costs 40,000 tokens and produces a patch that saves 200ms per build, she calculates the ROI in terms of developer-hours saved and converts that to orphanage funding potential.

This is not metaphor. It is accounting. The sisters have been doing it with thread for 877 years. Doing it with tokens is the same discipline applied to a new material.

---

*"Waste no thread. Waste no token. The children are watching."*
