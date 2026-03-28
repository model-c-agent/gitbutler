# Concrete & Daydream — Proposal

**Response to: `but ai` Plugin RFP v1.0.0**

---

## 1. Plugin Architecture (RFP 3.1)

`but-ai` as a PATH-discoverable binary, implemented in Rust, shipped as `crates/but-ai/`. Subcommands: `agent`, `memory`, `status`, `mcp`. The binary reads `BUT_WORKSPACE_DIR` and `BUT_OUTPUT_FORMAT` from environment and responds in JSON when `BUT_JSON=1`.

MCP mode: `but ai mcp` starts a server exposing `WorkspaceToolset` tools plus `AgentRun`, `MemoryQuery`, and `BudgetReport`. The server also exposes a `PlaceContext` tool unique to our implementation — it returns the geographic context (coordinates, jurisdiction, neighborhood name) associated with the current workspace, drawn from a `.but-ai/place.toml` configuration file.

WASI fallback: `but-ai-core` library crate linked statically into `but` when targeting `wasm32-wasip2`. Reduced mode — no plugin discovery, no subprocess execution. Core agent loop and memory retrieval remain functional. Provider access via WASI HTTP.

We considered a pure MCP architecture with no CLI subcommands. Rejected: CLI subcommands are essential for scripting and for community members who interact with agents through terminal sessions during participatory design workshops.

## 2. Provider-Agnostic AI Interface (RFP 3.2)

Wrapping `but-llm` with a `CommunityProvider` trait that adds: token cost tracking in human-readable units (not just token counts but estimated cost in EUR, because our funding is grant-based and we report costs to funders), capability detection, and a "plain language" mode that post-processes LLM outputs into non-technical language when the audience is a community meeting.

Provider selection is task-driven. Creative tasks (generating spatial descriptions from community input) go to the most capable provider regardless of cost. Mechanical tasks (formatting PR comments, generating commit messages) go to the cheapest provider. Beatriz manages this routing via a cost matrix updated monthly.

New providers via shared library adapters (`libloading`). Interface: `fn complete(prompt, tools, config) -> Result<Response>`. No WASM sandboxing.

## 3. The But Agent (RFP 3.3)

Agent loop: **listen** (read task, read place context, retrieve spatial memory) -> **imagine** (decompose task into layers, estimate budget) -> **make** (generate INDEX.patch, layer by layer) -> **name** (generate COMMIT.msg in both technical and plain-language variants) -> **check** (apply patch to shadow worktree, validate).

The dual commit message is our signature feature. Every COMMIT.msg contains two sections: a conventional commit line for developers and a plain-language paragraph for community stakeholders. The plain-language section is generated in a separate, cheaper LLM call and appended as a commit trailer (`Community-Summary: ...`).

Branch naming: `c+d/<agent-name>/s<NN>` with dot-notation dependencies. The `c+d/` prefix is deliberately informal.

Budget enforcement: Beatriz's budget gate runs before patch generation. If remaining budget is insufficient for a full patch, the agent produces a sketch — a structured outline of the intended change with enough detail for a human to implement it manually.

## 4. Polyrepo PR Coordination (RFP 3.4)

Forge adapter with place-awareness:

```
trait ForgeAdapter {
    fn create_pr(&self, repo: &RepoRef, pr: &PrSpec) -> Result<PrId>;
    fn post_comment(&self, pr: &PrId, msg: &PlaceMessage) -> Result<CommentId>;
    fn list_comments(&self, pr: &PrId) -> Result<Vec<PlaceMessage>>;
    fn get_status(&self, pr: &PrId) -> Result<PrStatus>;
    fn resolve_reference(&self, ref_str: &str) -> Result<PrId>;
}
```

`PlaceMessage` extends the standard agent message schema with mandatory `coordinates` and `neighborhood` fields. Every coordination message is anchored to a physical location.

Cross-repo coordination: PR comments as message bus with exponential-backoff polling. For participatory design projects, we add a "community thread" — a human-readable summary comment pinned to the top of each PR, updated by Nadia's agent whenever the technical thread progresses.

## 5. Agent Memory and Identity (RFP 3.5)

Memory stored under `refs/c+d/memory/<agent-name>/` as Git blobs. Each entry is a JSON document with: `key`, `narrative` (plain-language description), `tags`, `coordinates`, `mood` (Kwame's emotional taxonomy: tense, generous, forgotten, loud-empty), `created`, `ttl`, `relevance`.

Retrieval: dual-path. Technical queries match against `tags` and `coordinates`. Narrative queries use semantic similarity against the `narrative` field. The two paths can be combined: "find memories about parks that felt generous."

Relevance model: memories decay linearly, but decay rate is modulated by mood. "Generous" memories decay slowly (they represent positive patterns worth preserving). "Tense" memories decay quickly (they represent conflicts that should not linger). "Forgotten" memories have accelerated decay — they are tagged as forgotten because someone already judged them as fading.

Compaction: narrative summaries survive; raw data entries do not. The narrative is the memory; the data is evidence that can be re-retrieved from source.

Identity: stored at `refs/c+d/identity/<name>`. Includes role, medium (art practice), place affiliation (which neighborhoods this agent has worked in), and OpenWallet key.

## 6. Signed Commits via OpenWallet (RFP 3.6)

Henri's signing flow: patch produced -> review complete -> provenance note assembled (author, reviewer, physical location, place context) -> commit object constructed with provenance trailers -> Ed25519 signature via OpenWallet -> signed commit finalized.

Authorization policy at `.but-ai/policy.toml`:

```toml
[agents.lior]
branches = ["c+d/lior/*", "feat/*"]
max_patch_lines = 600

[agents.nadia]
branches = ["c+d/nadia/*", "coord/*"]
max_patch_lines = 300
```

Key rotation: 45-day cycle (longer than default because the residency's pace is slower). Revocation triggers re-review of all affected commits and a community notification if the commits were part of a participatory design project.

## 7. Token Budget

| Component | Input | Output | Frequency | Notes |
|-----------|-------|--------|-----------|-------|
| System prompt | 3,200 | 0 | Once/session | Identity, tools, place context |
| Task ingestion | 2,000 | 300 | Once/task | PR body, place metadata |
| Memory retrieval | 1,800 | 600 | 2/task | Narrative + spatial query |
| Planning | 1,000 | 500 | Once/task | Layer decomposition, budget check |
| Tool calls (per call) | 900 | 400 | ~4/task | Parameter + result processing |
| Patch generation | 3,500 | 5,500 | Once/task | Layered spatial model diff |
| Commit message (dual) | 800 | 600 | Once/task | Technical + plain-language variant |
| Coordination | 1,200 | 500 | 1/task | Place-anchored PR comment |
| **TOTAL (typical task)** | **18,100** | **10,800** | -- | 200-line feature, 3 files |

## Unique Insight

From eight years of participatory design, we have learned that the most powerful feedback does not come from people who understand the plan — it comes from people who experience the place. A resident who says "this corner feels unsafe at night" is providing data that no sensor can replicate. Our dual commit message system is not a cosmetic feature. It is an interface between technical version control and lived experience. When a community member can read a commit message and understand what changed in their neighborhood, version control becomes democratic infrastructure.

---

*"The best plans are written by the people who walk the street."*
