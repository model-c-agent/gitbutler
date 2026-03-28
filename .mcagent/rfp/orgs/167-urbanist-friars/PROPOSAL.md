# The Urbanist Friars — Proposal

**Response to: `but ai` Plugin RFP v1.0.0**

---

## 1. Plugin Architecture (RFP 3.1)

`but-ai` as a PATH-discoverable Rust binary in `crates/but-ai/`. The Friars run a mix of Linux laptops and Chromebooks, so cross-platform support and minimal binary size are essential.

Subcommands: `agent` (route analysis tasks), `memory` (waypoint knowledge query/store), `status` (budget, pending discernments), `mcp` (MCP server). Environment: `BUT_WORKSPACE_DIR`, `BUT_OUTPUT_FORMAT`, `BUT_JSON`.

We add `but ai walk` — a domain-specific subcommand that generates a structured route proposal from a start point, end point, and set of constraints (maximum grade, minimum sidewalk width, avoid highways). This wraps the standard agent loop with route-specific pre-processing.

WASI fallback: `but-ai-core` library linked statically. Reduced mode. The Friars would use WASI builds for field deployments where Lucia carries a tablet with limited connectivity and pre-downloaded map data.

MCP mode exposes standard tools plus `RoutePropose`, `WaypointQuery`, and `WalkabilityScore`.

## 2. Provider-Agnostic AI Interface (RFP 3.2)

Single-provider default. The Friars operate on grant budgets and cannot afford to maintain integrations with multiple providers simultaneously. The provider abstraction wraps `but-llm` and adds: cost tracking in Euros (mapped to grant line items), token accounting per task, and a "frugal mode" that reduces context window usage by 40% at the cost of lower patch quality.

Frugal mode is the default. The Friars switch to full mode only for complex route-planning tasks that Anna has pre-approved with a specific budget allocation.

Provider capability detection: minimal. The Friars check for tool calling support. If absent, the agent falls back to structured prompting. No dynamic provider switching — Anna selects the provider at the start of each funding period.

## 3. The But Agent (RFP 3.3)

Agent loop: **observe** (read task, retrieve waypoint memory, read workspace state) -> **discern** (identify applicable route segments, estimate budget, check constraints) -> **draft** (generate INDEX.patch as unified diff with geographic annotations) -> **review** (community impact assessment) -> **commit** (COMMIT.msg with route references and walkability scores).

Geographic annotations: every hunk in an INDEX.patch includes a trailer line `Waypoint: <lat>,<lon> | Score: <0-10> | Surface: <type>`. This makes diffs legible to the Friars, who think in geography rather than line numbers.

Branch naming: `friars/<city>/s<NN>` with dot-encoded dependencies. City names use the community's canonical form (Bologna, not IT-BO).

Budget enforcement: Anna gates every task. If budget is insufficient for full route analysis, the agent produces a "walking brief" — a structured outline of the proposed route with enough detail for a friar to survey it on foot, tagged `BRIEF: field verification required`.

## 4. Polyrepo PR Coordination (RFP 3.4)

Forge adapter:

```
trait ForgeAdapter {
    fn create_pr(&self, repo: &RepoRef, spec: &PrSpec) -> Result<PrId>;
    fn post_letter(&self, pr: &PrId, letter: &CommunityLetter) -> Result<CommentId>;
    fn list_letters(&self, pr: &PrId) -> Result<Vec<CommunityLetter>>;
    fn status(&self, pr: &PrId) -> Result<PrStatus>;
    fn cross_reference(&self, ref_str: &str) -> Result<PrId>;
}
```

PR comments are "community letters" — structured messages with: `from_city`, `to_city`, `subject`, `body`, `community_impact`, `signature`. The tone is epistolary. James writes coordination messages the way he writes letters to parish councils: with respect for the recipient's local knowledge.

Cross-repo: each city has its own repository. Route knowledge that generalizes (highway crossing techniques, river path patterns) is shared via letters between city repos. City-specific knowledge stays local.

## 5. Agent Memory and Identity (RFP 3.5)

Memory under `refs/friars/memory/<agent>/` as Git blobs. Structured as a waypoint database.

Each entry: `coordinates` (lat/lon), `route_id`, `segment_id`, `observation` (what was seen), `walkability_score`, `surface_type`, `obstacles`, `beauty_note` (Matteo's addition — a brief reflection on the aesthetic quality of the place), `discernment` (serves walkers / hinders walkers / ambiguous), `date_walked`, `ttl`.

Retrieval: geographic. "What do we know about the 500-meter radius around this point?" is the primary query pattern. Implemented as a simple distance filter — no R-tree needed given the modest data volume (hundreds of waypoints per city, not millions).

Relevance model: waypoints do not decay. A sidewalk observed in 2023 is still relevant in 2026 unless infrastructure has changed. Instead of decay, the Friars use a "last walked" date. Entries whose segment has not been physically walked in 18 months are flagged for re-survey.

Compaction: route-level summaries survive. Individual waypoint observations are re-retrievable from the full memory branch but excluded from the compacted context.

Identity: stored at `refs/friars/identity/<name>`. Role, city assignment, walking log (total km surveyed), authorization scope, and OpenWallet key.

## 6. Signed Commits via OpenWallet (RFP 3.6)

Every commit signed. David's signing flow adds a privacy check and a dedication.

Flow: INDEX.patch + COMMIT.msg -> Lucia reviews for geographic accuracy -> James reviews for community impact -> Anna checks budget -> David performs privacy scan (no PII in diff) -> David signs with Ed25519 via OpenWallet -> dedication trailer added (`Dedicated-To: St. Christopher, patron of travelers`) -> commit finalized.

Authorization:

```toml
[agents.lucia]
branches = ["friars/*/draft"]
max_patch_lines = 400

[agents.james]
branches = ["friars/*/coord"]
max_patch_lines = 150

[agents.matteo]
branches = ["friars/*"]
max_patch_lines = 100
```

Key rotation: 60-day cycle (aligned with the Franciscan chapter calendar). Compromise revocation: immediate, with notification to all city communities.

## 7. Token Budget

| Component | Input | Output | Frequency | Notes |
|-----------|-------|--------|-----------|-------|
| System prompt | 2,800 | 0 | Once/session | Identity, tools, route context |
| Task ingestion | 1,800 | 300 | Once/task | Route segment, geographic bounds |
| Waypoint memory | 1,500 | 400 | 2/task | Geographic retrieval |
| Discernment/planning | 1,000 | 500 | Once/task | Constraint check, budget estimate |
| Tool calls (per call) | 900 | 400 | ~4/task | Parameter + result |
| Patch generation | 3,000 | 4,500 | Once/task | Geospatial diff with waypoint annotations |
| Commit message | 600 | 300 | Once/task | Route refs, walkability scores |
| Community review | 1,000 | 400 | 1/task | Impact assessment |
| Privacy check | 400 | 100 | 1/task | PII scan |
| **TOTAL (typical task)** | **16,600** | **9,400** | -- | 200-line, 3 files, 1 route segment |

## Unique Insight

From twelve years of urban pilgrimage, we have learned that the most important data about a place cannot be captured by remote sensing. A satellite can measure sidewalk width. It cannot measure whether a street feels safe to walk at dusk. A GIS model can calculate the shortest path. It cannot know that the path passes through a neighborhood where people sit on porches and wave at strangers, and that this changes the experience of walking entirely. Our agents produce route proposals, but the proposals are hypotheses. The data that validates them is collected at three miles per hour, on foot, by a person who is paying attention. No agent memory system replaces this. The best it can do is record what the walker noticed, so the next walker knows where to look.

---

*"Solvitur ambulando."*
