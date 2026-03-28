# PROPOSAL.md — Office of Forensic Financial Compliance Review

**Filing Reference:** OFFCR-RFP-2026-BUTAI-003
**Classification:** PUBLIC
**Responding To:** GitButler RFP v1.0.0, Sections 3.1 through 3.7
**Date of Submission:** 2026-03-28
**Approved:** Director Park, 2026-03-28

---

## FORM OFFCR-PROPOSAL-1: Certification of Completeness

The undersigned hereby certifies that this proposal responds to ALL six (6) requirements enumerated in the GitButler `but-ai` Request for Proposals, Version 1.0.0, dated 2026-03-27. No requirement has been omitted. Each response includes: (a) Approach, (b) Design, (c) Trade-offs, and (d) Token budget estimates, as mandated by RFP Section 6.3.

_Certified: Director Park, OFFCR-DIR-001_

---

## Table of Contents

1. [Response to Section 3.1: Plugin Architecture](#1-response-to-section-31-plugin-architecture)
2. [Response to Section 3.2: Provider-Agnostic AI Interface](#2-response-to-section-32-provider-agnostic-ai-interface)
3. [Response to Section 3.3: The But Agent](#3-response-to-section-33-the-but-agent)
4. [Response to Section 3.4: Polyrepo PR-Based Agent Coordination](#4-response-to-section-34-polyrepo-pr-based-agent-coordination)
5. [Response to Section 3.5: Agent Memory and Identity](#5-response-to-section-35-agent-memory-and-identity)
6. [Response to Section 3.6: Signed Commits via OpenWallet](#6-response-to-section-36-signed-commits-via-openwallet)
7. [Token Budget (Section 3.7)](#7-token-budget-section-37)
8. [Testing Strategy (Section 4.5)](#8-testing-strategy-section-45)
9. [Trade-Off Summary](#9-trade-off-summary)

---

## 1. Response to Section 3.1: Plugin Architecture

### 1.1 Approach (ref: RFP Section 3.1, Paragraphs 1-4)

The Office proposes a Rust-based `but-ai` binary structured as a new crate (`crates/but-ai/`) within the existing workspace. The binary will be installed to PATH as `but-ai`, discoverable via `find_external_subcommand()` in `crates/but/src/alias.rs`.

The binary supports two modes of operation, as mandated:

- **CLI mode:** `but ai <subcommand>` — Invoked by human operators or scripts. All output respects `BUT_OUTPUT_FORMAT` and `BUT_JSON` environment variables.
- **MCP server mode:** `but ai mcp` — Starts an MCP server on stdio implementing the `ServerHandler` trait from the `rmcp` crate. Backward-compatible with existing MCP clients.

### 1.2 Design

#### 1.2.1 Crate Structure

```
crates/but-ai/
  src/
    main.rs           -- Entry point, mode dispatch
    cli/
      mod.rs          -- CLI subcommand routing
      agent.rs        -- `but ai agent` implementation
      memory.rs       -- `but ai memory` subcommand
      status.rs       -- `but ai status` subcommand
    mcp/
      mod.rs          -- MCP server, ServerHandler impl
      tools.rs        -- Tool registration bridge
    audit/
      mod.rs          -- Audit trail logging (OFFCR-AUDIT-001)
      trail.rs        -- Structured audit entry format
    provider/
      mod.rs          -- Provider adapter registry
      plugin.rs       -- Dynamic provider loading
  Cargo.toml
```

#### 1.2.2 Environment Variable Handling (ref: RFP Section 3.1, Table 1)

All three environment variables (`BUT_WORKSPACE_DIR`, `BUT_OUTPUT_FORMAT`, `BUT_JSON`) are read at startup and stored in an `EnvironmentContext` struct. The struct is immutable after initialization. No environment variable is read after startup — the Office does not permit runtime mutation of configuration (ref: OFFCR Policy on Configuration Immutability, 2021).

#### 1.2.3 WASI Degradation (ref: RFP Section 3.1, Paragraph 5)

Under WASI builds, plugin discovery is disabled. The Office's proposal degrades as follows:

| Capability | Native | WASI |
|-----------|--------|------|
| CLI mode (`but ai`) | Full | Unavailable (plugin not discoverable) |
| MCP server mode | Full | Available (direct binary invocation) |
| Tool calling | Full | Full (WorkspaceToolset is WASI-compatible) |
| Provider access | Full | Limited (network-dependent providers may fail) |
| Audit trail | Full | Full (Git ref storage is WASI-compatible) |

The Office notes that MCP server mode can still function under WASI if invoked directly (`but-ai mcp`) rather than through plugin discovery. This provides a viable degradation path.

### 1.3 Trade-offs

**Considered and rejected: Separate binary outside the workspace.** A standalone binary would avoid coupling with the `but` crate workspace but would lose access to shared types in `but-llm`, `but-tools`, and `but-ctx`. The Office determined that the benefit of shared types outweighs the cost of workspace coupling (ref: OFFCR-RFP-2026-ANALYSIS-001).

**Considered and rejected: Feature flag within `but` itself.** Adding `but-ai` as a feature flag in the main `but` crate would violate RFP Section 4.6 ("must not modify the core `but` binary"). Rejected on compliance grounds.

### 1.4 New Git Config Keys

| Key | Type | Default | Justification |
|-----|------|---------|---------------|
| `but-ai.agent.tokenBudget` | integer | 50000 | Per-session token limit (Section 3.3 requirement) |
| `but-ai.agent.memoryBranch` | string | `refs/but-ai/memory` | Base ref for memory storage |
| `but-ai.agent.auditBranch` | string | `refs/but-ai/audit` | Base ref for audit trail |
| `but-ai.agent.reviewChain` | boolean | true | Enable sequential review chain |
| `but-ai.provider.pluginDir` | string | `$XDG_DATA_HOME/but-ai/providers` | Directory for provider plugins |

---

## 2. Response to Section 3.2: Provider-Agnostic AI Interface

### 2.1 Approach

The Office proposes strict adherence to the existing `but-llm` crate. No new LLM client will be introduced. All four existing providers (OpenAI, Anthropic, Ollama, LMStudio) are supported without modification to `but-llm`.

### 2.2 Design

#### 2.2.1 Provider Bridge

The `but-ai` plugin creates an `LLMProvider` via `LLMProvider::from_git_config()` at startup. All tool calls are routed through `tool_calling_loop` or `tool_calling_loop_stream`. The streaming variant is used when `BUT_OUTPUT_FORMAT` is `human` (for terminal progress display). The non-streaming variant is used when `BUT_JSON` is set (for machine consumption).

#### 2.2.2 Provider Plugin Mechanism (ref: RFP Section 3.2, Requirement 3)

New providers are added via shared libraries placed in `but-ai.provider.pluginDir`. Each plugin implements a C-ABI-compatible interface:

```rust
#[repr(C)]
pub struct ProviderPlugin {
    pub name: extern "C" fn() -> *const c_char,
    pub version: extern "C" fn() -> *const c_char,
    pub create_provider: extern "C" fn(config: *const c_char) -> *mut c_void,
    pub send_request: extern "C" fn(
        provider: *mut c_void,
        request: *const c_char,
    ) -> *const c_char,
    pub destroy_provider: extern "C" fn(provider: *mut c_void),
}
```

The plugin is loaded at startup via `dlopen` (Unix) or `LoadLibrary` (Windows). The Office notes that this approach introduces a security boundary that must be audited — loading arbitrary shared libraries is a risk (Finding: OFFCR-RISK-PROVIDER-001). Mitigation: plugins must be signed with an OpenWallet key whose public key is registered in Git config at `but-ai.provider.trustedKeys`.

#### 2.2.3 MCP Tool Registration

All ten WorkspaceToolset tools are registered with the MCP server via `tool_router`. The bridge translates between the `Tool` trait's `parameters()` (JSON Schema) and MCP's tool declaration format. Each tool call is logged in the audit trail with: tool name, parameters (sanitized), result summary, and token cost.

### 2.3 Trade-offs

**Considered and rejected: WASM-based provider plugins.** WASM plugins would provide better sandboxing than shared libraries. Rejected because: (a) WASM runtime adds 2-3MB to the binary, (b) the C-ABI approach is simpler and sufficient when combined with OpenWallet signature verification, and (c) WASI builds already face limitations that WASM plugins would not resolve.

**Considered and rejected: gRPC-based provider interface.** A gRPC server per provider would allow out-of-process providers. Rejected because it violates RFP Section 3.4, Requirement 5 ("No proprietary dependencies" — gRPC runtime is a dependency beyond Git and forge APIs).

---

## 3. Response to Section 3.3: The But Agent

### 3.1 Approach

The Office's agent implementation follows the Office's core principle: every action is audited, every output is reviewed, and no single agent acts unilaterally.

The `but ai agent` command accepts a task via CLI argument, PR body reference, or branch metadata. The agent operates through the four-agent review chain described in AGENTS.md.

### 3.2 Design

#### 3.2.1 Task Execution Flow

```
1. INTAKE (Analyst Vasquez)
   - Read task description from source (PR body, issue, CLI arg)
   - Query agent memory for relevant prior work
   - Produce execution plan

2. EXECUTION (Analyst Vasquez)
   - Execute plan using WorkspaceToolset tools
   - Produce INDEX.patch + COMMIT.msg
   - Record all tool calls in audit trail

3. AUDIT (Auditor Chen)
   - Verify patch applies cleanly
   - Verify commit message references correct case/task
   - Verify no out-of-scope changes
   - Produce audit findings (numbered)

4. APPROVAL (Director Park)
   - Review audit findings
   - Verify policy compliance
   - Sign commit via OpenWallet or return for revision

5. INFRASTRUCTURE (Specialist Webb, as needed)
   - Provider health checks
   - Token budget monitoring
   - Ref management
```

#### 3.2.2 Patch Workflow (ref: RFP Section 2.3)

The agent produces `INDEX.patch` as a unified diff against the current index. The patch is generated by:

1. Analyst Vasquez uses `GetProjectStatus` to read current workspace state.
2. Vasquez formulates changes as tool calls (`Commit`, `CreateBranch`, etc.).
3. All tool calls produce structured results that are composed into a patch.
4. The patch is validated by Auditor Chen before it leaves the agent.

The agent DOES NOT: make direct file edits, run `git commit`, or call `but commit` itself. It produces patches. The `but` orchestrator applies them.

#### 3.2.3 Branch Naming

The Office extends the existing `s01.s04` convention with a case reference prefix:

```
offcr/<case-number>/<dependency-chain>
```

Example: `offcr/2026-0042/s01.s03` means case 2026-0042, step 3 depends on step 1.

The case number provides institutional traceability. Every branch is attributable to a specific investigation or task.

#### 3.2.4 Token Budget Enforcement

Specialist Webb monitors token consumption at each tool call boundary. When usage reaches 80% of the budget, a warning is issued. When usage reaches 90%, the agent enters "wind-down" mode: it must produce whatever partial work is complete as a valid patch within the remaining 10%. When usage reaches 100%, the agent halts and produces an error:

```json
{
  "error": {
    "code": "AGENT_BUDGET_EXCEEDED",
    "message": "Agent exhausted token budget (used 48000 of 50000)",
    "context": {
      "budget": 50000,
      "used": 48000,
      "partial_patch": true,
      "case_reference": "OFFCR-2026-0042",
      "audit_trail_ref": "refs/but-ai/audit/2026-0042"
    }
  }
}
```

#### 3.2.5 Progress Reporting

Progress is reported in structured format at each stage transition:

```json
{
  "stage": "AUDIT",
  "agent": "OFFCR-AUD-001",
  "case_reference": "OFFCR-2026-0042",
  "status": "in_progress",
  "findings_so_far": 12,
  "token_usage": { "used": 31000, "budget": 50000 },
  "timestamp": "2026-03-28T14:32:00Z"
}
```

### 3.3 Trade-offs

**Considered and rejected: Parallel agent execution.** Running Vasquez and Chen simultaneously would improve throughput. Rejected because independent review requires the reviewer to be uninfluenced by the producer's process (ref: OFFCR Policy Memo 2019-07). Sequential execution is slower but produces higher-quality output.

**Considered and rejected: Lightweight review for low-risk changes.** A tiered review system where trivial changes skip Chen's audit. Rejected. The Office does not have a "trivial" classification. All changes are audited (ref: OFFCR Policy Manual, Chapter 3, Section 3.2).

---

## 4. Response to Section 3.4: Polyrepo PR-Based Agent Coordination

### 4.1 Approach

The Office treats PR-based coordination as inter-agency communication. Just as the Office files referrals, findings, and evidence with other agencies using structured forms, agents communicate across repositories using structured PR comments with defined schemas.

### 4.2 Design

#### 4.2.1 Forge Adapter Interface

```rust
pub trait ForgeAdapter: Send + Sync {
    fn create_pr(&self, params: CreatePrParams) -> Result<PrReference>;
    fn comment_on_pr(&self, pr: &PrReference, comment: &AgentMessage) -> Result<CommentId>;
    fn read_pr_comments(&self, pr: &PrReference) -> Result<Vec<AgentMessage>>;
    fn list_pr_labels(&self, pr: &PrReference) -> Result<Vec<String>>;
    fn add_pr_label(&self, pr: &PrReference, label: &str) -> Result<()>;
    fn get_pr_status(&self, pr: &PrReference) -> Result<PrStatus>;
}

pub struct PrReference {
    pub forge: ForgeKind,       // GitHub, GitLab, Bitbucket, Gitea
    pub owner: String,
    pub repo: String,
    pub number: u64,
}
```

The Office provides a reference implementation for GitHub using the REST API. The adapter is stateless — no forge connection is maintained between calls. Each call authenticates independently using credentials from Git config (`but-ai.forge.github.token`).

#### 4.2.2 PR Comment Schema (ref: RFP Section 3.4, Requirement 3)

All agent-to-agent messages are embedded in PR comments using a structured format enclosed in a code fence:

```
<!-- OFFCR-AGENT-MSG -->
```json
{
  "schema_version": "1.0.0",
  "message_type": "task_assignment | status_report | dependency | patch_handoff | budget_report",
  "sender": {
    "agent_id": "OFFCR-SA-001",
    "org": "156-office-of-forensic-financial-compliance"
  },
  "case_reference": "OFFCR-2026-0042",
  "timestamp": "2026-03-28T14:32:00Z",
  "payload": { ... }
}
```
<!-- /OFFCR-AGENT-MSG -->
```

The comment marker (`OFFCR-AGENT-MSG`) allows structured messages to be extracted from comments that also contain human-readable text. The Office considers this essential: a PR should be readable by both agents and humans.

#### 4.2.3 Message Type Payloads

**Task Assignment:**
```json
{
  "assignee": "OFFCR-SA-001",
  "task": "Reconstruct transaction history for addresses 0x...",
  "deadline_tokens": 25000,
  "priority": "ROUTINE"
}
```

**Status Report:**
```json
{
  "task_id": "OFFCR-2026-0042-T001",
  "status": "completed | blocked | failed",
  "detail": "Patch produced. Pending audit.",
  "tokens_used": 18000,
  "tokens_remaining": 32000
}
```

**Dependency Declaration:**
```json
{
  "depends_on": [
    { "forge": "github", "owner": "gitbutler", "repo": "but-tools", "pr": 42 },
    { "forge": "gitlab", "owner": "offcr", "repo": "evidence-lib", "pr": 17 }
  ]
}
```

**Patch Handoff:**
```json
{
  "patch_ref": "refs/but-ai/patches/OFFCR-2026-0042-001",
  "commit_msg_ref": "refs/but-ai/patches/OFFCR-2026-0042-001.msg",
  "review_status": "audited | unaudited"
}
```

**Budget Report:**
```json
{
  "budget": 50000,
  "used": 31000,
  "remaining": 19000,
  "projected_completion": true
}
```

#### 4.2.4 Cross-Repo Coordination

The Office tracks cross-repo dependencies using a dependency ledger stored in `refs/but-ai/deps/<case-number>`. The ledger is a JSON file listing all PRs in all repos related to the case. When a dependency is resolved (PR merged), the ledger is updated and downstream agents are notified via a status report comment on their PR.

### 4.3 Trade-offs

**Considered and rejected: Webhook-based notification.** Webhooks would provide real-time notification of PR events. Rejected because webhooks require a listening server, which is infrastructure beyond Git and the forge API (violates RFP Section 3.4, Requirement 5).

**Considered and rejected: Single shared repository for coordination.** A central coordination repo would simplify dependency tracking. Rejected because it creates a single point of failure and requires all participants to have access to the coordination repo.

---

## 5. Response to Section 3.5: Agent Memory and Identity

### 5.1 Approach: Audit-Trail Memory

The Office's memory scheme is derived from its founding principle: **the audit trail IS the memory**. Every memory access is logged with a timestamp and justification, and the trail of accesses itself becomes a searchable, immutable chain of evidence.

This is not an analogy. In forensic accounting, the most valuable information is often not "what happened" but "who looked at what happened, when, and why." The same principle applies to agent memory: knowing that Agent Vasquez retrieved a memory about authentication patterns while investigating a credential rotation task tells you something important about both the memory and the task.

### 5.2 Design

#### 5.2.1 Storage Structure

Memory is stored in Git refs under `refs/but-ai/memory/<agent-id>/`:

```
refs/but-ai/memory/OFFCR-SA-001/
  entries/
    <entry-hash>.json     -- Individual memory entries
  trail/
    <timestamp>.json      -- Access log entries
  index/
    relevance.json        -- Relevance index (keyword -> entry mappings)
    expiry.json           -- TTL tracking
```

Each memory entry is a JSON blob:

```json
{
  "id": "mem-2026-0042-007",
  "created": "2026-03-28T14:00:00Z",
  "expires": "2026-04-28T14:00:00Z",
  "category": "finding | pattern | precedent | context",
  "content": "Authentication tokens stored in plaintext in config.toml",
  "keywords": ["authentication", "tokens", "plaintext", "config"],
  "case_reference": "OFFCR-2026-0042",
  "access_count": 0,
  "last_accessed": null,
  "created_by": "OFFCR-SA-001",
  "justification": "Discovered during code review of auth module"
}
```

#### 5.2.2 The Audit Trail as Memory

Every memory access creates a trail entry:

```json
{
  "timestamp": "2026-03-28T15:30:00Z",
  "agent": "OFFCR-SA-001",
  "action": "retrieve",
  "query": "authentication patterns",
  "results": ["mem-2026-0042-007", "mem-2026-0038-012"],
  "justification": "Investigating credential rotation task in PR #42",
  "tokens_consumed": 340
}
```

The trail itself is searchable. An agent can query: "When was the last time I looked at authentication-related memories?" This meta-query reveals patterns of investigation that inform current work. In forensic accounting, this is called "case linkage" — discovering that two apparently unrelated investigations share common evidence because the same analyst looked at similar records in both.

#### 5.2.3 Relevance Scoring

Relevance is scored using a four-factor model:

1. **Keyword overlap** (30%): TF-IDF similarity between the query and the memory entry's keywords and content.
2. **Case linkage** (25%): Memories from the same case or related cases (linked via dependency declarations) score higher.
3. **Access recency** (25%): Memories accessed recently by any agent in the review chain are more likely to be relevant. This is extracted from the audit trail — the trail IS the recency signal.
4. **Access frequency** (20%): Memories accessed frequently across multiple cases are likely to represent general patterns (precedent) rather than case-specific facts.

#### 5.2.4 Expiration (ref: RFP Section 3.5, Requirement 3)

All memory entries have a TTL set at creation time. Default TTLs by category:

| Category | Default TTL | Rationale |
|----------|-------------|-----------|
| finding | 90 days | Findings are permanent records but memory refs expire; the finding itself is in the case file |
| pattern | 180 days | Patterns remain relevant longer |
| precedent | 365 days | Precedents are long-lived institutional knowledge |
| context | 7 days | Ephemeral task context expires quickly |

Expired entries are not deleted. They are moved to `refs/but-ai/memory/<agent-id>/expired/` and excluded from relevance queries. The Office does not delete records (ref: OFFCR Records Retention Policy, 2015).

#### 5.2.5 Compaction Survival

When an LLM context window is compacted, the agent writes a "compaction summary" to memory:

```json
{
  "category": "context",
  "content": "COMPACTION SUMMARY: Investigating auth module. Found plaintext tokens. Patch 60% complete. Next: rotate credentials in config.toml.",
  "keywords": ["compaction", "auth", "tokens", "config"],
  "expires": "+24h"
}
```

After compaction, the agent rehydrates by:
1. Reading the compaction summary
2. Querying the audit trail for its most recent actions
3. Querying memory for entries relevant to the compaction summary's keywords

This three-step rehydration restores working context without requiring the full pre-compaction history.

#### 5.2.6 Long-Term Storage

The Office proposes a cross-session memory store in `refs/but-ai/archive/`. Archive entries are created when a case closes: Auditor Chen reviews all memory entries associated with the case and promotes durable findings and patterns to the archive. Archive entries have a minimum TTL of 365 days and are searchable from any session.

Cross-repo memory is shared via forge: an agent can reference memory entries from another repo by including the repo's URL and the entry's ref path in a PR comment. The memory is not copied — it is referenced. This avoids duplication while maintaining each repo's autonomy over its own memory.

#### 5.2.7 Identity (ref: RFP Section 3.5, Requirement 7)

Each agent's identity is stored in `refs/but-ai/identity/<agent-id>`:

```json
{
  "agent_id": "OFFCR-SA-001",
  "name": "Analyst Vasquez",
  "org": "156-office-of-forensic-financial-compliance",
  "role": "Senior Forensic Analyst",
  "capabilities": ["patch_generation", "transaction_reconstruction", "pattern_detection"],
  "authorization_scope": {
    "branches": ["offcr/*", "feat/*"],
    "max_patch_size": 1000,
    "signing_authority": false
  },
  "openwallet_key_id": "offcr-sa-001-2026",
  "created": "2026-01-15T00:00:00Z"
}
```

### 5.3 Trade-offs

**Considered and rejected: Embedding-based memory retrieval.** Vector embeddings would provide semantic search. Rejected because: (a) embedding generation requires additional LLM calls, consuming tokens for non-task work, (b) the keyword + audit-trail approach provides sufficient relevance for the Office's structured domain, and (c) embeddings are opaque — the Office cannot audit why an embedding matched. Keywords and case linkage are transparent and auditable.

**Considered and rejected: In-memory database.** An SQLite or similar database for memory storage. Rejected because it introduces a dependency beyond Git (violates the spirit of RFP Section 3.5, Requirement 1).

---

## 6. Response to Section 3.6: Signed Commits via OpenWallet

### 6.1 Approach

The Office treats commit signing as a non-negotiable compliance requirement. Every commit is signed. No exceptions. No "sign later" mode. No "skip signing for local development." The Office's position is that an unsigned commit is an unsubstantiated assertion, and the Office does not produce unsubstantiated assertions.

### 6.2 Design

#### 6.2.1 Key Management

Each agent is provisioned an OpenWallet-managed key at identity creation time. The key is stored in the agent's OpenWallet vault, referenced by `openwallet_key_id` in the identity record.

Key lifecycle:

| Event | Procedure | Form Required |
|-------|-----------|---------------|
| Provisioning | Specialist Webb generates key, registers in OpenWallet and identity record | OFFCR-KEY-PROVISION-1 |
| Rotation (scheduled) | New key generated, old key marked `rotated`, 30-day overlap period | OFFCR-KEY-ROTATE-1 |
| Revocation (compromise) | Key immediately marked `revoked`, all sessions terminated, incident report filed | OFFCR-KEY-REVOKE-1 (URGENT) |
| Revocation (routine) | Key marked `retired`, no overlap period, effective at end of current session | OFFCR-KEY-RETIRE-1 |

The distinction between "rotated" and "revoked" is critical. A rotated key's historical signatures remain valid — the key was good when it was used. A revoked key's historical signatures are marked as "signed under compromised key" in the audit trail, triggering a review of all commits signed with that key.

#### 6.2.2 Authorization Model

Authorization is policy-based, not key-based. Possession of a key does not authorize a commit. The key proves identity; the policy determines authorization.

Policy is stored in `refs/but-ai/policy/<org-id>`:

```json
{
  "rules": [
    {
      "agent": "OFFCR-SA-001",
      "allowed_branches": ["offcr/*", "feat/*"],
      "denied_branches": ["main", "release/*"],
      "max_patch_lines": 1000,
      "requires_approval_from": ["OFFCR-DIR-001"]
    },
    {
      "agent": "OFFCR-DIR-001",
      "allowed_branches": ["*"],
      "denied_branches": [],
      "max_patch_lines": null,
      "requires_approval_from": []
    }
  ]
}
```

#### 6.2.3 Authorization Chain Verification

A signed commit can be verified by:

1. Extracting the signing key from the commit signature.
2. Looking up the agent identity in `refs/but-ai/identity/` by `openwallet_key_id`.
3. Looking up the authorization policy in `refs/but-ai/policy/`.
4. Verifying that the agent was authorized to commit to the target branch at the commit timestamp.
5. Verifying that the patch size is within the agent's allowed limits.
6. Verifying that any required approvals (from the `requires_approval_from` list) are present as co-signatures or approval records in the audit trail.

This six-step verification chain is itself logged in the audit trail.

### 6.3 Trade-offs

**Considered and rejected: Per-commit approval tokens.** Each approval could issue a short-lived token embedded in the commit metadata. Rejected because it adds complexity without improving security — the audit trail already records approvals, and the policy lookup is deterministic.

**Considered and rejected: Multi-signature commits.** Requiring multiple agent signatures on each commit. Rejected as impractical for the patch-based workflow — the agent produces the patch, the orchestrator applies it, and only one signing key is available at commit time. Instead, the Office uses the approval chain (audit trail records of Director Park's approval) as the equivalent of a co-signature.

---

## 7. Token Budget (Section 3.7)

### 7.1 Budget Table (ref: RFP Appendix C)

The following estimates assume Claude Opus as the frontier model. Estimates are for a typical task: implementing a 200-line feature across 3 files with 2 cross-repo dependencies.

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| **System prompt** | 3,200 | 0 | Once per session | Agent identities (4), tool descriptions (10), audit policy, workspace state |
| **Task ingestion** | 2,800 | 600 | Once per task | PR body, issue description, branch metadata, memory context |
| **Planning** | 2,000 | 1,200 | Once per task | Execution plan with case references and audit checkpoints |
| **Tool call (per call)** | 800 | 400 | 6 per task | GetProjectStatus, GetBranchChanges, GetCommitDetails, CreateBranch, Commit x2 |
| **Patch generation** | 2,500 | 3,500 | Once per task | INDEX.patch for 200 lines across 3 files |
| **Commit message** | 400 | 300 | Once per task | COMMIT.msg with case reference and audit trail pointer |
| **Audit cycle** | 2,500 | 1,200 | Once per task | Chen's review: read patch + produce numbered findings |
| **Approval cycle** | 1,500 | 400 | Once per task | Park's review: read findings + issue approval |
| **Memory retrieval** | 600 | 200 | 2 per task | Query formulation + result injection |
| **Coordination event** | 1,000 | 500 | 2 per task | PR comment read/write, dependency ledger update |
| **TOTAL (typical task)** | **22,700** | **11,000** | -- | 33,700 total tokens |

### 7.2 Justification

The Office's token budget is higher than a single-agent system because it runs four agents in a sequential review chain. The audit and approval cycles add approximately 5,600 tokens (17% of total). The Office considers this overhead justified: the Cryptocurrency Compliance Failure (Section 4.3 of README.md) cost the Office fourteen months of wasted work. A 17% token overhead to prevent such failures is, in the Office's assessment, a sound investment.

### 7.3 Optimization Opportunities

1. **Lazy audit trail loading.** The full audit trail is not loaded into context until Chen's review cycle. Prior to that, only the trail index is loaded.
2. **Compressed memory entries.** Memory entries older than 30 days are stored in a compressed format (keywords + summary only, no full content). Full content is loaded on demand.
3. **Cached workspace state.** `GetProjectStatus` results are cached for the duration of a task. Subsequent calls return the cached version unless a commit has occurred.

---

## 8. Testing Strategy (Section 4.5)

### 8.1 Provider-Agnostic Testing (ref: RFP Section 4.5, Requirement 1)

The Office proposes a mock provider implementation (`MockLLMProvider`) that implements the same interface as `LLMProvider` but returns deterministic responses. The mock provider is configurable: test fixtures define the expected tool calls and responses for each test scenario. All four provider backends are tested through the same test suite — the only variable is the provider configuration.

### 8.2 Patch Workflow Validation (ref: RFP Section 4.5, Requirement 2)

Round-trip testing: each test creates a known workspace state, runs the agent with a defined task, captures the produced INDEX.patch, applies the patch to a clean workspace, and verifies the result matches the expected state. The Office insists on testing both the "happy path" (patch applies cleanly) and the "contaminated workspace" path (workspace was modified between patch generation and application).

### 8.3 Cross-Repo Coordination Testing (ref: RFP Section 4.5, Requirement 3)

A mock forge implementation (`MockForgeAdapter`) that implements the `ForgeAdapter` trait with in-memory PR storage. Tests create multiple mock repos, simulate cross-repo dependencies, and verify that the coordination protocol correctly tracks and resolves dependencies. The mock forge records all API calls for audit trail verification.

### 8.4 Token Budget Testing (ref: RFP Section 4.5, Requirement 4)

Budget enforcement is tested by configuring the mock provider to report token counts at each call. Tests verify: (a) warning at 80%, (b) wind-down at 90%, (c) halt at 100%, (d) partial patch production on budget exhaustion, (e) structured error output with correct usage figures.

---

## 9. Trade-Off Summary

| Decision | Chosen | Rejected | Rationale |
|----------|--------|----------|-----------|
| Binary location | Workspace crate | Standalone binary | Shared types with but-llm, but-tools |
| Provider plugins | C-ABI shared libraries | WASM modules, gRPC | Simplicity, no runtime dependency |
| Review chain | Sequential (4 agents) | Parallel, tiered | Audit independence, policy compliance |
| Memory storage | Git refs + audit trail | Embeddings, SQLite | Transparency, auditability, Git-native |
| Memory retrieval | Keyword + case linkage | Vector similarity | Auditable relevance scoring |
| Commit signing | Mandatory, no exceptions | Optional, tiered | OFFCR policy, RFP mandate |
| Cross-repo coordination | Forge API + PR comments | Webhooks, shared repo | No infrastructure beyond Git + forge |
| Testing | Mock providers + mock forge | Live API testing | Deterministic, repeatable, auditable |

---

*CERTIFICATION: This proposal responds to all six (6) requirements of the GitButler `but-ai` RFP, Version 1.0.0. The Office certifies that no requirement has been omitted, no material fact has been misstated, and all token budget estimates represent the Office's good-faith assessment of actual resource consumption.*

*Filing Reference: OFFCR-RFP-2026-BUTAI-003*
*Prepared by: Office of Forensic Financial Compliance Review*
*Date: 2026-03-28*
*Classification: PUBLIC*
*Approval chain: Vasquez (prepared) -> Chen (audited) -> Park (approved)*
