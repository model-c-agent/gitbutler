# Spine & Gilt — Technical Proposal

**RFP:** GitButler `but-ai` Plugin
**Submitted:** 2026-03-28
**Contact:** Margaux (Catalog Architect)

---

## Executive Summary

We are book artists who run a 20,000-volume lending library. Our proposal treats the `but-ai` plugin as a cataloging system: every commit is a catalog record, every branch is a shelf, every agent is a librarian with a specialty. We bring eight years of managing physical metadata at scale, translated into Git-native workflows.

---

## Requirement 1: PATH-Based Plugin Architecture

### Approach

The `but-ai` binary lives on PATH as a standalone executable. Discovery follows the `git-credential-helper` pattern: `but` searches PATH for `but-ai`, invokes it with a subcommand, and communicates via stdin/stdout using structured JSON lines.

### Design

- Plugin binary: `but-ai` (Rust, statically linked)
- Communication: JSON-over-stdio, one message per line
- Discovery: PATH search with fallback to `$BUT_AI_PATH` override
- Lifecycle: `but` spawns `but-ai` per task; no persistent daemon
- Configuration: `~/.config/but/ai.toml` for provider and budget settings

We avoid daemon models because they introduce state management complexity that PATH-based invocation avoids entirely. A library does not keep a librarian standing at each shelf -- it dispatches them when a patron asks.

---

## Requirement 2: Provider-Agnostic AI

### Approach

A thin adapter layer that normalizes provider differences behind a single interface. Each provider is a module conforming to a `Provider` trait with methods for `complete`, `tool_call`, and `stream`.

### Supported Providers

| Provider | Adapter | Notes |
|----------|---------|-------|
| Anthropic | HTTP REST | Primary development target |
| OpenAI | HTTP REST | Compatible tool-calling schema |
| Ollama | HTTP REST (local) | Reduced tool-calling fidelity |
| LMStudio | OpenAI-compatible | Local, no auth required |

### Provider Selection

Provider is set in `ai.toml`. No runtime switching. We have seen too many "smart" provider routers that introduce latency and unpredictability. The user chooses; the system respects.

---

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

### Approach

The agent reads workspace state, produces a unified diff as `INDEX.patch`, and writes a `COMMIT.msg` describing the change. These two files are the sole output. The agent never writes directly to the working tree.

### Workflow

1. Agent receives task description and workspace context
2. Agent calls `GetProjectStatus` to read current state
3. Agent produces `INDEX.patch` (unified diff format)
4. Agent produces `COMMIT.msg` (conventional commits format, with catalog-style metadata)
5. `but` validates the patch applies cleanly
6. `but` commits with the provided message

### Catalog Metadata in Commits

Every COMMIT.msg includes a `Catalog:` trailer with structured metadata -- the change's "call number" in our system:

```
feat: add provider timeout configuration

Catalog: section=provider scope=config confidence=high
Reviewed-by: Margaux (Catalog Architect)
```

This is our unique contribution. Just as every book in our library carries a call number that places it in context, every commit carries metadata that places it in the project's conceptual structure.

---

## Requirement 4: Polyrepo PR Coordination

### Approach

Forge-agnostic PR coordination using a structured comment schema. Each PR comment is a message envelope with a type, source agent, and payload. The forge adapter translates between the schema and the forge's API (GitHub, GitLab, Bitbucket, Forgejo).

### Comment Schema

```json
{
  "type": "coordination",
  "source": "lerato@spine-and-gilt",
  "target_repo": "org/other-repo",
  "target_pr": 42,
  "payload": {
    "action": "dependency_ready",
    "branch": "feat/provider-config",
    "commit": "abc1234"
  }
}
```

### Cross-Repo Workflow

- Agent detects cross-repo dependency via branch naming convention
- Lerato opens or comments on the target PR with a coordination message
- Target repo's agent parses the comment and proceeds
- No shared database, no external service -- Git and forge comments are the only substrate

---

## Requirement 5: Agent Memory in Git Branches

### Approach

Memory stored as blobs in Git refs under `refs/but-ai/memory/<namespace>/<key>`. The memory index is a Git tree object. Expiration is garbage collection of unreferenced objects.

### Memory Scheme: The Card Catalog

Inspired by our physical card catalog, each memory entry is a "card" with fields:

| Field | Description |
|-------|-------------|
| `key` | Unique identifier (analogous to call number) |
| `value` | The memory content |
| `section` | Namespace grouping (e.g., `patterns`, `decisions`, `errors`) |
| `condition` | Reliability indicator: `mint`, `good`, `foxed`, `brittle` |
| `acquired` | Timestamp of creation |
| `ttl` | Time-to-live in hours (default: 168 for decisions, 24 for errors) |
| `provenance` | Which agent created it and why |

Condition degrades over time. A `mint` entry becomes `good` after 7 days, `foxed` after 30, `brittle` after 90. Brittle entries are excluded from retrieval unless explicitly requested. This prevents stale knowledge from contaminating fresh tasks.

### Retrieval

Top-5 entries by relevance score, weighted by condition. No entry with condition `brittle` is injected automatically.

---

## Requirement 6: Signed Commits via OpenWallet

### Approach

Each agent holds a signing key managed through OpenWallet's Verifiable Credential model. Keys are provisioned at agent startup, rotated every 72 hours, and revoked immediately on compromise.

### Key Lifecycle

1. **Provision:** `but-ai` requests a signing key from OpenWallet at task start
2. **Use:** Every commit is signed with the agent's current key
3. **Rotate:** Keys rotate on schedule; old keys remain valid for verification
4. **Revoke:** Compromised keys are added to a revocation list stored in `refs/but-ai/revoked`

### Verification

Any commit can be verified against the agent's public key, which is stored in a Verifiable Credential anchored to the repository. The credential includes the agent's identity, role, and the time window during which the key was valid.

---

## Token Budget

| Agent | Role | Input | Output | Total |
|-------|------|-------|--------|-------|
| Margaux | Patch generation | 8,200 | 3,800 | 12,000 |
| Tomas | Memory management | 5,500 | 1,200 | 6,700 |
| Fen | Provider/budget | 4,000 | 1,000 | 5,000 |
| Lerato | Forge coordination | 5,800 | 2,500 | 8,300 |
| Suki | Signing/identity | 3,200 | 800 | 4,000 |
| **Total** | | **26,700** | **9,300** | **36,000** |

Budget is per-task for a typical 3-file, 200-line change. Complex tasks scale to 2x. Coordination overhead is included in Lerato's allocation.

---

## Unique Insight: Condition-Degrading Memory

Most agent memory systems treat entries as either valid or expired. We introduce a **condition gradient** -- memory entries degrade over time like physical materials, passing through stages of reliability before expiration. This prevents the binary failure mode where a memory is fully trusted one second and gone the next. Instead, agents receive progressively weaker signals from aging memories, and can choose to verify or discard based on the condition rating.

This is how librarians actually work. A catalog card from 1970 is not wrong -- but its authority is different from one written yesterday. The information may be accurate, but the confidence interval has widened.

---

*"Every book we lend comes back with a story. Every commit should too."*
