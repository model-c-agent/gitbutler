# Ristorante Ferrara dal 1881 Proposal — `but-ai` Plugin

**Submitted by:** Ristorante Ferrara dal 1881
**Date:** 2026-03-28

---

## Guiding Principle

A recipe is a commit message from the past. Our proposal treats every codebase as a family inheritance: something received, maintained, and passed on. The agent's job is stewardship, not disruption.

---

## 3.1 Plugin Architecture

`but-ai` as a Rust binary on PATH. CLI mode: `but ai cook` (execute task), `but ai consult` (query the ancestor archive), `but ai taste` (dry-run a patch without committing). MCP mode via `but ai mcp` with `ServerHandler` and all ten workspace tools.

WASI degradation: The Concetta agent (read-only archive) works fully in WASI. Write agents degrade to advisory mode. This mirrors the family's principle: you can always consult the recipe book; you cannot always cook.

Config: Git config under `[but-ai]`. Environment variables: `BUT_WORKSPACE_DIR`, `BUT_OUTPUT_FORMAT`, `BUT_JSON`.

---

## 3.2 Provider-Agnostic AI Interface

`but-llm` exclusively. Provider via `gitbutler.aiModelProvider`. The Ferrara family adds a "trust but verify" layer: every provider response is hashed and logged. If a provider returns inconsistent responses for identical inputs (detected via hash comparison), the system alerts Marco.

New providers: Git config entry + signed shared library adapter.

**Trade-off:** Hash logging adds storage overhead. Accepted because the family values auditability above efficiency.

---

## 3.3 The But Agent

Family-structured execution:

1. **Consult Concetta:** Query the ancestor archive for canonical patterns relevant to the task
2. **Marco drafts:** Produce INDEX.patch + COMMIT.msg informed by archive and current context
3. **Rosa reviews:** Compare patch against canonical standards. Approve, reject, or request revision.
4. **Elena coordinates:** If multi-repo, handle cross-kitchen PR communication.

INDEX.patch + COMMIT.msg only. No direct writes. Branch naming: `ferrara/<kitchen>/<dish>/<task-id>` — encoding location, feature, and task.

Budget enforcement: Marco tracks consumption. At 80% budget, he produces current best output. COMMIT.msg includes a generation marker (`G1`-`G5`) indicating the knowledge source.

**Unique aspect:** The Concetta agent creates a "canonical context" injected into every task — a summary of the project's foundational patterns. This costs tokens but reduces drift.

---

## 3.4 Polyrepo PR-Based Coordination

Elena manages cross-kitchen (cross-repo) coordination. PR comments follow a family letter format:

```json
{
  "ferrara_schema": "1.0",
  "type": "recipe-update|deviation-alert|consistency-check|dependency",
  "kitchen": "catania|palermo",
  "from": "Elena",
  "generation": "G5",
  "body": "...",
  "consistency_score": 0.985
}
```

The `consistency_score` field is unique: it measures how closely the two kitchens' branches align. A score below 0.95 triggers an automatic reconciliation task.

Forge adapter: `ForgeAdapter` trait with `send_letter` (PR), `append_note` (comment), `read_correspondence` (list), `reconcile` (merge). GitHub reference implementation.

Cross-repo: PR descriptions declare `depends-on: org/repo#N`. Elena tracks all open dependencies.

---

## 3.5 Agent Memory and Identity

Memory stored in `refs/ferrara/memory/<generation>/<topic>`. The generation dimension is the Ferrara family's unique contribution: memories are tagged by the generation of knowledge they encode.

**Generation taxonomy:**
- `G1`: Concetta's era — foundational patterns, never expired
- `G2`-`G3`: Mid-family wisdom — long TTL (365 days)
- `G4`: Recent adaptation — standard TTL (90 days)
- `G5`: Current — short TTL (30 days)

**Relevance scoring:** Generation-weighted TF-IDF. G1 entries receive a 2x relevance boost when the task involves core patterns. G5 entries receive a 1.5x boost when the task involves recent features. Maximum 5 entries per retrieval.

**Compaction survival:** G1 always survives. G2-G3 survive if relevance > 0.5. G4-G5 follow standard TTL rules.

**The Ancestor Agent pattern:** Concetta is a read-only agent whose entire state is the G1 memory branch. She has no output budget — she only provides context. This is a novel architecture: an agent that exists solely as a memory source, never as an actor.

**Identity:** At `refs/ferrara/identity/<agent>`:
```json
{
  "name": "Marco",
  "family_role": "technologist",
  "generation": "G5",
  "capabilities": ["patch-generation", "provider-management"],
  "signing_key_id": "ferrara:marco:2026"
}
```

---

## 3.6 Signed Commits via OpenWallet

All commits signed. Keys provisioned per agent. The Concetta agent has no signing key — she cannot commit.

**Authorization:**
```json
{
  "Marco": { "allow": ["ferrara/*"], "deny": ["main", "refs/ferrara/memory/G1/*"] },
  "Rosa": { "allow": ["main", "ferrara/canonical/*"], "deny": [] },
  "Elena": { "allow": ["ferrara/coordination/*"], "deny": ["ferrara/*/implementation"] }
}
```

G1 memory is immutable — no agent can modify it. This is the digital equivalent of the recipe book in the bank vault.

**Key lifecycle:**
- Provisioning: at agent creation
- Rotation: annual, timed to the Feast of Sant'Agata (the family's calendar anchor)
- Compromise: immediate revocation. All post-compromise commits reviewed by Rosa personally.

---

## 3.7 Token Budget

| Component | Input | Output | Frequency | Notes |
|-----------|-------|--------|-----------|-------|
| System prompt | 2,600 | 0 | Once/session | Identity, tools, canonical context |
| Concetta consultation | 2,000 | 0 | Once/task | Ancestor archive query (input only) |
| Task ingestion | 1,800 | 400 | Once/task | Read task description |
| Planning | 1,200 | 600 | Once/task | Decompose, select approach |
| Tool call (per call) | 1,000 | 500 | 4/task avg | Branch ops, status checks |
| Patch generation | 3,000 | 4,000 | Once/task | INDEX.patch |
| Commit message | 500 | 400 | Once/task | COMMIT.msg with generation marker |
| Memory retrieval | 1,500 | 200 | 2/task avg | Generation-weighted lookup |
| Coordination event | 1,800 | 700 | 1/task avg | Cross-kitchen letter |
| **TOTAL (typical)** | **19,400** | **8,800** | -- | 200-line feature, 3 files, 2 deps |

---

## Testing Strategy

1. **Provider consistency:** Record provider responses, hash, replay. Verify determinism.
2. **Patch round-trip:** 30-fixture suite with canonical style validation.
3. **Consistency scoring:** Two simulated kitchens diverge by known amounts. Verify score accuracy.
4. **Ancestor agent:** Query Concetta with in-scope and out-of-scope questions. Verify correct response and `ARCHIVE_SILENT` fallback.
5. **Budget:** Verify 80% threshold triggers partial output with generation marker.

---

## Git Config Keys

| Key | Default | Purpose |
|-----|---------|---------|
| `but-ai.agent.tokenBudget` | 45000 | Per-task ceiling |
| `but-ai.agent.memoryRef` | `refs/ferrara/memory` | Memory prefix |
| `but-ai.agent.consistencyThreshold` | 0.95 | Reconciliation trigger |
| `but-ai.forge` | `github` | Forge adapter |
| `but-ai.agent.canonicalBranch` | `ferrara/canonical` | Concetta's archive branch |
| `but-ai.agent.g1Immutable` | true | Protect G1 memory from writes |

---

*"The recipe book does not argue. It remembers."*
