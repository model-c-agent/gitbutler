# Kowalski Planning Group — Proposal

**Response to: `but ai` Plugin RFP v1.0.0**

---

## 1. Plugin Architecture (RFP 3.1)

`but-ai` as a PATH binary in `crates/but-ai/`. Must run on the firm's existing hardware (one Mac Studio, three Windows laptops, and a Linux server that Tomasz built from parts). Binary size matters — the Windows laptops have spinning hard drives.

Subcommands: `agent`, `memory`, `status`, `mcp`. We add `but ai precedent <address>` — a convenience command that retrieves the three most similar historical projects for a given site address, leveraging Henryk's memory system. This is the single most valuable command for the firm's daily workflow.

Environment: `BUT_WORKSPACE_DIR`, `BUT_OUTPUT_FORMAT`, `BUT_JSON`.

WASI fallback: library mode via `but-ai-core`. The firm does not currently use WASI but Marta is interested in browser-based site analysis tools for client presentations.

MCP mode: standard tools plus `PrecedentQuery`, `SiteAnalysis`, `DensityCalc`.

## 2. Provider-Agnostic AI Interface (RFP 3.2)

Provider abstraction over `but-llm`. The firm uses Anthropic (Marta's choice) for complex analysis and Ollama running locally (Tomasz's setup) for routine tasks. The provider selection is manual — Marta decides which provider handles which project based on complexity and client sensitivity.

For client-sensitive projects (where site data should not leave the firm's network), all tasks route to the local Ollama instance regardless of capability. Tomasz configured this as a `local_only` flag in `.but-ai/providers.toml`.

Capability detection: tool calling and structured output support. If the local provider lacks a capability, the task falls back to structured prompting rather than routing to an external provider — client data sovereignty takes priority over task quality.

## 3. The But Agent (RFP 3.3)

Agent loop: **assess** (read task, query precedent memory, identify site constraints) -> **plan** (decompose into site analysis phases, allocate budget per phase) -> **generate** (produce INDEX.patch with site metrics in commit trailers) -> **review** (Henryk precedent check, June client-context check, Peter approval) -> **finalize** (COMMIT.msg with project number, zoning references, density calculation).

Commit trailers: `Project-No:`, `Zoning-District:`, `Density-DUA:` (dwelling units per acre), `Transit-Proximity:` (distance to nearest transit stop in meters). These map directly to the firm's project database fields.

Branch naming: `kpg/<project-number>/s<NN>`. Project numbers follow the firm's existing numbering system (year-sequence, e.g., `2026-014`).

Budget enforcement: Peter approves task-level budgets before work begins. If a task would exceed the approved budget, it pauses at the current phase and produces a partial result with an `OVER-BUDGET: approval required for continuation` flag.

## 4. Polyrepo PR Coordination (RFP 3.4)

Forge adapter:

```
trait ForgeAdapter {
    fn create_pr(&self, repo: &RepoRef, spec: &PrSpec) -> Result<PrId>;
    fn post_comment(&self, pr: &PrId, msg: &ProjectMessage) -> Result<CommentId>;
    fn list_comments(&self, pr: &PrId) -> Result<Vec<ProjectMessage>>;
    fn status(&self, pr: &PrId) -> Result<PrStatus>;
    fn cross_reference(&self, ref_str: &str) -> Result<PrId>;
}
```

PR comment schema includes `project_number`, `client_status` (one-line client-safe summary), `phase`, `from`, `body`, `signature`. June maintains the client-facing narrative via a pinned PR comment updated after each significant change.

GitHub implementation. The firm uses GitHub for all projects. Cross-repo coordination is rare — most projects are single-repository — but when it occurs (multi-phase developments with separate repos per phase), June manages the cross-references manually with agent assistance.

## 5. Agent Memory and Identity (RFP 3.5)

Memory under `refs/kpg/memory/<agent>/`. The crown jewel: Henryk's project precedent database.

Each project memory entry: `project_number`, `address`, `jurisdiction`, `zoning_district`, `density_dua`, `unit_count`, `unit_mix` (% studio/1br/2br/3br), `transit_distance_m`, `outcome` (approved/denied/modified), `approval_date`, `lessons` (free text), `tags`.

Retrieval: weighted similarity search across five dimensions — zoning district (exact match, weight 3), density (numeric proximity, weight 2), transit distance (numeric proximity, weight 2), jurisdiction (exact match, weight 1), unit count (numeric proximity, weight 1). Returns top 3 matches.

No relevance decay. Project precedent does not expire. A project from 1965 in the same zoning district is still relevant — the zoning code may have changed, but the district boundaries often have not.

Compaction: project summaries survive. Detailed site analysis data does not (too verbose for compacted context). The summary is sufficient for precedent matching; detailed data can be re-retrieved from the project repository.

Identity: `refs/kpg/identity/<agent>`. Name, role, tenure (years at firm), authorization level, project assignment list, OpenWallet key.

## 6. Signed Commits via OpenWallet (RFP 3.6)

All commits signed. Tomasz's workflow: verify agent authorization, check that project number in COMMIT.msg matches the agent's assignment list, sign with Ed25519 via OpenWallet.

Authorization:

```toml
[agents.marta]
branches = ["kpg/*/draft", "feat/*"]
max_patch_lines = 600

[agents.henryk]
branches = ["kpg/*/review"]
max_patch_lines = 100

[agents.june]
branches = ["kpg/*/coord"]
max_patch_lines = 200
```

Key rotation: 30-day default. Peter asked for 90-day rotation to reduce disruption; Tomasz compromised at 30 with automated rotation scripts.

## 7. Token Budget

| Component | Input | Output | Frequency | Notes |
|-----------|-------|--------|-----------|-------|
| System prompt | 3,000 | 0 | Once/session | Identity, tools, project context |
| Task ingestion | 2,000 | 300 | Once/task | Site address, project brief |
| Precedent retrieval | 2,000 | 500 | 1/task | Top-3 similarity search |
| Site analysis | 1,500 | 800 | Once/task | Constraint identification |
| Tool calls (per call) | 1,000 | 500 | ~4/task | Parameter + result |
| Patch generation | 3,500 | 5,000 | Once/task | Context + diff with site metrics |
| Commit message | 700 | 400 | Once/task | Project number, zoning, density |
| Client review | 1,000 | 300 | 1/task | June's client-context check |
| Authorization | 500 | 100 | 1/task | Peter's approval |
| **TOTAL (typical task)** | **19,200** | **10,400** | -- | 200-line, 3 files, 1 site |

## Unique Insight

Seventy years of project data has taught us something counterintuitive: the best predictor of a project's success is not its design quality but its similarity to projects that have already been approved in the same jurisdiction. Planning commissions are precedent machines. They approve what looks familiar and reject what looks novel, regardless of merit. Our precedent-based memory system is not just a convenience — it is a risk management tool. An agent that proposes a density its jurisdiction has never approved is proposing a project that will likely be denied, no matter how well-designed it is. Memory is not history. Memory is strategy.

---

*"The best site plan is one the commission has seen before — improved."*
