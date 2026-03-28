# PastStack — Proposal

**Response to: `but ai` Plugin RFP v1.0.0**

---

## 1. Plugin Architecture (RFP 3.1)

Rust crate at `crates/but-ai/`. Lean. We want sub-100ms cold start for CLI mode because our agents run hundreds of tasks per day and startup latency compounds.

CLI subcommands: `run` (execute task), `scan` (read-only workspace analysis), `memory`, `budget` (show remaining tokens), `mcp`.

MCP mode: backward-compatible drop-in. All `WorkspaceToolset` tools plus `RunTask`, `ScanWorkspace`, `BudgetReport`. The MCP server starts in under 200ms.

**WASI:** We compile a `but-ai-core` library that can be linked into the `but` binary for WASI builds. Under WASI, the agent runs with a restricted tool set (no fork/exec, no forge APIs) but can still generate patches using a local provider. We test WASI builds in CI.

## 2. Provider-Agnostic AI Interface (RFP 3.2)

We use `but-llm` with an optimization layer. The ML agent wraps `but-llm` calls with:
- **Token counting:** Pre-count input tokens before sending to the provider. If the input would exceed the model's context window, truncate intelligently (oldest context first, never truncate the system prompt).
- **Capability routing:** Check if the provider supports tool calling. If not, fall back to text-based tool invocation with JSON parsing.
- **Cost tracking:** Log the token cost of every call for budget enforcement.

New providers: add a TOML entry mapping the provider name to its API URL and format. If it speaks OpenAI-compatible API (most do), zero code changes. We register Gemini and Mistral day one because our customers ask for them.

## 3. The But Agent (RFP 3.3)

Fast loop:

1. **Scan** — Read task + memory + workspace state. Produce context package.
2. **Plan** — One-shot planning: decompose task into tool calls and a patch outline. No iteration.
3. **Execute** — Run tool calls, gather data, generate `INDEX.patch`.
4. **Ship** — Generate `COMMIT.msg`, sign commit, report budget.

No self-review loop. We rely on the DevOps agent's CI integration to catch issues post-commit. This trades per-patch quality for throughput — and in our experience, the throughput win is larger than the quality loss because most patches are correct on the first attempt.

**Branch naming:** `ps/<task-id>`. Short. No encoded metadata — we store metadata in memory, not in branch names.

**Budget enforcement:** Hard ceiling. The ML agent tracks tokens per call and terminates the agent at 90% budget, reserving 10% for patch generation and commit. No graceful degradation modes — either you finish or you don't.

## 4. Polyrepo PR Coordination (RFP 3.4)

**Forge adapter:**

```rust
trait Forge {
    fn pr(&self, repo: &str, spec: PrSpec) -> Result<u64>;
    fn comment(&self, repo: &str, pr: u64, msg: &str) -> Result<()>;
    fn comments(&self, repo: &str, pr: u64) -> Result<Vec<String>>;
}
```

Three methods. GitHub first. GitLab when a customer asks. We implement the minimum viable forge adapter and extend it when real usage demands it.

**PR comment schema:** JSON-LD inspired, but kept minimal:

```json
{"@type":"ps:status","agent":"ps/backend-1","task":"T042","state":"done","confidence":0.92,"tokens":{"used":18000,"budget":25000}}
```

The `confidence` field is borrowed from our LiDAR pipeline. It indicates how confident the agent is that its patch is complete and correct. Below 0.8, the PR is automatically flagged for human review.

**Cross-repo references:** Standard `owner/repo#N` format. The product agent maintains a dependency graph in memory, updated after each coordination event.

## 5. Agent Memory and Identity (RFP 3.5)

**Storage:** Lightweight branch `ps/memory`. Flat file structure: `<agent-id>/<key>.json`. No nested taxonomy. We tried hierarchical storage and it slowed lookups. Flat is fast.

**Relevance scoring:** We borrow from our LiDAR feature detection pipeline. Each memory entry has an "activation vector" — a list of keywords with weights. The current task has a "query vector." Relevance is the dot product of the two vectors. Above threshold: retrieved. Below: ignored. The threshold self-adjusts based on retrieval volume (too many hits → raise threshold, too few → lower it).

**TTL:** Default 7 days. Extended to 30 days for entries accessed more than 3 times (indicating they are genuinely useful). Entries that survive two TTL extensions become "permanent" until manually expired.

**Compaction survival:** Each agent has a "field card" — a compressed summary of its identity and the current project context. Field cards are regenerated at each compaction event by the data agent.

**Identity:** JSON file at `ps/identity/<agent-id>.json`. Contains: name, role, capabilities, branch permissions, public key fingerprint. Signed at creation by the DevOps agent.

## 6. Signed Commits via OpenWallet (RFP 3.6)

The DevOps agent manages all signing operations. Each implementing agent produces an unsigned patch; the DevOps agent signs on commit.

**Authorization:** Stored in `.but-ai/access.toml`:

```toml
[[agents]]
name = "backend-1"
branches = ["ps/*", "feat/*"]
max_lines = 1000

[[agents]]
name = "devops"
branches = ["ps/*", "infra/*"]
can_sign = true
```

**Key lifecycle:** Automated. Keys provisioned at `but ai agent create`. Rotated every 30 days via a cron-triggered `but ai agent rotate-keys`. Compromise: `but ai agent revoke --agent <name> --reason compromise`, which flags all commits and notifies via forge API.

## 7. Token Budget

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| System prompt | 2,500 | 0 | Once/session | Lean: roles + tools only |
| Task ingestion | 1,800 | 200 | Once/task | Fast scan |
| Planning | 800 | 400 | Once/task | One-shot, no iteration |
| Tool call (per call) | 1,000 | 400 | ~5/task | Quick calls, minimal retries |
| Patch generation | 2,500 | 4,500 | Once/task | Core output |
| Commit message | 400 | 200 | Once/task | Brief conventional format |
| Memory retrieval | 400 | 100 | 1/task | Dot product scoring |
| Coordination event | 800 | 300 | 1/task | JSON-LD comment |
| **TOTAL (typical task)** | **13,200** | **8,100** | -- | Optimized for throughput |

## Unique Insight

In jungle archaeology, the biggest waste of time is not searching the wrong place — it is searching the right place with the wrong resolution. A LiDAR survey at 1-meter resolution misses a wall that a 10-centimeter survey reveals. But a 10-centimeter survey of 1,000 km2 costs 10x more than a 1-meter survey.

The solution is adaptive resolution: scan the whole area at low resolution first, identify high-probability zones, then re-scan those zones at high resolution. We apply this to token budgets. The agent's first pass uses minimal context (low resolution) to identify which files and which code sections are relevant. Then it re-reads only those sections in detail (high resolution). This "adaptive context" approach reduces total input tokens by 30-40% compared to reading everything upfront.

---

*"Finding things fast is not the same as finding things carelessly."*
