# Bowline Ventures — Proposal

**Response to: `but ai` Plugin RFP v1.0.0**

---

## 1. Plugin Architecture (RFP 3.1)

Ship it simple. Single Rust binary, `crates/but-ai/`, no sub-crates. CLI mode and MCP mode in the same binary, switched by the first argument. `but ai mcp` starts the server; everything else is CLI.

We lean into the environment contract: `BUT_WORKSPACE_DIR` is the source of truth for workspace location. No additional config parsing needed for basic operation.

Subcommands: `agent run <task>`, `agent status`, `memory get <key>`, `memory set <key> <value>`, `budget`, `mcp`.

**WASI:** We don't over-engineer the WASI story. Under WASI, `but-ai` is unavailable as a plugin. We provide a `but-ai-lite` crate that can be linked into the main binary for WASI builds, exposing a reduced API (patch generation and memory queries only, no forge coordination, no MCP server). Ship what works; iterate later.

## 2. Provider-Agnostic AI Interface (RFP 3.2)

We use `but-llm` directly. No wrapper layer unless we need one. Our agents call `tool_calling_loop` for standard tasks and `tool_calling_loop_stream` for long-running operations where we need progress updates.

For new providers, we define a config-driven adapter: a TOML file maps provider names to endpoint URLs and authentication schemes. If the provider speaks OpenAI-compatible API (which most do), no code changes needed — just add the config entry. For providers with non-standard APIs, we write a thin Rust shim crate.

**Trade-off:** We considered a full plugin architecture for providers. Rejected — YAGNI. The four existing providers cover 95% of use cases. We'll build the plugin system when the fifth provider actually shows up.

## 3. The But Agent (RFP 3.3)

The agent loop is dead simple:

1. Read task → 2. Retrieve memory → 3. Plan (internally, no user output) → 4. Execute tool calls → 5. Generate `INDEX.patch` → 6. Generate `COMMIT.msg` → 7. Report budget

No self-review pass. No crit cycle. Ship it and let the human reviewer catch issues. We're optimizing for throughput, not perfection. A 90%-correct patch shipped in 30 seconds beats a 99%-correct patch shipped in 5 minutes, because the human review is happening anyway.

**Branch naming:** `bv/<task-id>/<deps>`. Short, scannable. Example: `bv/T042/s01`.

**Budget enforcement:** Hard kill at 95% budget. The agent produces whatever partial result it has with a `BUDGET_LIMIT` flag. No graceful degradation modes, no "abstract mode." Either you have budget or you don't.

## 4. Polyrepo PR Coordination (RFP 3.4)

**Forge adapter:**

```rust
trait Forge: Send + Sync {
    fn create_pr(&self, repo: &str, title: &str, body: &str, head: &str, base: &str) -> Result<u64>;
    fn comment(&self, repo: &str, pr_id: u64, body: &str) -> Result<u64>;
    fn get_comments(&self, repo: &str, pr_id: u64) -> Result<Vec<Comment>>;
    fn add_label(&self, repo: &str, pr_id: u64, label: &str) -> Result<()>;
}
```

Four methods. That's it. We implement GitHub first. GitLab and Bitbucket later, when customers ask for them.

**PR comment schema:** JSON in a code fence. No fancy markdown wrapping. Machines parse the JSON; humans read the PR body and diff.

```json
{"v":1,"from":"bv/fullstack-1","type":"status","task":"T042","state":"done","tokens":{"used":22000,"max":35000}}
```

One line. Parseable. Done.

**Cross-repo references:** We use the standard `owner/repo#123` format. The forge adapter resolves it. If the target repo is on a different forge, we store the full URL.

## 5. Agent Memory and Identity (RFP 3.5)

**Storage:** A branch named `bv/memory` in the repo. Memory entries are JSON files: `<agent-id>/<key>.json`. We keep it flat — no nested directories, no taxonomies. Grep works. `git log` works. Keep it simple.

**Relevance scoring:** TF-IDF on memory entry tags versus current task keywords. Nothing fancy. It works for 2M+ manifests/day; it'll work for agent memory. When a frontier model is in the loop, we can upgrade to embedding-based scoring, but we won't build that until the simple thing fails.

**TTL:** Each entry has a `ttl_seconds` field. A background sweep (triggered at agent startup) deletes expired entries and commits the deletion. No orphan entries.

**Compaction survival:** We tag critical memories with `"pin": true`. Pinned entries are always injected into compacted context. Unpinned entries are ranked by TF-IDF score. Top N entries (configurable, default 5) are included.

**Identity:** Agent identity is a JSON file at `bv/identity/<agent-id>.json`. Contains: name, role, public key fingerprint, authorized branches (glob patterns), created timestamp. Signed with the agent's own key at creation.

**Long-term storage:** A `bv/knowledge-base` branch shared across agents. Any agent can write; all agents can read. No approval gates for writes. If bad entries accumulate, we'll add gates later. Ship first.

## 6. Signed Commits via OpenWallet (RFP 3.6)

Every commit is signed. Period. The infra agent manages keys. The flow:

1. Agent produces unsigned `INDEX.patch` + `COMMIT.msg`
2. Orchestrator applies the patch
3. Orchestrator calls OpenWallet to sign the commit with the agent's key
4. Signed commit is finalized

**Authorization:** A TOML file at `.but-ai/auth.toml`:

```toml
[[agents]]
name = "fullstack-1"
branches = ["bv/*", "feat/*"]
max_lines = 1000

[[agents]]
name = "qa"
branches = ["bv/*"]
max_lines = 0  # QA doesn't produce patches
```

**Key lifecycle:** Keys created at `but ai agent init`. Rotated every 30 days via `but ai agent rotate-key`. Compromise revocation via `but ai agent revoke --reason compromise`, which marks all commits since last audit as suspect and posts a warning to all open PRs.

## 7. Token Budget

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| System prompt | 2,800 | 0 | Once/session | Minimal — tools + identity only |
| Task ingestion | 2,000 | 200 | Once/task | Read task, extract requirements |
| Planning | 800 | 400 | Once/task | Quick internal plan |
| Tool call (per call) | 1,000 | 400 | ~5/task | Standard tool interactions |
| Patch generation | 2,500 | 4,500 | Once/task | Read context, produce diff |
| Commit message | 500 | 200 | Once/task | Terse, conventional format |
| Memory retrieval | 400 | 100 | 1/task | Quick TF-IDF lookup |
| Coordination event | 800 | 300 | 1/task | Minimal JSON comment |
| **TOTAL (typical task)** | **14,800** | **8,100** | -- | Lean by design |

## Unique Insight

From two years of processing millions of shipping manifests, we learned that the most valuable system property is not accuracy — it's recovery speed. Our manifest system is 99.7% accurate. The 0.3% that's wrong gets caught and fixed within minutes because the system is designed for fast detection and rollback. We apply the same principle here: don't over-invest in preventing agent errors. Invest in detecting and reverting them quickly. A `but ai revert` that works in 3 seconds is worth more than a review process that adds 5 minutes to every commit.

---

*"Ship it. Fix it. Ship it again."*
