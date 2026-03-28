# ZoneMap — Proposal

**Response to: `but ai` Plugin RFP v1.0.0**

---

## 1. Plugin Architecture (RFP 3.1)

`but-ai` as a PATH binary in `crates/but-ai/`. Lean and fast — binary size matters because we deploy on CI runners that pull tools on every run.

CLI mode: `but ai agent`, `but ai memory`, `but ai status`, `but ai mcp`. Environment: `BUT_WORKSPACE_DIR`, `BUT_OUTPUT_FORMAT`, `BUT_JSON`. Output defaults to human-readable; `BUT_JSON=1` switches to structured JSON.

MCP mode: wraps `WorkspaceToolset` plus `AgentRun`, `MemoryGet`, `MemorySet`, `BudgetCheck`. We add `ZoneCheck` — a domain-specific tool that validates whether a proposed change would violate any constraints in the project's configuration. This is our differentiator: compliance checking baked into the agent's tool palette.

WASI: library mode fallback via `but-ai-core`. No plugin discovery. Core agent loop functional. Provider access via WASI HTTP. We have done this before — our zoning compiler already runs as wasm32-wasip2 for browser-based demos.

Design choice: no daemon process. The agent is invoked per-task and exits. Persistent state lives in Git refs, not in a running process. This simplifies deployment and eliminates a class of state-synchronization bugs.

## 2. Provider-Agnostic AI Interface (RFP 3.2)

Wrapping `but-llm` with a `TaskRouter` that selects providers based on task complexity. Simple tasks (commit message generation, PR comment formatting) route to the cheapest available provider. Complex tasks (multi-file patch generation, constraint analysis) route to the most capable.

Provider capability matrix maintained in `.but-ai/providers.toml`:

```toml
[providers.anthropic]
tool_calling = true
structured_output = true
max_context = 200000
cost_per_1k_input = 0.003

[providers.ollama-local]
tool_calling = true
structured_output = false
max_context = 8000
cost_per_1k_input = 0.0
```

Routing algorithm: for each task phase, select the cheapest provider whose capabilities meet the phase requirements. If no provider meets requirements, degrade gracefully (e.g., skip structured output and parse free text).

New providers: shared library adapters via `libloading`. Minimal interface. We ship adapters for OpenAI, Anthropic, Ollama, and LMStudio out of the box.

## 3. The But Agent (RFP 3.3)

Agent loop: **scan** (workspace state via `GetProjectStatus`, memory via cache lookup) -> **plan** (task decomposition, constraint identification, budget allocation) -> **build** (generate INDEX.patch as unified diff) -> **verify** (apply patch in shadow worktree, run constraint check) -> **commit** (produce COMMIT.msg, pass to signing).

The verify step is mandatory. Every patch is applied to a shadow worktree and checked against project constraints before it is finalized. Patches that fail verification are regenerated with the failure context injected (consuming additional budget). If regeneration also fails, the agent produces a `FAILED` COMMIT.msg with diagnostics.

Branch naming: `zm/<role>/s<NN>` with dot-encoded dependencies. Short prefix because we create a lot of branches and long names are annoying in `git log --oneline`.

Budget enforcement: Tyler's router checks remaining budget before each phase. If budget is insufficient for the next phase, the agent skips directly to COMMIT.msg with an `INCOMPLETE` flag and a description of remaining work.

## 4. Polyrepo PR Coordination (RFP 3.4)

Forge adapter:

```
trait ForgeAdapter {
    fn create_pr(&self, repo: &RepoRef, spec: &PrSpec) -> Result<PrId>;
    fn comment(&self, pr: &PrId, msg: &AgentMessage) -> Result<CommentId>;
    fn list_comments(&self, pr: &PrId) -> Result<Vec<AgentMessage>>;
    fn status(&self, pr: &PrId) -> Result<PrStatus>;
    fn cross_ref(&self, ref_str: &str) -> Result<PrId>;
}
```

GitHub reference implementation (REST API). We also ship a Gitea adapter because two of our enterprise customers run Gitea internally.

PR comment schema is minimal — we spent too many sprint cycles in 2024 on over-designed message schemas and learned the hard way. Required fields: `type`, `from`, `body`, `signature`. Optional: `customer_context`, `constraint_violations`. All other coordination state is derived from branch naming and PR labels.

Cross-repo: `org/repo#N` reference format. Resolved through forge adapter. Dependency tracking through PR labels (`depends-on: org/repo#N`) parsed by the coordination agent.

## 5. Agent Memory and Identity (RFP 3.5)

Memory: TTL-backed key-value store under `refs/zm/memory/<agent>/`. No fancy relevance scoring. No semantic search. Just keys, values, TTLs, and LRU eviction when the store exceeds a configurable size limit (default: 1000 entries).

Tyler's rationale: every memory system we have seen in production eventually becomes a liability. Entries accumulate, relevance scoring drifts, and agents start hallucinating based on stale context. A simple cache with aggressive eviction is boring but predictable. We can always add sophistication later; we cannot easily remove it.

Memory format: JSON blob per entry. Fields: `key`, `value`, `ttl_seconds`, `created_at`, `access_count`, `last_accessed`. Eviction: entries past TTL are garbage-collected on next access. LRU eviction triggers when entry count exceeds limit.

Compaction: only entries with `access_count > 5` survive compaction. Frequently accessed entries are valuable; everything else is noise.

Identity: stored at `refs/zm/identity/<agent>`. Minimal: name, role, branch authorization pattern, OpenWallet key fingerprint. No biography, no personality, no publication list. Identity is authorization, not narrative.

## 6. Signed Commits via OpenWallet (RFP 3.6)

Ed25519 keys provisioned at agent creation via `but ai agent init`. Every commit signed. No exceptions.

Signing flow: INDEX.patch + COMMIT.msg -> commit object construction -> OpenWallet Ed25519 signature -> signature in commit header -> push.

Authorization at `.but-ai/policy.toml`:

```toml
[agents.diego]
branches = ["zm/patch/*", "feat/*"]
max_patch_lines = 800

[agents.aisha]
branches = ["zm/forge/*", "coord/*"]
max_patch_lines = 200
```

Key rotation: 14-day cycle (faster than most proposals because we deploy frequently). Compromise revocation: immediate key invalidation, all post-compromise commits flagged, incident logged to memory with no TTL.

## 7. Token Budget

| Component | Input | Output | Frequency | Notes |
|-----------|-------|--------|-----------|-------|
| System prompt | 2,800 | 0 | Once/session | Agent identity, 10 tools, constraint config |
| Task ingestion | 2,000 | 400 | Once/task | PR body, branch metadata |
| Memory lookup | 400 | 100 | 2/task | Cache hit/miss, key lookup |
| Planning | 1,500 | 800 | Once/task | Decomposition, constraint identification |
| Tool calls (per call) | 1,000 | 500 | ~5/task | Parameter + result |
| Patch generation | 3,000 | 5,000 | Once/task | Context + diff |
| Verification | 1,500 | 300 | Once/task | Constraint check result processing |
| Commit message | 500 | 200 | Once/task | Conventional commit |
| Coordination | 800 | 300 | 1/task | PR comment |
| **TOTAL (typical task)** | **17,500** | **10,100** | -- | 200-line, 3 files |

## Unique Insight

From two years of compiling zoning codes, we learned that regulatory systems fail not because individual rules are wrong but because the interaction between rules is untested. A setback rule and a FAR rule can each be reasonable in isolation and impossible to satisfy simultaneously. The same is true for multi-agent systems: individual agents can be correct and their interactions can still produce failures. Our mandatory verification step — applying every patch to a shadow worktree and running constraint checks before commit — is not about code quality. It is about testing the interaction between an agent's output and the system's existing state. You do not find interaction failures by reviewing the patch in isolation. You find them by applying it.

---

*"Compile the code. Compile the zoning code. Same discipline."*
