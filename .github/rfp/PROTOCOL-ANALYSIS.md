# How but-ai Relates to the GitButler Protocol

## Executive Summary

The "GitButler Protocol" is an emergent multi-layer design for AI agents working with version control. It crystallized through practical experience (28 sub-PRs, 6 documented failures), a 100-agent analysis, and the RFP process (200 proposals, 5 implementations, 1 unified crate). The but-ai unified crate (9,508 lines of Rust) is the first complete implementation, covering 5 of the protocol's 7 layers.

## The Protocol's Origin Story

- **PR #1 (WASI compilation):** 16 sub-PRs revealed filesystem contention, null commit IDs, and sync race conditions.
- **6 documented failures (F1-F6):** Each spawned a protocol rule.
- **100-agent analysis:** Across 10 specializations (DevOps, security, Rust, UX, testing, architecture, performance, workflow, AI, operations).
- **RFP process:** 200 organizations proposed designs, 5 built implementations, cross-evaluation produced consensus.

## The 7 Protocol Layers

### Layer 1: Write Primitive

- Agents produce INDEX.patch + COMMIT.msg.
- Never modify the working tree directly.
- Sole committer model prevents race conditions.
- **Origin:** F2 (hunk lock contention), F4 (background sync race).

### Layer 2: CLI Contract

- `but` CLI is the single write interface.
- `--status-after` on all mutations.
- Never use git directly for writes.
- **Origin:** SKILL.md, virtual branch model integrity.

### Layer 3: Agent-Native Contract

- Exit codes are semantic (no null-on-success).
- Batch operations are primitive.
- State is observable without polling.
- Mutations return immediate feedback.
- **Origin:** F3 (null commit IDs), 100-agent analysis.

### Layer 4: Coordination

- PR comments with structured JSON in code fences.
- Forge-agnostic (GitHub/GitLab/Bitbucket/Gitea).
- Dependency DAG with topological sort.
- CRDT gossip for distributed memory sync.
- **but-ai implementation:** `coordination/` module (760 lines).
- **Key types:** CoordinationMessage, ForgeAdapter, VectorClock, DependencyGraph.

### Layer 5: Memory & Knowledge

- Git refs for storage (`refs/but-ai/memory/...`).
- Unified MemoryEntry with 5 integrated models.
- **Classification:** 5 systems (subject, call number, source, temporal, relational).
- **Survival:** 4 distributions fitted from access patterns.
- **Three-state lifecycle:** Alive (S(t) >= 0.25) -> Moribund -> Deceased (S(t) < 0.10).
- **Retrieval:** 6-component scoring formula.
- **Narrative:** Motifs (3+ appearances), tensions (Weibull urgency), arcs.
- **Cross-references:** Bidirectional see-also graph.
- **but-ai implementation:** `memory/` (1,280), `survival/` (480), `narrative/` (640 lines).

### Layer 6: Identity & Authorization

- 4 agent roles: Architect, Implementer, Validator, Coordinator.
- Glob-based authorization (branch patterns, call number ranges).
- Cryptographic commit signing (OpenWallet).
- Key lifecycle with audit log.
- Performance tracking.
- **but-ai implementation:** `identity/` module (390 lines).

### Layer 7: Task Orchestration

- 6 phases: Classify, Plan, Implement, Validate, Catalog, Coordinate.
- Phase-gated tool loading (least privilege).
- Token budget with mandatory reserves.
- 4 budget modes with graceful degradation.
- Catalog + Coordinate always execute (reserved tokens).
- **but-ai implementation:** `agent/` module (1,040 lines).

## What but-ai Adds Beyond the Existing Protocol

1. **Survival-function-based memory expiration** (vs. fixed TTL).
2. **6-dimensional retrieval scoring** (vs. keyword search).
3. **Motif emergence** (pattern crystallization at 3+ appearances).
4. **Tension tracking** with escalating urgency.
5. **CRDT gossip** for distributed memory sync.
6. **Phase-gated least privilege** for tool loading.

## The Protocol Gap

Layers 1-3 (Write Primitive, CLI Contract, Agent Contract) are about the `but` CLI itself -- they are specified in SKILL.md and the agent-native epic but only partially implemented. Layers 4-7 are about the `but-ai` plugin -- fully implemented in the unified crate. The protocol spans both the CLI and the plugin.

## Key Design Principles

1. **Patch-first:** Agents produce artifacts, not mutations.
2. **Memory-native:** Knowledge persists in Git, not external databases.
3. **Forge-agnostic:** Protocol works with any forge (GitHub, GitLab, etc.).
4. **Budget-aware:** Every operation has a token cost; graceful degradation is mandatory.
5. **Failure-driven:** Every protocol rule traces to a documented incident.
6. **Convergence-guaranteed:** CRDT merge semantics ensure distributed consistency.

## Reference Implementation

The but-ai unified crate at `crates/but-ai/` (branch `rfp/unified`) is the reference implementation:

- 9,508 lines of Rust across 43 source files.
- 7 modules: types, memory, survival, narrative, coordination, agent, identity, validation.
- Compiles with `cargo check -p but-ai`.
- Built by 5 teams from the top-ranked RFP organizations.
