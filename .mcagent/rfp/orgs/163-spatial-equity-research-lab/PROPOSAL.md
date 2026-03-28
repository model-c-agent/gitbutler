# Spatial Equity Research Lab — Proposal

**Response to: `but ai` Plugin RFP v1.0.0**

---

## 1. Plugin Architecture (RFP 3.1)

We propose `but-ai` as a standalone Rust binary installed to PATH, structured as a workspace crate at `crates/but-ai/`. The binary responds to `but ai <subcommand>` via the existing plugin discovery mechanism and exposes MCP server mode via `but ai mcp`.

Environment variables consumed: `BUT_WORKSPACE_DIR`, `BUT_OUTPUT_FORMAT`, `BUT_JSON`. Subcommands: `agent` (autonomous task execution), `memory` (spatial memory query/store), `status` (agent health, budget remaining), `mcp` (start MCP server for IDE integration).

Under WASI, the plugin falls back to library mode — `but-ai-core` is linked statically into the `but` binary. Plugin discovery is disabled. The agent operates in reduced mode: no subprocess spawning, no filesystem plugin scanning, but core patch generation and memory retrieval remain functional via WASI HTTP for provider access.

We considered embedding the AI functionality directly in the `but` binary without a plugin boundary. Rejected: the plugin boundary provides a clean compilation firewall and allows `but-ai` to evolve on a separate release cadence.

## 2. Provider-Agnostic AI Interface (RFP 3.2)

We wrap the existing `but-llm` crate with an `AgentProvider` trait that adds three concerns absent from the raw LLM interface: per-call token accounting, capability detection (tool calling support, structured output support), and graceful degradation when a provider lacks a required capability.

Provider registration: each provider implements a `ProviderProfile` struct declaring capabilities, rate limits, and cost-per-token. The agent selects providers dynamically based on task requirements and remaining budget — expensive reasoning tasks go to capable providers, cheap formatting tasks go to fast providers.

New providers are added as shared library adapters loaded via `libloading`. The adapter interface is minimal: `fn complete(prompt, tools, config) -> Result<Response>`. We explicitly avoid WASM-sandboxed provider plugins; the overhead is unjustified for HTTP client wrappers.

Trade-off: dynamic linking introduces platform-specific concerns (.so vs .dylib vs .dll). We accept this because the alternative — compiling all providers into the binary — defeats the plugin architecture's purpose.

## 3. The But Agent (RFP 3.3)

The agent loop: **survey** (read workspace state via `GetProjectStatus`, read memory via spatial query) -> **scope** (estimate token cost, check budget, decide full-patch vs. abstract mode) -> **generate** (produce INDEX.patch as unified diff) -> **document** (produce COMMIT.msg in a separate, cheaper call) -> **validate** (apply patch to shadow worktree, run syntax check).

Branch naming convention: `serl/<agent-role>/s<NN>` with dependency encoding via dot notation (`serl/patch/s01.s03`). The `serl/` prefix allows coexistence with branches from other agent teams in the same repository.

Our agents produce patches with inline comments that reference the spatial data justifying each change. This is unusual — most teams treat patches as pure code artifacts. We treat them as evidentiary documents. Every hunk in an INDEX.patch includes a trailer line citing the data source: `Spatial-Source: census-tract/E02001234, ONS-2021`.

Budget enforcement: the scoping phase produces a token estimate with 20% margin. If remaining budget cannot cover the estimate, the agent enters abstract mode — producing a structured description of the intended change rather than attempting generation. Abstract mode output is valid COMMIT.msg content tagged `ABSTRACT: manual implementation required`.

## 4. Polyrepo PR Coordination (RFP 3.4)

Forge adapter trait:

```
trait ForgeAdapter {
    fn create_pr(&self, repo: &RepoRef, pr: &PrSpec) -> Result<PrId>;
    fn post_comment(&self, pr: &PrId, msg: &AgentMessage) -> Result<CommentId>;
    fn list_comments(&self, pr: &PrId, filter: &MessageFilter) -> Result<Vec<AgentMessage>>;
    fn get_status(&self, pr: &PrId) -> Result<PrStatus>;
    fn cross_reference(&self, local: &PrId, remote: &str) -> Result<PrId>;
}
```

Reference implementation for GitHub REST API. The adapter is stateless; all coordination state lives in PR comments using a structured schema with mandatory fields: `jurisdiction` (which repo/context), `data_vintage` (when the referenced data was current), `confidence` (0.0-1.0), and `signature`.

Cross-repo coordination uses the PR comment thread as a message bus. Each agent posts structured messages; other agents poll (with exponential backoff) for messages addressed to them. We considered webhooks but rejected them — they require server infrastructure, and the lab does not maintain servers.

## 5. Agent Memory and Identity (RFP 3.5)

Memory is stored under `refs/serl/memory/<agent-name>/` as Git blobs. The distinguishing feature of our memory system is spatial indexing: every memory entry includes a GeoJSON bounding box. Retrieval supports both keyword queries and spatial queries (point-in-polygon, bounding-box intersection).

Storage format: each entry is a JSON blob with fields `key`, `value`, `bbox` (GeoJSON), `jurisdiction`, `tags`, `created`, `ttl`, `relevance`. The memory index is a Git tree mapping keys to blob SHAs, with a secondary spatial index stored as an R-tree serialized to a separate blob.

Relevance decay: linear decay with jurisdiction-aware boosting. Memories about the jurisdiction currently being analyzed receive a 2x relevance multiplier. Memories from adjacent jurisdictions receive 1.5x. This ensures that spatial context remains available when an agent crosses a municipal boundary.

Compaction survival: memories tagged `methodological` (general analysis patterns) survive compaction unconditionally. Memories tagged `jurisdictional` (specific to one place) survive only if the current task involves that jurisdiction.

Identity: stored at `refs/serl/identity/<agent-name>`. Includes role, publication list (yes, really — each agent has a simulated publication record that establishes its methodological authority), authorization scope, and OpenWallet key fingerprint.

## 6. Signed Commits via OpenWallet (RFP 3.6)

Signing flow: agent produces INDEX.patch + COMMIT.msg -> orchestrator constructs commit object -> commit passed to agent's OpenWallet signer -> Ed25519 signature embedded in commit headers -> verification resolves public key via OpenWallet and checks authorization scope.

Authorization policy in `.but-ai/policy.toml`:

```toml
[agents.adaeze]
branches = ["serl/memory/*"]
max_patch_lines = 200

[agents.priya]
branches = ["serl/patch/*", "feat/*"]
max_patch_lines = 500

[agents.sam]
branches = ["serl/identity/*"]
max_patch_lines = 100
```

Key rotation: 30-day default. Compromise revocation flags all commits signed after the suspected compromise timestamp for re-review. Revocation events are stored as memory entries tagged `security-incident` with no TTL (permanent).

## 7. Token Budget

| Component | Input | Output | Frequency | Notes |
|-----------|-------|--------|-----------|-------|
| System prompt | 3,000 | 0 | Once/session | Agent identity, 10 tools, spatial context preamble |
| Task ingestion | 2,500 | 400 | Once/task | PR body, branch metadata, jurisdiction identification |
| Memory retrieval (spatial) | 2,000 | 500 | 2/task | Bounding-box query + R-tree traversal + injection |
| Planning/scoping | 1,200 | 600 | Once/task | Dependency graph, budget estimate |
| Tool calls (avg per call) | 1,000 | 500 | ~4/task | Parameter formulation + result processing |
| Patch generation | 3,500 | 5,000 | Once/task | Context + unified diff with spatial source citations |
| Commit message | 600 | 300 | Once/task | Conventional commit + spatial source trailers |
| Coordination | 1,200 | 400 | 1/task | PR comment exchange |
| **TOTAL (typical task)** | **19,000** | **10,200** | -- | 200-line feature, 3 files, 1 jurisdiction |

## Unique Insight

From fifteen years of spatial analysis, we have learned that the most dangerous bias is the one encoded in the coordinate system. A dataset projected in a system that distorts area near the equator will systematically undercount land in tropical nations. The same principle applies to agent memory: the indexing scheme determines what is findable. If memory is indexed only by keyword, spatial relationships are invisible. Our spatial memory indexing ensures that agents can ask "what do we know about *this place*" — not just "what do we know about this keyword." Geography is not metadata. It is structure.

---

*"The projection is the politics."*
