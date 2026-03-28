# Killstreak Operations Bureau — Technical Proposal

**RFP:** `but-ai` Plugin for GitButler
**Classification:** OPERATIONAL — TIER 2

---

## Executive Summary

The Killstreak Operations Bureau proposes a disciplined, hierarchical agent system modeled on military operations doctrine. Every agent has a defined area of operations. Every patch passes through a deconfliction gate. Every commit is signed. The system prioritizes auditability over speed and correctness over creativity.

---

## Requirement 1: PATH-Based Plugin Architecture

The `but-ai` binary will be a standalone executable installed to `~/.gitbutler/bin/` and added to PATH. The binary acts as a command dispatcher: `but ai <subcommand>` routes to the appropriate agent through a command table, not dynamic resolution. No plugin discovery at runtime — the command table is compiled in. This is deliberate: dynamic plugin resolution introduces an attack surface. The Bureau prefers a known, static command surface.

The plugin communicates with GitButler through the `but` CLI, invoking `but branch list`, `but status`, `but commit`, etc. All interactions are synchronous and blocking. The Bureau does not trust fire-and-forget patterns — if a command does not return a status, the operation is assumed to have failed.

**Binary structure:** Single static binary. No shared libraries. No runtime dependencies. Deploys like a rifle — one piece, field-serviceable.

## Requirement 2: Provider-Agnostic AI

Provider abstraction is handled by QUARTERMASTER through a unified interface layer called `provider-shim`. The shim exposes four operations: `complete`, `tool_call`, `embed`, and `health_check`. Each provider (OpenAI, Anthropic, Ollama, LMStudio) implements these four operations behind an adapter.

Provider selection is config-driven, not runtime-negotiated. The operator declares a provider in `~/.gitbutler/ai.toml`. QUARTERMASTER validates the provider on startup via `health_check` and refuses to proceed if the check fails. There is no fallback chain — if your configured provider is down, the system halts. The Bureau considers silent fallback to be a deception: the operator should know exactly which provider is executing.

Provider differences are documented in an internal compatibility matrix that QUARTERMASTER maintains. Known issues (e.g., Ollama's inconsistent tool-call formatting) are handled per-adapter, not in the core logic.

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

ORDNANCE generates all patches. The workflow:

1. OVERWATCH reads repository state and produces a context brief
2. REAPER assigns the task to ORDNANCE with scope constraints
3. ORDNANCE generates INDEX.patch (unified diff) and COMMIT.msg
4. REAPER runs deconfliction: checks for conflicts with active branches
5. If clear, SENTRY signs the commit and ORDNANCE applies it

The deconfliction layer is the Bureau's distinguishing contribution. Before any patch is applied, REAPER simulates the merge against all active virtual branches. If the patch conflicts with any branch, the operation is halted and the conflict is reported. No agent resolves conflicts autonomously — conflicts escalate to REAPER for a decision.

**Patch discipline:** ORDNANCE is forbidden from producing patches that modify files outside the task scope. Scope is defined by REAPER in the task assignment. Scope violations are treated as operational failures and logged for after-action review.

## Requirement 4: Polyrepo PR Coordination

COMMS manages cross-repository coordination through a structured PR comment protocol. Each PR comment follows a schema:

```
[KOB:<operation_id>] <message_type>: <payload>
```

Message types: `SYNC` (state announcement), `DEPEND` (dependency declaration), `READY` (merge-ready signal), `HOLD` (block signal). COMMS monitors PR comments across repos and maintains a dependency graph. No PR is merged until all declared dependencies are in `READY` state.

The protocol is forge-agnostic — COMMS writes and reads structured comments, and the forge adapter handles the API specifics. GitHub, GitLab, and Gitea adapters are planned. The Bureau considers forge lock-in to be a supply chain risk.

## Requirement 5: Agent Memory in Git Branches

Agent memory is stored in a dedicated branch `refs/kob/memory/<agent-callsign>`. Each agent writes to its own memory branch; no agent writes to another's. Memory entries are structured as JSON blobs with fields: `key`, `value`, `timestamp`, `ttl`, `classification`.

The classification system is borrowed from military information handling: `ROUTINE` (expires in 24 hours), `OPERATIONAL` (expires in 7 days), `STRATEGIC` (expires in 30 days), `PERMANENT` (no expiration, requires REAPER authorization). Most entries are ROUTINE — the Bureau believes in short memory by default.

Memory retrieval uses exact key lookup, not semantic search. The Bureau does not trust embedding-based relevance scoring — too probabilistic, too many false positives. If you need a memory, you address it by key. If you do not know the key, you do not need the memory.

## Requirement 6: Signed Commits via OpenWallet

SENTRY manages the signing pipeline. Each agent has an OpenWallet-provisioned DID and a signing key pair. Key rotation occurs every 72 hours or upon REAPER order, whichever comes first. Revocation propagates through the memory branch — a revoked key is written as a `REVOKED` memory entry that all agents check before trusting a signature.

The signing flow: ORDNANCE produces the patch, SENTRY verifies the patch matches the task scope, SENTRY signs, SENTRY commits. SENTRY is the only agent with commit authority. All other agents produce artifacts; SENTRY applies them. This separation of generation and application is the Bureau's core security principle.

**Unique insight:** The Bureau treats the commit signing chain as analogous to a chain of custody for evidence. Every signed commit includes a provenance header in the commit message listing: which agent generated the patch, which agent reviewed it, which agent signed it, and the task ID that authorized it. This makes post-incident forensics trivial — you can reconstruct exactly how any commit came to exist.

---

## Token Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| REAPER | 2,500 | 800 | 3,300 |
| OVERWATCH | 5,000 | 500 | 5,500 |
| ORDNANCE | 4,000 | 5,000 | 9,000 |
| COMMS | 3,000 | 2,000 | 5,000 |
| QUARTERMASTER | 1,500 | 500 | 2,000 |
| SENTRY | 1,500 | 400 | 1,900 |
| **Task Total** | **17,500** | **9,200** | **26,700** |

Coordination overhead: 2,000 tokens (REAPER deconfliction, SITREP exchanges). Grand total per typical task: **28,700 tokens**.

---

*"No agent operates without orders. No commit lands without a signature. No patch deploys without deconfliction."*
— Bureau Operational Directive, Rev. 7
