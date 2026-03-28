# Phosphor Maritime Institute — Proposal

**Response to: `but ai` Plugin RFP v1.0.0**

---

## 1. Plugin Architecture (RFP 3.1)

We propose `but-ai` as a Rust binary crate within the existing workspace, structured as `crates/but-ai/`. The binary serves dual duty: CLI subcommands via `but ai <cmd>` and MCP server mode via `but ai mcp`.

**CLI mode** parses `BUT_WORKSPACE_DIR`, `BUT_OUTPUT_FORMAT`, and `BUT_JSON` from environment. Subcommands: `agent` (run autonomous task), `memory` (query/store agent memory), `status` (agent health and budget), `mcp` (start MCP server).

**MCP mode** implements `ServerHandler` from `rmcp`, registering all `WorkspaceToolset` tools plus three new tools: `AgentRun`, `MemoryQuery`, `BudgetStatus`. Backward-compatible with existing `gitbutler_update_branches` by wrapping it as a legacy adapter.

**WASI degradation:** Under WASI, plugin discovery is disabled. We propose a fallback: `but ai` functionality is compiled as a library crate (`but-ai-core`) that the main `but` binary can link statically when WASI-targeted. The agent operates in "reduced mode" — no plugin discovery, no fork/exec, but the core patch generation and memory systems work via direct function calls. Provider support is limited to providers reachable via WASI HTTP.

## 2. Provider-Agnostic AI Interface (RFP 3.2)

We use `but-llm` as-is. The provider-agnostic layer is a trait `AgentProvider` that wraps `LLMProvider` methods with agent-specific concerns: token budget tracking per call, automatic retry with exponential backoff, and provider capability detection (does this provider support tool calling? structured output?).

**New provider mechanism:** A `ProviderAdapter` trait with `negotiate_capabilities()` and `translate_tool_schema()` methods. New providers implement this trait as a shared library (`.so`/`.dylib`) loaded at runtime via `libloading`. The adapter translates between `but-llm`'s internal format and the provider's native API.

**Trade-off:** We considered WASM-based provider plugins but rejected them — the overhead of WASM sandboxing for what is essentially an HTTP client adapter is not justified. Dynamic linking is simpler and sufficient.

## 3. The But Agent (RFP 3.3)

The agent operates in a loop: **observe** (read task, read memory, read workspace state) → **plan** (decompose into steps, estimate token cost) → **execute** (tool calls, patch generation) → **report** (structured output, budget accounting).

Patch generation follows a biological metaphor we call "phosphorescent tracing." The agent reads the target files, constructs a mental model (consuming input tokens), then emits the unified diff in a single output burst. The diff is validated locally before being written as `INDEX.patch`. The commit message is generated in a separate, cheaper call using the diff as context.

**Branch naming:** We extend the `sNN.sNN` convention with an agent identity prefix: `phosphor/<agent-role>/s01.s04`. This encodes the originating team, the agent's role, and the dependency chain.

**Budget enforcement:** The planning phase produces a token estimate. If the estimate exceeds remaining budget by more than 20%, the agent enters "abstract mode" — it produces a high-level patch description and a `PARTIAL` flag in COMMIT.msg instead of attempting a patch that would exhaust budget mid-generation.

## 4. Polyrepo PR Coordination (RFP 3.4)

**Forge adapter interface:**

```
trait ForgeAdapter {
    fn create_pr(&self, repo: &RepoRef, pr: &PullRequest) -> Result<PrId>;
    fn comment(&self, pr: &PrId, msg: &AgentMessage) -> Result<CommentId>;
    fn list_comments(&self, pr: &PrId) -> Result<Vec<AgentMessage>>;
    fn get_pr_status(&self, pr: &PrId) -> Result<PrStatus>;
    fn resolve_cross_ref(&self, ref_str: &str) -> Result<PrId>;
}
```

Reference implementation: GitHub (REST API). The adapter is stateless — all state lives in PR comments and labels.

**PR comment schema:**

```json
{
  "schema": "phosphor-agent-msg/v1",
  "type": "task_assignment | status_report | dependency | patch_handoff | budget_report",
  "from": "<agent-identity>",
  "timestamp": "<ISO-8601>",
  "body": { ... },
  "signature": "<OpenWallet-signature>"
}
```

Every agent message is signed. Unsigned messages are ignored. Cross-repo references use the `org/repo#NN` format, resolved through the forge adapter.

**Trade-off:** We considered using Git notes instead of PR comments for coordination. Rejected because notes require push access to all repos, while PR comments only require comment permission.

## 5. Agent Memory and Identity (RFP 3.5)

**Storage:** Memory is stored under `refs/phosphor/memory/<agent-name>/` as Git blobs. Each memory entry is a JSON blob with fields: `key`, `value`, `tags`, `created`, `ttl`, `relevance_score`. The memory index is a Git tree object that maps keys to blob SHAs.

**Relevance scoring:** We use a "photobleaching" model. Each memory entry has an initial relevance score of 1.0 that decays exponentially over time: `score = initial * e^(-λt)`, where λ is the decay constant and t is time since creation. When an agent accesses a memory entry, it is "re-illuminated" — the decay timer resets. Entries that fall below a threshold (0.1) are eligible for garbage collection.

**Semantic matching:** For retrieval, we compute a lightweight embedding of the query using the first 200 tokens of the current task context, then compare against stored entry tags using cosine similarity. This avoids the cost of embedding the full memory corpus — only tags are compared.

**Compaction survival:** Critical memories are tagged `persistent`. These are injected into every compacted context regardless of relevance score. The agent distinguishes persistent from ephemeral memory at write time based on the memory's source: memories derived from architectural decisions are persistent; memories derived from specific task context are ephemeral.

**Long-term storage:** A shared `refs/phosphor/archive/` branch stores cross-session, cross-repo knowledge. Agents contribute to the archive when they discover patterns that generalize beyond a single task. Archive entries require two agent signatures (the author and one reviewer) before being stored.

**Identity:** Each agent has an identity record stored at `refs/phosphor/identity/<agent-name>` containing: name, role, capabilities list, authorization scope (branch patterns), creation timestamp, and public key fingerprint (linked to OpenWallet).

## 6. Signed Commits via OpenWallet (RFP 3.6)

Every agent commit is signed using an OpenWallet-managed Ed25519 key. The signing flow:

1. Agent produces `INDEX.patch` + `COMMIT.msg`
2. The orchestrator calls `but commit` with the patch
3. Before finalizing, the commit object is passed to the agent's OpenWallet signer
4. The signature is embedded in the commit headers
5. Verification reads the signature, resolves the public key via OpenWallet, and checks the agent's authorization scope against the target branch

**Authorization model:** Policy is stored as a TOML file at `.but-ai/policy.toml`:

```toml
[agents.postdoc-1]
branches = ["phosphor/*", "feat/*"]
max_patch_lines = 500

[agents.lab-tech]
branches = ["infra/*"]
max_patch_lines = 200
```

**Key lifecycle:** Keys are provisioned at agent creation via `but ai agent init`. Rotation happens on a configurable schedule (default: 30 days). Revocation for compromise immediately invalidates all commits signed after the suspected compromise time and flags them for re-review.

## 7. Token Budget

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| System prompt | 3,200 | 0 | Once/session | Agent identity, 10 tool descriptions, memory context |
| Task ingestion | 2,500 | 500 | Once/task | PR body, branch metadata, relevant file headers |
| Planning | 1,500 | 800 | Once/task | Step decomposition, tool selection, budget estimate |
| Tool call (per call) | 1,200 | 600 | ~5/task | Parameter formulation + result processing |
| Patch generation | 3,000 | 4,500 | Once/task | Context reading + unified diff output |
| Commit message | 800 | 300 | Once/task | Diff summary + conventional commit format |
| Memory retrieval | 600 | 200 | 2/task | Query + relevance scoring + injection |
| Coordination event | 1,500 | 500 | 1/task | PR comment read/write |
| **TOTAL (typical task)** | **19,100** | **10,300** | -- | 200-line feature, 3 files, 2 deps |

## Unique Insight

From our oceanographic work, we learned that the most effective signaling systems in nature are not the loudest — they are the ones that decay gracefully. A bioluminescent signal that persists indefinitely becomes noise. The same is true for agent memory: a memory system without decay becomes a liability. Our photobleaching model ensures that agent memory is self-cleaning. Stale context does not accumulate. The system forgets what it should forget, and remembers what it keeps using.

---

*"In the deep water, the only light that matters is the light that still glows."*
