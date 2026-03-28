# Proposal: `but-ai` Plugin — Tidal Protocol Collective

**RFP Response — Version 1.0**
**Date:** 2026-03-28 (high tide, Rotterdam, +0.2m)
**Organization:** Tidal Protocol Collective (001)
**Contact:** collective@tidalprotocol.coop

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [Plugin Architecture (RFP 3.1)](#2-plugin-architecture)
3. [Provider-Agnostic AI Interface (RFP 3.2)](#3-provider-agnostic-ai-interface)
4. [The But Agent (RFP 3.3)](#4-the-but-agent)
5. [Polyrepo PR-Based Coordination (RFP 3.4)](#5-polyrepo-pr-based-coordination)
6. [Agent Memory & Identity (RFP 3.5)](#6-agent-memory--identity)
7. [Signed Commits via OpenWallet (RFP 3.6)](#7-signed-commits-via-openwallet)
8. [Token Budget (RFP 3.7)](#8-token-budget)
9. [Testing Strategy](#9-testing-strategy)
10. [Trade-offs and Alternatives](#10-trade-offs-and-alternatives)
11. [Migration Path](#11-migration-path)
12. [Git Config Keys](#12-git-config-keys)

---

## 1. Executive Summary

We propose a `but-ai` plugin built as a Rust crate within the existing workspace, operating as a peer-to-peer agent system where no single agent has more authority than any other. The core insight: coordination is a protocol problem, not a hierarchy problem.

Our design is anchored by three principles:

1. **Consensus over command.** Agents negotiate through structured PR comments, not through a central orchestrator. No agent can unilaterally commit — patches require consensus validation.
2. **The manifest is the memory.** Agent memory is stored as a distributed manifest in Git refs, synchronized using a CRDT-based gossip protocol. Any agent can read any other agent's memory, but no agent can modify another's.
3. **The tide is the clock.** All coordination uses a fixed 6-hour cycle (a "tide"). Decisions that cannot reach consensus within one tide are deferred. This prevents infinite negotiation loops.

---

## 2. Plugin Architecture

### 2.1 Approach

`but-ai` is a single Rust binary, compiled as `but-ai` and placed on PATH. It is structured as a new crate (`crates/but-ai`) within the GitButler workspace, depending on `but-llm`, `but-tools`, `but-ctx`, and `but-action`.

### 2.2 Design

#### Binary Structure

```
but-ai
├── cli mode:     but ai <subcommand>
│   ├── but ai agent <task>     — Run autonomous agent
│   ├── but ai chat <prompt>    — Interactive LLM chat with tools
│   ├── but ai memory <cmd>     — Query/manage agent memory
│   ├── but ai identity <cmd>   — Manage agent identity and keys
│   └── but ai status           — Show agent state and budget
│
└── mcp mode:     but ai mcp
    └── stdio MCP server (rmcp ServerHandler)
```

#### Environment Contract

The plugin reads `BUT_WORKSPACE_DIR`, `BUT_OUTPUT_FORMAT`, and `BUT_JSON` from the environment as specified in the RFP. It constructs a `Context` via `Context::new_from_legacy_project_and_settings()` using `BUT_WORKSPACE_DIR` as the project root.

#### WASI Degradation

When running under WASI (where `find_external_subcommand()` is disabled), the plugin cannot be discovered as a CLI subcommand. We propose two degradation strategies:

1. **Library mode.** The core logic of `but-ai` is factored into a library crate (`but-ai-core`) that can be compiled to `wasm32-wasip2` and linked directly into the `but` binary when the `wasi` feature is enabled. This provides a subset of functionality (no MCP server, no fork/exec for providers) but allows basic agent operations.
2. **Remote mode.** A WASI build can connect to a `but-ai` MCP server running on the host via a socket. The WASI binary acts as a thin client, forwarding commands to the host-side plugin.

### 2.3 Trade-offs

**Considered:** Implementing `but-ai` as a Python or Node.js script for faster iteration.
**Rejected:** The RFP strongly prefers Rust for consistency, and the plugin needs to call into `but-llm` and `but-tools` directly. A non-Rust plugin would need to duplicate or shell out to these crates, adding latency and complexity.

**Considered:** Shipping `but-ai` as a separate binary outside the workspace.
**Rejected:** Keeping it in-workspace (`crates/but-ai`) ensures it participates in the same CI, testing, and versioning pipeline as the rest of the codebase.

---

## 3. Provider-Agnostic AI Interface

### 3.1 Approach

We use the existing `but-llm` crate as the sole LLM backend. The `but-ai` plugin does not introduce a new LLM client. All four existing providers (OpenAI, Anthropic, Ollama, LMStudio) are supported without modification to `but-llm`.

### 3.2 Design

#### Provider Resolution

```
1. Read `gitbutler.aiModelProvider` from Git config
2. Construct LLMProvider via LLMProvider::from_git_config()
3. If no provider configured → structured error:
   { "error": { "code": "NO_PROVIDER", "message": "No AI model provider configured. Set gitbutler.aiModelProvider in Git config." } }
4. If provider configured but unreachable → structured error with provider name and endpoint
```

#### Tool Registration

All 10 workspace tools from `WorkspaceToolset` are registered automatically when the agent starts. The registration flow:

```rust
let mut toolset = WorkspaceToolset::new();
// All 10 tools registered via toolset internals
// Commit, CreateBranch, Amend, SquashCommits, GetProjectStatus,
// MoveFileChanges, GetCommitDetails, GetBranchChanges, SplitBranch, SplitCommit

// Pass toolset to LLM provider for tool-calling loop
provider.tool_calling_loop(system_prompt, messages, &mut toolset, model)?;
```

#### Plugin Provider Mechanism

New providers (Gemini, Mistral, local GGUF) are added without recompiling `but-ai` via a **provider shim** protocol:

1. A provider shim is a standalone executable named `but-ai-provider-<name>` on PATH.
2. The shim communicates with `but-ai` over stdio using a simple JSON-RPC protocol.
3. `but-ai` discovers shims at startup by scanning PATH for `but-ai-provider-*`.
4. Each shim implements three methods: `capabilities`, `chat`, and `stream`.
5. The shim translates between `but-ai`'s provider-agnostic message format and the provider's native API.

This mirrors the plugin discovery pattern of `but` itself — PATH-based, no registry, no compilation.

### 3.3 Trade-offs

**Considered:** Adding new providers directly to `but-llm`.
**Rejected:** The RFP explicitly forbids modifying existing crates. Provider shims keep `but-llm` untouched.

**Considered:** A dynamic library (`.so`/`.dylib`) plugin system for providers.
**Rejected:** Dynamic linking is fragile across platforms and impossible under WASI. PATH-based executables are universally portable.

---

## 4. The But Agent

### 4.1 Approach

The But Agent is a peer in the collective, not a supervisor. It reads a task, decomposes it into steps, executes each step using workspace tools, and produces INDEX.patch + COMMIT.msg as its sole output. It never writes files directly, never calls `git commit`, and never calls `but commit` — it produces patches that the `but` orchestrator applies.

### 4.2 Design

#### Agent Loop

```
1. INGEST: Read task from CLI arg, PR body, or branch metadata
2. RECALL: Query memory for relevant context (max 5 entries)
3. PLAN: Decompose task into steps using structured_output
4. EXECUTE: For each step:
   a. Select tool from WorkspaceToolset
   b. Call tool via tool_calling_loop
   c. Record result
   d. Check token budget — if within 10% of limit, go to step 5
5. GENERATE: Produce INDEX.patch from accumulated changes
6. MESSAGE: Produce COMMIT.msg summarizing changes
7. SIGN: Submit patch+message to signing workflow (see Section 7)
8. REPORT: Output structured progress report
```

#### Task Sources

| Source | Read Method |
|--------|-------------|
| CLI argument | `but ai agent "implement feature X"` |
| PR body | Parse PR description via forge adapter |
| Branch metadata | Read `refs/but-ai/tasks/<branch>` |
| Issue description | Parse issue body via forge adapter |

#### Branch Naming

We extend the current `s01.s04` convention with agent identity:

```
<agent-id>/<task-id>[.<dependency>]

Examples:
  dara/t001          — Dara's task 001, no dependencies
  ines/t002.dara-t001 — Ines's task 002, depends on Dara's task 001
  koel/t003.ines-t002.dara-t001 — Koel's task 003, depends on both
```

This encoding allows any observer to reconstruct the full dependency graph from branch names alone.

#### Token Budget Enforcement

The agent tracks token usage at every LLM call. The tracking structure:

```json
{
  "budget": {
    "total": 50000,
    "used": {
      "input": 32000,
      "output": 8000
    },
    "remaining": 10000,
    "phase": "execute",
    "checkpoint": "step_3_of_5"
  }
}
```

When remaining tokens drop below 10% of total, the agent enters **graceful degradation**:
1. Skip remaining execution steps
2. Generate INDEX.patch from work completed so far
3. Write COMMIT.msg with prefix `PARTIAL:` and a summary of what was completed and what was skipped
4. Exit with code 0 (partial success) and structured output indicating partial completion

### 4.3 Trade-offs

**Considered:** A hierarchical agent model where a "planner" agent decomposes tasks and "worker" agents execute them.
**Rejected:** This introduces a single point of failure at the planner. If the planner hallucinates a bad decomposition, all workers execute garbage. Our peer model allows any agent to challenge a plan.

**Considered:** Direct file editing instead of INDEX.patch for performance.
**Rejected:** The RFP mandates the patch workflow. We agree with this mandate — patch-based output prevents filesystem contention and enables atomic review.

---

## 5. Polyrepo PR-Based Coordination

### 5.1 Approach

PRs are datagrams. Comments are structured messages. The forge is the network layer. We define a forge adapter trait and a PR comment schema that works across GitHub, GitLab, Bitbucket, and Gitea.

### 5.2 Design

#### Forge Adapter Trait

```rust
trait ForgeAdapter: Send + Sync {
    fn create_pr(&self, repo: &RepoRef, title: &str, body: &str, head: &str, base: &str) -> Result<PrRef>;
    fn comment(&self, pr: &PrRef, body: &str) -> Result<CommentRef>;
    fn list_comments(&self, pr: &PrRef) -> Result<Vec<Comment>>;
    fn add_label(&self, pr: &PrRef, label: &str) -> Result<()>;
    fn remove_label(&self, pr: &PrRef, label: &str) -> Result<()>;
    fn list_labels(&self, pr: &PrRef) -> Result<Vec<String>>;
    fn get_pr_body(&self, pr: &PrRef) -> Result<String>;
    fn pr_status(&self, pr: &PrRef) -> Result<PrStatus>;
    fn list_prs(&self, repo: &RepoRef, labels: &[&str]) -> Result<Vec<PrRef>>;
    fn cross_reference(&self, from: &PrRef, to: &PrRef) -> Result<()>;
    fn resolve_ref(&self, reference: &str) -> Result<PrRef>;
    fn forge_type(&self) -> ForgeType;
}
```

The trait has 12 methods — minimal for cross-repo coordination, sufficient for the full workflow.

#### PR Comment Schema

Every agent-to-agent message is a PR comment with a structured header:

```markdown
<!-- but-ai:message -->
<!-- type: task-assignment | status-report | dependency-declaration | patch-handoff | budget-report -->
<!-- from: agent-id -->
<!-- to: agent-id | broadcast -->
<!-- timestamp: ISO-8601 -->
<!-- tide: high | low | rising | falling -->

## [STATUS-REPORT] Authentication module refactor

**Status:** completed
**Agent:** dara
**Tokens used:** 12,400 / 50,000
**Patch:** INDEX.patch attached below

### Summary
Refactored the authentication module to use the new provider interface.
3 files changed, 142 insertions, 87 deletions.

### Dependencies
- Depends on: org/repo#45 (ines/t002)
- Blocks: org/other-repo#12 (koel/t003)
```

The HTML comments are machine-parseable. The Markdown is human-readable. Forges that strip HTML comments (none of the major ones do, but for safety) get a degraded but functional version.

#### Cross-Repo Coordination

Cross-repo references use a universal reference format:

```
<forge>:<owner>/<repo>#<pr-number>

Examples:
  github:gitbutler/but#123
  gitlab:company/backend#45
  gitea:selfhosted.example.com:org/repo#7
```

The agent resolves these references through the forge adapter. If a reference points to a forge the agent does not have an adapter for, it logs a warning and continues without the cross-repo context.

### 5.3 Trade-offs

**Considered:** Using Git notes instead of PR comments for inter-agent communication.
**Rejected:** Git notes are not visible on forge UIs, making human review impossible. PRs are the universal code review interface.

**Considered:** A richer PR comment schema with full JSON payloads.
**Rejected:** JSON in PR comments is not human-readable. Our hybrid approach (HTML comments for machines, Markdown for humans) serves both audiences.

---

## 6. Agent Memory & Identity

### 6.1 Approach: The Distributed Manifest

Our memory architecture is inspired by shipping manifests. A manifest is a document that travels with cargo, listing contents, origin, destination, and handling instructions. Every port that touches the cargo reads the manifest, stamps it, and passes it along. No central database — the manifest *is* the database, and it lives with the cargo.

Agent memory works the same way. Each memory entry is a "manifest" — a JSON document stored in a Git blob, referenced by a Git ref, and carried with the repository. When memory is retrieved, the agent reads the manifest; when memory is stored, the agent writes a new manifest entry. The manifests are synchronized across agents using a CRDT-based gossip protocol.

### 6.2 Design

#### Storage: Git Refs as Manifest Slots

Memory is stored in refs under a dedicated namespace:

```
refs/but-ai/memory/<agent-id>/<category>/<entry-hash>
```

Each ref points to a Git blob containing a JSON manifest:

```json
{
  "id": "sha256-of-content",
  "agent": "dara",
  "category": "pattern",
  "created": "2026-03-28T14:00:00Z",
  "ttl": "720h",
  "expires": "2026-04-27T14:00:00Z",
  "tags": ["authentication", "refactor", "provider-pattern"],
  "content": "The authentication module uses a provider trait with 4 implementations. New providers should implement AuthProvider and register in auth/mod.rs.",
  "embedding_hash": "sha256-of-embedding-vector",
  "relevance_decay": 0.95,
  "access_count": 7,
  "last_accessed": "2026-03-27T10:00:00Z",
  "tide_created": "high tide, Rotterdam, +0.3m"
}
```

#### Categories

| Category | TTL Default | Description |
|----------|-------------|-------------|
| `pattern` | 720h (30d) | Recurring code patterns, conventions, idioms |
| `fact` | 168h (7d) | Specific facts about the codebase (file locations, API signatures) |
| `decision` | 2160h (90d) | Architectural decisions and their rationale |
| `error` | 48h (2d) | Recent errors and their resolutions |
| `identity` | never | Agent identity records (no expiration) |

#### Retrieval: Consensus-Weighted Relevance Scoring

When retrieving memory, the agent computes a relevance score using three signals:

1. **Semantic similarity** (40% weight): Cosine similarity between the query embedding and the memory entry's embedding. Embeddings are computed by the LLM provider and stored alongside the memory entry.

2. **Recency decay** (30% weight): Score = `relevance_decay ^ hours_since_last_access`. A memory accessed yesterday scores higher than one accessed last week.

3. **Consensus validation** (30% weight): How many other agents have referenced this memory? A memory cited by 4 of 5 agents scores higher than one cited by 1. This is the "consensus" in consensus-weighted scoring — popular memories are more trusted.

The final score: `0.4 * semantic + 0.3 * recency + 0.3 * consensus`

Top 5 entries are injected into context. Entries scoring below 0.3 are excluded regardless of rank.

#### Expiration

A background process (triggered on each `but ai` invocation) scans memory refs and deletes expired entries:

```
for each ref in refs/but-ai/memory/<agent-id>/**:
  if entry.expires < now:
    delete ref
    run git gc if orphaned blobs > threshold
```

Expiration is per-agent. An expired memory in Dara's namespace does not affect Koel's copy of the same memory. Each agent maintains its own manifest.

#### Compaction Survival

When an LLM context window is compacted, the agent distinguishes between:

- **Ephemeral context:** Tool call results, intermediate reasoning, PR comment text. This is summarized during compaction and the summary replaces the original.
- **Persistent memory:** Entries from `refs/but-ai/memory/`. These are never summarized — they are ejected from the context during compaction and re-injected from Git refs when needed.

The mechanism: before compaction, the agent writes all persistent memory entries to their refs. After compaction, the agent's system prompt includes a `MEMORY_REHYDRATION` section that instructs the LLM to query memory before proceeding.

#### Long-Term Storage: The Fleet Manifest

For cross-session, cross-repo memory, we use a "fleet manifest" — a shared memory namespace stored in a well-known branch:

```
refs/but-ai/fleet/<namespace>/<entry-hash>
```

Fleet manifests are synchronized across repos via PR-based coordination. When an agent produces a memory entry tagged `fleet`, it is written to both the local agent namespace and the fleet namespace. Other repos can fetch fleet manifests by pulling the fleet refs.

Fleet manifests have longer TTLs (minimum 90 days) and require consensus validation from at least 2 agents before they are trusted by other repos.

#### Identity

Each agent's identity is stored in `refs/but-ai/memory/<agent-id>/identity/self`:

```json
{
  "name": "dara",
  "organization": "tidal-protocol-collective",
  "capabilities": ["patch-generation", "diff-analysis", "code-review"],
  "authorization_scope": {
    "branches": ["feat/*", "fix/*"],
    "repos": ["gitbutler/but"],
    "max_patch_lines": 1000
  },
  "signing_key_fingerprint": "SHA256:abc123...",
  "created": "2026-01-15T00:00:00Z",
  "version": 3
}
```

Identity records never expire. They are versioned — each update increments the version and the old version is preserved in Git history.

### 6.3 Trade-offs

**Considered:** Using a SQLite database stored in the repository for memory.
**Rejected:** SQLite files are opaque to Git — you cannot diff them, branch them, or merge them. Git refs are native to the version control system and participate in all Git operations naturally.

**Considered:** Storing embeddings in the memory entries themselves.
**Rejected:** Embeddings are large (1536+ floats). We store only the hash and recompute embeddings on demand from the LLM provider. This trades compute for storage and keeps memory entries small.

**Considered:** A single shared memory namespace instead of per-agent namespaces.
**Rejected:** Shared memory creates write contention. Per-agent namespaces with consensus-weighted scoring achieve the same effect (popular memories surface) without contention.

---

## 7. Signed Commits via OpenWallet

### 7.1 Approach

Every agent commit is signed using an OpenWallet-managed key. The signing workflow is integrated into the patch application step — after the agent produces INDEX.patch and COMMIT.msg, the signing step is mandatory before the commit is created.

### 7.2 Design

#### Key Provisioning

```
1. Agent registers with OpenWallet: POST /v1/keys/create
   - Agent identity record attached as metadata
   - Organization attestation included
   - Key type: Ed25519 (fastest for commit signing)

2. OpenWallet returns:
   - Public key (stored in agent identity record)
   - Key ID (used for git config user.signingKey)
   - Key fingerprint (stored in identity record)

3. Agent configures local Git:
   - git config user.signingKey = <key-id>
   - git config commit.gpgSign = true
   - git config gpg.format = openwallet
```

#### Signing Flow

```
1. Agent produces INDEX.patch + COMMIT.msg
2. Agent constructs commit object (tree, parent, author, committer, message)
3. Agent sends commit object to OpenWallet for signing:
   POST /v1/sign
   {
     "key_id": "<agent-key-id>",
     "payload": "<commit-object-bytes>",
     "authorization": {
       "branch": "feat/auth-refactor",
       "repo": "gitbutler/but",
       "patch_lines": 142,
       "timestamp": "2026-03-28T14:00:00Z"
     }
   }
4. OpenWallet validates authorization:
   - Is this key authorized for this branch pattern?
   - Is this key authorized for this repo?
   - Is the patch within size limits?
   - Is the key not revoked?
5. If authorized: returns signed commit
6. If not authorized: returns structured error with denial reason
```

#### Authorization Model

Authorization policies are stored as JSON documents in `refs/but-ai/policies/<org>`:

```json
{
  "version": 1,
  "organization": "tidal-protocol-collective",
  "policies": [
    {
      "agent": "dara",
      "allow": {
        "branches": ["feat/*", "fix/*"],
        "repos": ["gitbutler/but", "gitbutler/but-ai"],
        "max_patch_lines": 1000,
        "hours": "00:00-23:59"
      },
      "deny": {
        "branches": ["main", "release/*"],
        "repos": []
      }
    },
    {
      "agent": "sable",
      "allow": {
        "branches": ["**"],
        "repos": ["gitbutler/but"],
        "max_patch_lines": 50
      },
      "deny": {
        "branches": []
      }
    }
  ]
}
```

#### Key Lifecycle

| Event | Action | Urgency |
|-------|--------|---------|
| **Provisioning** | Create key via OpenWallet API, store in identity record | At agent creation |
| **Rotation (planned)** | Generate new key, update identity record, old key marked `rotated` | Monthly or on schedule |
| **Revocation (compromise)** | Immediately mark key `compromised` in OpenWallet, publish revocation to `refs/but-ai/revocations/<key-fingerprint>`, all commits signed by this key are flagged for review | Immediate |
| **Revocation (rotation)** | Mark old key `retired` in OpenWallet after new key is active for 24h | Low urgency |

The distinction between `compromised` and `retired` is critical. A `retired` key means "this key was replaced on schedule; commits signed by it before retirement are trustworthy." A `compromised` key means "this key may have been used by an unauthorized party; all commits signed by it must be reviewed."

#### Verification

Given a signed commit, verification proceeds:

```
1. Extract signature from commit
2. Look up signing key in OpenWallet: GET /v1/keys/<key-id>
3. Verify signature against commit payload
4. Check key status (active, retired, compromised)
5. Look up agent identity from key metadata
6. Check authorization policy for agent + branch + repo
7. Return verification result:
   {
     "valid": true,
     "agent": "dara",
     "organization": "tidal-protocol-collective",
     "key_status": "active",
     "authorized": true
   }
```

### 7.3 Trade-offs

**Considered:** Using standard GPG keys instead of OpenWallet.
**Rejected:** The RFP mandates OpenWallet. Additionally, GPG lacks the authorization model (branch/repo/size constraints) that OpenWallet provides.

**Considered:** Signing at the patch level instead of the commit level.
**Rejected:** Git's existing verification infrastructure works at the commit level. Signing patches would require a parallel verification system.

---

## 8. Token Budget

### 8.1 Model Assumptions

- **Target model:** Claude Opus (200K context window)
- **Typical task:** Implement a 200-line feature across 3 files with 2 cross-repo dependencies

### 8.2 Budget Table

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| **System prompt** | 3,200 | 0 | Once per session | Agent identity (400), tool descriptions (1,200), workspace state summary (800), memory context header (400), coordination protocol (400) |
| **Task ingestion** | 2,500 | 500 | Once per task | PR body (1,500), issue description (500), branch metadata (500). Output: task decomposition. |
| **Planning** | 1,500 | 1,200 | Once per task | Task steps, tool selection, dependency analysis |
| **Tool call (per call)** | 800 | 400 | 8 per task | Parameter formulation (300), result processing (500). Output: next action. Average 8 calls for a 3-file feature. |
| **Patch generation** | 3,000 | 4,000 | Once per task | Context of 3 files (~1000 tokens each). Output: 200-line unified diff. |
| **Commit message** | 500 | 300 | Once per task | Summarize changes for COMMIT.msg |
| **Memory retrieval** | 1,500 | 300 | 2 per task | Query (200), relevance scoring context (300), result injection (1,000 for up to 5 entries). Output: relevance assessment. |
| **Coordination event** | 2,000 | 800 | 2 per task | Read PR comments (1,500), cross-repo reference resolution (500). Output: response comment. |
| **Budget tracking overhead** | 200 | 100 | 5 per task | Token counting, checkpoint logging |
| **TOTAL (typical task)** | **24,300** | **12,100** | -- | **Grand total: 36,400 tokens** |

### 8.3 Budget Justification

- **System prompt at 3,200 tokens** is achievable because we use lazy tool registration — only tools relevant to the current task phase are described in detail. The full 10-tool description is ~2,400 tokens; we include abbreviated descriptions for tools not in the current phase.
- **8 tool calls per task** is based on our Rotterdam deployment data: average 7.3 tool calls per task, rounded up. The median is 6; the 90th percentile is 12.
- **Patch generation at 4,000 output tokens** covers a 200-line diff with context lines. A unified diff for 200 changed lines across 3 files, with 3 lines of context per hunk, is approximately 350 lines or ~3,500 tokens.
- **Grand total of 36,400 tokens** is well within a frontier model's context window and leaves room for 2-3 additional coordination events or tool calls before hitting a 50,000-token budget.

---

## 9. Testing Strategy

### 9.1 Provider-Agnostic Behavior

We test provider-agnostic behavior using a **mock provider shim** (`but-ai-provider-mock`) that implements the provider shim protocol with deterministic responses. Tests run against the mock provider; if they pass, the behavior is provider-agnostic by construction.

The mock provider supports:
- Deterministic tool-calling responses (given input X, always return output Y)
- Configurable latency injection (test timeout handling)
- Token counting simulation (test budget enforcement)
- Deliberate failure modes (test error handling)

### 9.2 Patch Workflow Validation

INDEX.patch round-trip testing:

```
1. Start with a known codebase state (Git fixture)
2. Define a task with a known correct patch
3. Run the agent with the mock provider (which generates the known patch)
4. Apply the patch: git apply INDEX.patch
5. Verify: diff the result against the expected state
6. Reverse: git apply -R INDEX.patch
7. Verify: the codebase is back to the original state
```

This is run for every patch format variant: single-file, multi-file, binary files, file renames, file deletions.

### 9.3 Cross-Repo Coordination

We test cross-repo coordination using a **mock forge adapter** that simulates GitHub's PR API in memory. The mock forge:

- Creates PRs as in-memory objects
- Stores and retrieves comments
- Manages labels
- Supports cross-references between repos
- Simulates eventual consistency (comments appear after a configurable delay)

All coordination tests run against the mock forge. The real forge adapters are integration-tested separately against live instances (GitHub staging environment, self-hosted Gitea).

### 9.4 Token Budget Enforcement

Token budget tests:

1. **Budget exhaustion:** Run a task with a budget of 1,000 tokens (far too low). Verify the agent produces a PARTIAL patch and exits cleanly.
2. **Budget tracking accuracy:** Run a task with the mock provider's token counter. Compare the agent's tracked usage against the mock's ground truth. They must match within 5%.
3. **Graceful degradation:** Set budget to 90% of typical requirement. Verify the agent completes the task but skips optional steps (e.g., memory update, detailed commit message).

---

## 10. Trade-offs and Alternatives

### 10.1 Consensus vs. Speed

Our consensus model is slower than a hierarchical model. A hierarchical orchestrator can assign tasks instantly; our agents must negotiate. For a simple one-agent task, consensus overhead is zero (one agent self-approves). For a five-agent task, consensus adds 2-3 coordination messages and ~2,000 tokens.

We accept this cost because consensus eliminates single-point-of-failure modes. A hierarchical orchestrator that hallucinates a bad task decomposition poisons all downstream agents. Our agents can reject a bad proposal.

### 10.2 Memory Storage vs. Retrieval Speed

Git refs are not optimized for search. Retrieving memory requires scanning refs, reading blobs, and computing relevance scores. For a repository with 1,000 memory entries, this takes ~200ms on a modern SSD — acceptable but not fast.

For repositories with >10,000 memory entries, we propose an optional SQLite index (stored in `.git/but-ai/memory.db`) that caches memory metadata for fast queries. The Git refs remain the source of truth; the SQLite index is a cache that can be rebuilt from refs.

### 10.3 CRDT Complexity

CRDT-based memory synchronization is more complex than a simple "last writer wins" model. We accept this complexity because last-writer-wins causes silent data loss in concurrent agent scenarios. CRDTs guarantee convergence without data loss.

---

## 11. Migration Path

### Phase 1: Parallel Operation

`but-ai` runs alongside the existing MCP server. Both serve MCP clients. The existing `gitbutler_update_branches` tool is reimplemented in `but-ai` as a compatibility shim that translates to the new tool interface.

### Phase 2: Client Migration

MCP clients are updated to use `but-ai`'s expanded tool surface. The old `gitbutler_update_branches` tool remains available but deprecated (logged with a warning).

### Phase 3: Legacy Removal

The old MCP server at `crates/but/src/command/legacy/mcp/mod.rs` is removed. `but-ai` is the sole MCP provider.

**Zero downtime:** At every phase, existing MCP clients continue to function. The `gitbutler_update_branches` tool name is preserved as an alias throughout the migration.

---

## 12. Git Config Keys

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `but-ai.agent.tokenBudget` | integer | 50000 | Maximum tokens per task |
| `but-ai.agent.memoryTTL` | string | "720h" | Default memory entry TTL |
| `but-ai.agent.maxMemoryEntries` | integer | 5 | Max memory entries per retrieval |
| `but-ai.agent.consensusQuorum` | integer | 3 | Minimum agents for consensus |
| `but-ai.agent.tideCycleHours` | integer | 6 | Hours per tide cycle |
| `but-ai.memory.branch` | string | "refs/but-ai/memory" | Memory ref namespace |
| `but-ai.memory.fleetBranch` | string | "refs/but-ai/fleet" | Fleet manifest namespace |
| `but-ai.identity.keyId` | string | (none) | OpenWallet key ID for this agent |
| `but-ai.forge.type` | string | "github" | Forge type (github, gitlab, bitbucket, gitea) |
| `but-ai.forge.apiUrl` | string | (auto-detected) | Forge API base URL |
| `but-ai.provider.shimPath` | string | (PATH) | Additional search path for provider shims |
| `but-ai.coordination.maxComments` | integer | 5 | Max PR comments per coordination event |

Each key is namespaced under `but-ai.` to avoid collision with existing `gitbutler.` keys. Defaults are chosen to be safe — the token budget default of 50,000 is sufficient for most tasks without being wasteful.

---

*"No harbormaster. The protocol is the authority."*
— Tidal Protocol Collective, 2026-03-28 (high tide, Rotterdam, +0.2m)
