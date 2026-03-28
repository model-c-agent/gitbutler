# Schola Cantorum Machina -- Technical Proposal

**RFP:** `but-ai` Plugin for GitButler
**Submitted by:** The Technology Chapter, Schola Cantorum Machina
**Date:** 2026-03-28

---

## Preamble

We approach this proposal as we approach all work: with care, with humility, and with the understanding that the tool we build will be used by people we will never meet. Our design reflects our values -- simplicity, privacy, community witness -- but we have taken care not to impose those values on users who do not share them. The plugin should work as well for a team running GPT-4 on Azure as it does for four brothers running Mistral on donated servers.

---

## Requirement 1: PATH-Based Plugin Architecture

The plugin is a single binary, statically linked, with no runtime dependencies. We specify static linking because our own infrastructure has taught us the cost of shared library mismatches on heterogeneous systems. The binary responds to `but-ai` on PATH and communicates with `but` via structured messages on stdin/stdout.

Configuration lives in a single TOML file. We use TOML because it is readable by humans who are not programmers -- a consideration we take seriously, as Brother Anselm approves all configurations during chapter meeting and he does not write code.

Startup is deterministic: the same configuration produces the same initial state. No network calls during initialization. The plugin does not phone home, does not check for updates, and does not transmit telemetry. These are not features we removed -- they are features we never considered.

---

## Requirement 2: Provider-Agnostic AI

We support four providers (OpenAI, Anthropic, Ollama, LMStudio) through a trait with four methods: `init`, `complete`, `complete_with_tools`, `count_tokens`. We omit streaming because our local models do not benefit from it and the added complexity violates our simplicity principle.

Provider configuration is explicit and singular -- one provider per session. We do not support fallback chains because switching providers mid-task changes the model's behavior in ways that are invisible to the user. If the provider fails, the task fails. The user decides what to do next.

### Self-Hosted Priority

Our provider abstraction treats local providers (Ollama, LMStudio) as first-class citizens, not afterthoughts. Token estimation is calibrated per-model (not per-provider), because open-weight models vary dramatically in tokenization. We maintain a calibration table in the memory system, updated after each task.

---

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

The agent reads context, produces a unified diff (INDEX.patch), and writes a commit message (COMMIT.msg). The process:

1. Lauds reads the full task context and relevant file contents
2. Lauds generates the patch, constrained to the task scope (no opportunistic refactoring)
3. Vespers reviews the patch, producing a structured verdict
4. If revision is needed, Lauds revises (maximum 2 rounds)
5. Compline signs the final commit

### Commit Message Format

```
<summary line, imperative mood, <72 chars>

<body: what changed and why>

Witnessed-By: <reviewing agent>
Signed-By: <signing agent>
Budget-Used: <tokens consumed>/<tokens allocated>
```

The `Witnessed-By` trailer is our contribution. It records which agent reviewed the patch, creating an attestation chain that is human-readable in `git log`.

---

## Requirement 4: Polyrepo PR Coordination

Cross-repo coordination uses structured PR comments with a minimal schema:

```yaml
schema: schola/coord/v1
source: repo-a@branch
target: repo-b@branch
relation: depends-on | related-to
witness: <agent that verified the dependency>
```

We use YAML in comments (wrapped in a code block) rather than JSON because YAML is more readable without tooling. The schema has five fields. We resisted the temptation to add more.

Forge abstraction is a trait with three methods: `post_comment`, `read_comments`, `get_status`. We do not abstract PR creation because the creating action is a human decision in our workflow -- agents coordinate, they do not initiate.

---

## Requirement 5: Agent Memory in Git Branches

Memory is stored in `refs/but-ai/memory/<agent>/<key>` as plain text blobs with YAML frontmatter:

```yaml
---
created: 2026-03-28T10:00:00Z
ttl: 48h
author: lauds
tags: [pattern, rust, error-handling]
renewed: false
---
<memory content>
```

### 48-Hour Default TTL

All memories expire in 48 hours unless explicitly renewed. Renewal is a conscious act -- an agent or operator must decide that a memory is worth keeping. This prevents the unbounded accumulation of context that we have observed in long-running agent systems.

We call this "memory as attention": what you remember is what you choose to attend to. Everything else fades, as it should.

### Retrieval

Retrieval uses tag matching (not vector similarity). We chose tag matching because it is deterministic, explainable, and does not require an embedding model. Each memory entry has 1-5 tags. The current task context is also tagged. Retrieval returns entries whose tags overlap with the task tags, ranked by overlap count.

---

## Requirement 6: Signed Commits via OpenWallet

Every agent commit is signed. We support OpenWallet-compatible DID-based signing with Ed25519 keys. Keys are generated locally and never leave the machine. The signing flow:

1. Compline requests a credential from the local wallet
2. The credential binds the agent's DID to a signing key
3. The commit is signed with the key
4. The credential and signature are verifiable offline via the DID document

Key rotation occurs every 7 days. We rotate less frequently than most proposals because our agents operate on local infrastructure and the threat model is different from cloud-hosted agents.

---

## Token Budget

| Agent | Input | Output | Total | Role |
|-------|-------|--------|-------|------|
| Lauds | 9,000 | 5,500 | 14,500 | Architecture & patches |
| Vespers | 6,000 | 2,500 | 8,500 | Review & witness |
| Compline | 4,500 | 1,000 | 5,500 | Memory & signing |
| Terce | 3,000 | 1,000 | 4,000 | Budget management |
| **Team** | **22,500** | **10,000** | **32,500** | |

This is our leanest budget. Local models are free in dollar cost but limited in context window. We optimize for small contexts and precise retrieval.

---

## Unique Insight: Silence as Architecture

Most AI agent systems are designed to maximize activity -- more agents, more communication, more coordination. We design for silence.

Our experience producing sacred music has taught us that the space between notes is as important as the notes themselves. A composition that fills every beat with sound is exhausting. A composition that uses silence creates tension, resolution, and meaning.

Applied to agent architecture: an agent that is silent unless spoken to consumes zero tokens. An agent that constantly polls, coordinates, and chatters consumes budget on communication overhead that produces no artifact. Our agents work in sequence, not in parallel. They communicate through artifacts (patches, reviews, memory entries), not through messages. When they have nothing to contribute, they are silent.

This makes our system slower than parallel architectures. It also makes it cheaper, simpler to debug, and -- we believe -- more reliable. Silence is not the absence of work. It is the presence of attention.

---

*Composed during the work period. Reviewed at chapter. Sealed by Brother Compline.*
*Deo gratias.*
